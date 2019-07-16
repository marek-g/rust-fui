// grid.rs is based on WPF's Grid.cs file which is on MIT licence:
// https://github.com/dotnet/wpf/blob/master/src/Microsoft.DotNet.Wpf/src/PresentationFramework/System/Windows/Controls/Grid.cs
//
// The MIT License (MIT)
//
// Copyright (c) .NET Foundation and Contributors
//
// All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::cell::RefCell;
use std::collections::HashMap;
use std::f32;
use std::rc::Rc;

use common::*;
use control::*;
use control_object::*;
use drawing::primitive::Primitive;
use drawing::units::{UserPixelPoint, UserPixelRect, UserPixelSize, UserPixelThickness};
use drawing_context::DrawingContext;
use events::*;
use style::*;
use typed_builder::TypedBuilder;
use view::*;

//
// GridLength.
//

pub enum GridLength {
    // The value indicates that content should be calculated without constraints.
    Auto,

    // The value is expressed in size units.
    Size(f32),

    // The value is expressed as a weighted proportion of available space.
    Star(f32),
}

//
// DefinitionBase.
//

struct DefinitionBase {
    pub user_size: GridLength,
    pub size_type: GridLength,
    pub user_min_size: f32,
    pub user_max_size: f32,

    //  used during measure to accumulate size for "Auto" and "Star" DefinitionBase's
    pub min_size: f32,

    //  size, calculated to be the input contstraint size for child measure
    pub measure_size: f32,

    // cache used for various purposes (sorting, caching, etc) during calculations
    pub size_cache: f32,

    // offset of the DefinitionBase from left / top corner (assuming LTR case)
    pub final_offset: f32,
}

impl DefinitionBase {
    pub fn new(
        user_size: GridLength,
        user_min_size: f32,
        user_max_size: f32,
        treat_star_as_auto: bool,
    ) -> DefinitionBase {
        let mut user_min_size = user_min_size;
        let user_size_value;
        let size_type = match user_size {
            GridLength::Size(v) => {
                user_size_value = v;
                user_min_size = user_min_size.max(user_size_value.min(user_max_size));
                GridLength::Size(v)
            }
            GridLength::Auto => {
                user_size_value = f32::INFINITY;
                GridLength::Auto
            }
            GridLength::Star(v) => {
                user_size_value = f32::INFINITY;
                if treat_star_as_auto {
                    GridLength::Auto
                } else {
                    GridLength::Star(v)
                }
            }
        };

        DefinitionBase {
            user_size: user_size,
            size_type: size_type,
            user_min_size: user_min_size,
            user_max_size: user_max_size,
            min_size: user_min_size,
            measure_size: user_min_size.max(user_size_value.min(user_max_size)),
            size_cache: 0.0f32,
            final_offset: 0.0f32,
        }
    }

    pub fn update_min_size(&mut self, min_size: f32) {
        self.min_size = self.min_size.max(min_size)
    }

    pub fn get_preferred_size(&self) -> f32 {
        if let GridLength::Auto = self.user_size {
            self.min_size
        } else {
            self.min_size.max(self.measure_size)
        }
    }

    pub fn get_min_size_for_arrange(&self) -> f32 {
        // TODO: Add support for SharedSizeGroup attribute
        self.min_size
    }

    pub fn is_shared(&self) -> bool {
        false
    }
}

//
// CellCache.
//

struct CellCache {
    pub child_index: usize,
    pub column_index: usize,
    pub row_index: usize,
    pub column_span: usize,
    pub row_span: usize,
    pub is_star_u: bool,
    pub is_auto_u: bool,
    pub is_star_v: bool,
    pub is_auto_v: bool,
}

//
// Grid.
//

#[derive(TypedBuilder)]
pub struct Grid {
    #[builder(default = 1)]
    pub rows: i32,

    #[builder(default = 1)]
    pub columns: i32,
}

impl View for Grid {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>> {
        Control::new(self, GridDefaultStyle::new(), context)
    }
}

//
// GridDefaultStyle.
//

pub struct GridDefaultStyle {
    rect: Rect,

    definitions_u: Vec<DefinitionBase>,
    definitions_v: Vec<DefinitionBase>,
    cell_group_1: Vec<CellCache>,
    cell_group_2: Vec<CellCache>,
    cell_group_3: Vec<CellCache>,
    cell_group_4: Vec<CellCache>,
    has_star_cells_u: bool,
    has_star_cells_v: bool,
    has_group_3_cells_in_auto_rows: bool,
}

impl GridDefaultStyle {
    pub fn new() -> Self {
        GridDefaultStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            definitions_u: Vec::new(),
            definitions_v: Vec::new(),
            cell_group_1: Vec::new(),
            cell_group_2: Vec::new(),
            cell_group_3: Vec::new(),
            cell_group_4: Vec::new(),
            has_star_cells_u: false,
            has_star_cells_v: false,
            has_group_3_cells_in_auto_rows: false,
        }
    }

    fn prepare_definitions(
        &mut self,
        data: &Grid,
        size_to_content_u: bool,
        size_to_content_v: bool,
    ) {
        self.definitions_u = Vec::new();
        for _ in 0..data.columns {
            let definition = DefinitionBase::new(
                GridLength::Star(1.0f32),
                0.0f32,
                f32::INFINITY,
                size_to_content_u,
            );
            self.definitions_u.push(definition);
        }

        self.definitions_v = Vec::new();
        for _ in 0..data.rows {
            let definition = DefinitionBase::new(
                GridLength::Star(1.0f32),
                0.0f32,
                f32::INFINITY,
                size_to_content_v,
            );
            self.definitions_v.push(definition);
        }
    }

    fn prepare_cell_cache(&mut self, _data: &Grid, children: &Vec<Rc<RefCell<ControlObject>>>) {
        self.has_star_cells_u = false;
        self.has_star_cells_v = false;
        self.has_group_3_cells_in_auto_rows = false;

        let mut child_index = 0;
        let mut column_index = 0;
        let mut row_index = 0;

        self.cell_group_1 = Vec::new();
        self.cell_group_2 = Vec::new();
        self.cell_group_3 = Vec::new();
        self.cell_group_4 = Vec::new();

        for _child in children {
            let column_span = 1;
            let row_span = 1;

            let mut is_star_u = false;
            let mut is_auto_u = false;
            let mut is_star_v = false;
            let mut is_auto_v = false;

            for i in column_index..column_index + column_span {
                match self.definitions_u[i].user_size {
                    GridLength::Star(_) => is_star_u = true,
                    GridLength::Auto => is_auto_u = true,
                    _ => (),
                }
            }

            for i in row_index..row_index + row_span {
                match self.definitions_v[i].user_size {
                    GridLength::Star(_) => is_star_v = true,
                    GridLength::Auto => is_auto_v = true,
                    _ => (),
                }
            }

            self.has_star_cells_u |= is_star_u;
            self.has_star_cells_v |= is_star_v;

            let cell_cache = CellCache {
                child_index: child_index,
                column_index: column_index,
                row_index: row_index,
                column_span: column_span,
                row_span: row_span,
                is_star_u: is_star_u,
                is_auto_u: is_auto_u,
                is_star_v: is_star_v,
                is_auto_v: is_auto_v,
            };

            if is_star_v {
                if is_star_u {
                    self.cell_group_1.push(cell_cache);
                } else {
                    self.cell_group_3.push(cell_cache);
                    self.has_group_3_cells_in_auto_rows |= is_auto_v;
                }
            } else {
                if is_auto_u && !is_star_u {
                    self.cell_group_2.push(cell_cache);
                } else {
                    self.cell_group_4.push(cell_cache);
                }
            }

            child_index += 1;
            column_index += 1;
            if column_index == 2 {
                row_index += 1;
                column_index = 0;
            }
        }
    }

    fn measure_cells_group(
        drawing_context: &mut DrawingContext,
        definitions_u: &mut Vec<DefinitionBase>,
        definitions_v: &mut Vec<DefinitionBase>,
        cells: &Vec<CellCache>,
        children: &Vec<Rc<RefCell<ControlObject>>>,
        ignore_desired_size_u: bool,
        force_infinity_v: bool,
    ) -> bool {
        let mut has_desired_size_u_changed = false;

        let mut span_store = HashMap::new();
        let ignore_desired_size_v = force_infinity_v;

        for cell in cells {
            let child = &children[cell.child_index];

            let old_width = child.borrow().get_rect().width;
            Self::measure_cell(
                drawing_context,
                &cell,
                &child,
                definitions_u,
                definitions_v,
                force_infinity_v,
            );
            let new_rect = child.borrow().get_rect();
            let new_width = new_rect.width;
            let new_height = new_rect.height;
            has_desired_size_u_changed |=
                new_width != old_width && (new_width - old_width).abs() > 0.0000015f32;

            if !ignore_desired_size_u {
                if cell.column_span == 1 {
                    let user_max_size = definitions_u[cell.column_index].user_max_size;
                    definitions_u[cell.column_index].update_min_size(new_width.min(user_max_size));
                } else {
                    let key = (cell.column_index, cell.column_span, true);
                    span_store
                        .entry(key)
                        .and_modify(|v| {
                            if new_width > *v {
                                *v = new_width
                            }
                        })
                        .or_insert(new_width);
                }
            }

            if !ignore_desired_size_v {
                if cell.row_span == 1 {
                    let user_max_size = definitions_v[cell.row_index].user_max_size;
                    definitions_v[cell.row_index].update_min_size(new_height.min(user_max_size));
                } else {
                    let key = (cell.row_index, cell.row_span, false);
                    span_store
                        .entry(key)
                        .and_modify(|v| {
                            if new_height > *v {
                                *v = new_height
                            }
                        })
                        .or_insert(new_height);
                }
            }
        }

        for (key, requested_size) in span_store.iter() {
            Self::ensure_min_size_in_definition_range(
                if key.2 { definitions_u } else { definitions_v },
                key.0,
                key.1,
                *requested_size,
            );
        }

        has_desired_size_u_changed
    }

    fn measure_cell(
        drawing_context: &mut DrawingContext,
        cell: &CellCache,
        child: &Rc<RefCell<ControlObject>>,
        definitions_u: &mut Vec<DefinitionBase>,
        definitions_v: &mut Vec<DefinitionBase>,
        force_infinity_v: bool,
    ) {
        let cell_measure_width;
        let cell_measure_height;

        if cell.is_auto_u && !cell.is_star_u {
            cell_measure_width = f32::INFINITY;
        } else {
            cell_measure_width = Self::get_measure_size_for_range(
                definitions_u,
                cell.column_index,
                cell.column_span,
            );
        }

        if force_infinity_v {
            cell_measure_height = f32::INFINITY;
        } else if cell.is_auto_v && !cell.is_star_v {
            cell_measure_height = f32::INFINITY;
        } else {
            cell_measure_height =
                Self::get_measure_size_for_range(definitions_v, cell.row_index, cell.row_span);
        }

        child.borrow_mut().measure(
            drawing_context,
            Size::new(cell_measure_width, cell_measure_height),
        );
    }

    fn get_measure_size_for_range(
        definitions: &mut Vec<DefinitionBase>,
        start: usize,
        count: usize,
    ) -> f32 {
        let mut measure_size = 0.0f32;
        for i in start..start + count {
            measure_size += if let GridLength::Auto = definitions[i].size_type {
                definitions[i].min_size
            } else {
                definitions[i].measure_size
            };
        }
        measure_size
    }

    ///
    /// Distributes min size back to definition array's range.
    ///
    fn ensure_min_size_in_definition_range(
        definitions: &mut Vec<DefinitionBase>,
        start: usize,
        count: usize,
        requested_size: f32,
    ) {
        if requested_size.abs() < 0.00001 {
            return;
        }

        let mut temp_definitions = Vec::with_capacity(count);
        let end = start + count;
        let mut auto_definitions_count = 0;
        let mut range_min_size = 0.0f32;
        let mut range_preferred_size = 0.0f32;
        let mut range_max_size = 0.0f32;
        let mut max_max_size = 0.0f32;

        for i in start..end {
            let min_size = definitions[i].min_size;
            let preferred_size = definitions[i].get_preferred_size();
            let max_size = definitions[i].user_max_size.max(min_size);

            range_min_size += min_size;
            range_preferred_size += preferred_size;
            range_max_size += max_size;

            definitions[i].size_cache = max_size;

            if max_max_size < max_max_size {
                max_max_size = max_size;
            }
            if let GridLength::Auto = definitions[i].user_size {
                auto_definitions_count += 1;
            }

            temp_definitions.push(i);
        }

        if requested_size <= range_min_size {
            return;
        }

        if requested_size <= range_preferred_size {
            temp_definitions.sort_by(|x, y| {
                if let GridLength::Auto = definitions[*x].user_size {
                    if let GridLength::Auto = definitions[*y].user_size {
                        definitions[*x]
                            .min_size
                            .partial_cmp(&definitions[*y].min_size)
                            .unwrap()
                    } else {
                        std::cmp::Ordering::Less
                    }
                } else {
                    if let GridLength::Auto = definitions[*y].user_size {
                        std::cmp::Ordering::Greater
                    } else {
                        definitions[*x]
                            .get_preferred_size()
                            .partial_cmp(&definitions[*y].get_preferred_size())
                            .unwrap()
                    }
                }
            });

            let mut size_to_distribute = requested_size;
            for i in 0..auto_definitions_count {
                size_to_distribute -= definitions[temp_definitions[i]].min_size;
            }

            for i in auto_definitions_count..count {
                let new_min_size = definitions[temp_definitions[i]]
                    .get_preferred_size()
                    .min(size_to_distribute / ((count - i) as f32));
                if new_min_size > definitions[temp_definitions[i]].min_size {
                    definitions[temp_definitions[i]].update_min_size(new_min_size);
                }
                size_to_distribute -= new_min_size;
            }
        } else if requested_size <= range_max_size {
            temp_definitions.sort_by(|x, y| {
                if let GridLength::Auto = definitions[*x].user_size {
                    if let GridLength::Auto = definitions[*y].user_size {
                        definitions[*x]
                            .size_cache
                            .partial_cmp(&definitions[*y].size_cache)
                            .unwrap()
                    } else {
                        std::cmp::Ordering::Greater
                    }
                } else {
                    if let GridLength::Auto = definitions[*y].user_size {
                        std::cmp::Ordering::Less
                    } else {
                        definitions[*x]
                            .size_cache
                            .partial_cmp(&definitions[*y].size_cache)
                            .unwrap()
                    }
                }
            });

            let mut size_to_distribute = requested_size - range_preferred_size;
            for i in 0..count - auto_definitions_count {
                let preferred_size = definitions[temp_definitions[i]].get_preferred_size();
                let new_min_size = preferred_size
                    + size_to_distribute / ((count - auto_definitions_count - i) as f32);
                let size_cache = definitions[temp_definitions[i]].size_cache;
                definitions[temp_definitions[i]].update_min_size(new_min_size.min(size_cache));
                size_to_distribute -= definitions[temp_definitions[i]].min_size - preferred_size;
            }

            for i in count - auto_definitions_count..count {
                let preferred_size = definitions[temp_definitions[i]].min_size;
                let new_min_size = preferred_size + size_to_distribute / ((count - i) as f32);
                let size_cache = definitions[temp_definitions[i]].size_cache;
                definitions[temp_definitions[i]].update_min_size(new_min_size.min(size_cache));
                size_to_distribute -= definitions[temp_definitions[i]].min_size - preferred_size;
            }
        } else {
            let equal_size = requested_size / (count as f32);
            if equal_size < max_max_size && max_max_size - equal_size > 0.0000015f32 {
                let total_remaining_size = max_max_size * (count as f32) - range_max_size;
                let size_to_distribute = requested_size - range_max_size;

                for i in 0..count {
                    let delta_size = (max_max_size - definitions[temp_definitions[i]].size_cache)
                        * size_to_distribute
                        / total_remaining_size;
                    let size_cache = definitions[temp_definitions[i]].size_cache;
                    definitions[temp_definitions[i]].update_min_size(size_cache + delta_size);
                }
            } else {
                for i in 0..count {
                    definitions[temp_definitions[i]].update_min_size(equal_size);
                }
            }
        }
    }

    fn resolve_star(definitions: &mut Vec<DefinitionBase>, available_size: f32) {
        let def_count = definitions.len();
        let mut definition_indices_min = Vec::<i32>::with_capacity(def_count);
        let mut definition_indices_max = Vec::<i32>::with_capacity(def_count);
        let mut taken_size = 0.0f32;
        let mut star_count = 0;
        let mut scale = 1.0f32;

        // Phase 1.  Determine the maximum *-weight and prepare to adjust *-weights
        let mut max_star = 0.0f32;
        for def in definitions.iter_mut() {
            if let GridLength::Star(v) = def.size_type {
                star_count += 1;
                def.measure_size = 1.0f32;
                max_star = max_star.max(v);
            }
        }

        if max_star.is_infinite() && max_star.is_sign_positive() {
            scale = -1.0f32;
        } else if star_count > 0 {
            let power = (f32::MAX / max_star / (star_count as f32)).log2().floor();
            if power < 0.0 {
                scale = 2.0f32.powf(power - 4.0f32);
            }
        }

        // Phase 2 & 3
        let mut run_phase_2_and_3 = true;
        while run_phase_2_and_3 {
            // Phase 2.   Compute total *-weight W and available space S.
            let mut total_star_weight = 0.0f32;
            taken_size = 0.0f32;
            let mut min_count = 0;
            let mut max_count = 0;
            definition_indices_min.truncate(0);
            definition_indices_max.truncate(0);

            for (i, def) in definitions.iter_mut().enumerate() {
                match def.size_type {
                    GridLength::Auto => {
                        taken_size += def.min_size;
                    }
                    GridLength::Size(_) => {
                        taken_size += def.measure_size;
                    }
                    GridLength::Star(v) => {
                        if def.measure_size < 0.0f32 {
                            taken_size += -def.measure_size;
                        } else {
                            let star_weight = Self::get_star_weight(v, scale);
                            total_star_weight += star_weight;

                            if def.min_size > 0.0f32 {
                                definition_indices_min.push(i as i32);
                                min_count += 1;
                                def.measure_size = star_weight / def.min_size;
                            }

                            let effective_max_size = def.min_size.max(def.user_max_size);
                            if !effective_max_size.is_infinite()
                                || !effective_max_size.is_sign_positive()
                            {
                                definition_indices_max.push(i as i32);
                                max_count += 1;
                                def.size_cache = star_weight / effective_max_size;
                            }
                        }
                    }
                };
            }

            // Phase 3.  Resolve *-items whose proportional sizes are too big or too small.
            let min_count_phase2 = min_count;
            let max_count_phase2 = max_count;
            let mut taken_star_weight = 0.0f32;
            let mut remaining_available_size = available_size - taken_size;
            let mut remaining_star_weight = total_star_weight - taken_star_weight;

            definition_indices_min.sort_by(|x, y| {
                definitions[*y as usize]
                    .measure_size
                    .partial_cmp(&definitions[*x as usize].measure_size)
                    .unwrap()
            });
            definition_indices_max.sort_by(|x, y| {
                definitions[*x as usize]
                    .size_cache
                    .partial_cmp(&definitions[*y as usize].size_cache)
                    .unwrap()
            });

            while min_count + max_count > 0 && remaining_available_size > 0.0f32 {
                if remaining_star_weight < total_star_weight / 256.0f32 {
                    taken_star_weight = 0.0f32;
                    total_star_weight = 0.0f32;

                    for def in definitions.iter_mut() {
                        if let GridLength::Star(v) = def.size_type {
                            if def.measure_size > 0.0f32 {
                                total_star_weight += Self::get_star_weight(v, scale);
                            }
                        }
                    }

                    remaining_star_weight = total_star_weight - taken_star_weight;
                }

                let min_ratio = if min_count > 0 {
                    definitions[definition_indices_min[min_count - 1] as usize].measure_size
                } else {
                    f32::INFINITY
                };
                let max_ratio = if max_count > 0 {
                    definitions[definition_indices_max[max_count - 1] as usize].size_cache
                } else {
                    -1.0f32
                };
                let proportion = remaining_star_weight / remaining_available_size;

                let choose_min = if min_ratio < proportion {
                    if max_ratio > proportion {
                        let min_power = min_ratio.log2().floor();
                        let max_power = max_ratio.log2().floor();
                        let f = 2.0f32.powf(((min_power + max_power) / 2.0f32).floor());
                        if (proportion / f) * (proportion / f) > (min_ratio / f) * (min_ratio / f) {
                            true
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                } else if max_ratio > proportion {
                    false
                } else {
                    break;
                };

                let (mut resolved_def, resolved_size) = if choose_min {
                    let index = definition_indices_min[min_count - 1] as usize;
                    min_count -= 1;
                    let def = &mut definitions[index];
                    let size = def.min_size;
                    (def, size)
                } else {
                    let index = definition_indices_max[max_count - 1] as usize;
                    max_count -= 1;
                    let def = &mut definitions[index];
                    let size = def.min_size.max(def.user_max_size);
                    (def, size)
                };

                taken_size += resolved_size;
                resolved_def.measure_size = -resolved_size;
                if let GridLength::Star(v) = resolved_def.user_size {
                    taken_star_weight += Self::get_star_weight(v, scale);
                    star_count -= 1;
                }

                remaining_available_size = available_size - taken_size;
                remaining_star_weight = total_star_weight - taken_star_weight;

                while min_count > 0
                    && definitions[definition_indices_min[min_count - 1] as usize].measure_size
                        < 0.0
                {
                    min_count -= 1;
                    definition_indices_min[min_count] = -1;
                }
                while max_count > 0
                    && definitions[definition_indices_max[max_count - 1] as usize].measure_size
                        < 0.0
                {
                    max_count -= 1;
                    definition_indices_max[max_count] = -1;
                }
            }

            run_phase_2_and_3 = false;
            if star_count == 0 && taken_size < available_size {
                for i in min_count..min_count_phase2 {
                    let index = definition_indices_min[i];
                    if index >= 0 {
                        definitions[index as usize].measure_size = 1.0f32;
                        star_count += 1;
                        run_phase_2_and_3 = true;
                    }
                }
            }

            if taken_size > available_size {
                for i in max_count..max_count_phase2 {
                    let index = definition_indices_max[i];
                    if index >= 0 {
                        definitions[index as usize].measure_size = 1.0f32;
                        star_count += 1;
                        run_phase_2_and_3 = true;
                    }
                }
            }
        }

        // Phase 4.  Resolve the remaining defs proportionally.
        star_count = 0;
        definition_indices_min.truncate(0);
        for i in 0..def_count {
            let def = &mut definitions[i];

            if let GridLength::Star(v) = def.size_type {
                if def.measure_size < 0.0f32 {
                    def.measure_size = -def.measure_size;
                } else {
                    definition_indices_min.push(i as i32);
                    star_count += 1;
                    def.measure_size = Self::get_star_weight(v, scale);
                }
            }
        }

        if star_count > 0 {
            definition_indices_min.sort_by(|x, y| {
                definitions[*x as usize]
                    .measure_size
                    .partial_cmp(&definitions[*y as usize].measure_size)
                    .unwrap()
            });

            let mut total_star_weight = 0.0f32;
            for i in 0..star_count {
                let def = &mut definitions[definition_indices_min[i] as usize];
                total_star_weight += def.measure_size;
                def.size_cache = total_star_weight;
            }

            for i in (0..star_count).rev() {
                let def = &mut definitions[definition_indices_min[i] as usize];
                let mut resolved_size = if def.measure_size > 0.0f32 {
                    (available_size - taken_size).max(0.0f32) * (def.measure_size / def.size_cache)
                } else {
                    0.0f32
                };

                resolved_size = resolved_size.min(def.user_max_size);
                resolved_size = resolved_size.max(def.min_size);

                def.measure_size = resolved_size;
                taken_size += resolved_size;
            }
        }
    }

    fn set_final_size(definitions: &mut Vec<DefinitionBase>, final_size: f32, columns: bool) {
        let def_count = definitions.len();
        let mut definition_indices_min = Vec::<i32>::with_capacity(def_count);
        let mut definition_indices_max = Vec::<i32>::with_capacity(def_count);
        let mut taken_size = 0.0f32;
        let mut star_count = 0;
        let mut scale = 1.0f32;

        // Phase 1.  Determine the maximum *-weight and prepare to adjust *-weights
        let mut max_star = 0.0f32;
        for def in definitions.iter_mut() {
            if let GridLength::Star(v) = def.user_size {
                star_count += 1;
                def.measure_size = 1.0f32;
                max_star = max_star.max(v);
            }
        }

        if max_star.is_infinite() && max_star.is_sign_positive() {
            scale = -1.0f32;
        } else if star_count > 0 {
            let power = (f32::MAX / (max_star as f32) / (star_count as f32))
                .log(2.0f32)
                .floor();
            if power < 0.0f32 {
                scale = 2.0f32.powf(power - 4.0f32);
            }
        }

        // Phase 2 & 3
        let mut run_phase_2_and_3 = true;
        while run_phase_2_and_3 {
            // Phase 2.   Compute total *-weight W and available space S.
            let mut total_star_weight = 0.0f32;
            taken_size = 0.0f32;
            let mut min_count = 0;
            let mut max_count = 0;
            definition_indices_min.truncate(0);
            definition_indices_max.truncate(0);

            for (i, def) in definitions.iter_mut().enumerate() {
                if let GridLength::Star(v) = def.user_size {
                    if def.measure_size < 0.0f32 {
                        taken_size += -def.measure_size;
                    } else {
                        let star_weight = Self::get_star_weight(v, scale);
                        total_star_weight += star_weight;

                        let min_size_for_arrange = def.get_min_size_for_arrange();
                        if min_size_for_arrange > 0.0f32 {
                            definition_indices_min.push(i as i32);
                            min_count += 1;
                            def.measure_size = star_weight / min_size_for_arrange;
                        }

                        let effective_max_size = min_size_for_arrange.max(def.user_max_size);
                        if !effective_max_size.is_infinite()
                            || !effective_max_size.is_sign_positive()
                        {
                            definition_indices_max.push(i as i32);
                            max_count += 1;
                            def.size_cache = star_weight / effective_max_size;
                        }
                    }
                } else {
                    let min_size_for_arrange = def.get_min_size_for_arrange();

                    let user_size = match def.user_size {
                        GridLength::Size(v) => v,
                        GridLength::Auto => min_size_for_arrange,
                        _ => 0.0f32,
                    };

                    let user_max_size = if def.is_shared() {
                        user_size
                    } else {
                        def.user_max_size
                    };

                    def.size_cache = min_size_for_arrange.max(user_size.min(user_max_size));
                    taken_size += def.size_cache;
                }
            }

            // Phase 3.  Resolve *-items whose proportional sizes are too big or too small.
            let min_count_phase2 = min_count;
            let max_count_phase2 = max_count;
            let mut taken_star_weight = 0.0f32;
            let mut remaining_available_size = final_size - taken_size;
            let mut remaining_star_weight = total_star_weight - taken_star_weight;

            definition_indices_min.sort_by(|x, y| {
                definitions[*y as usize]
                    .measure_size
                    .partial_cmp(&definitions[*x as usize].measure_size)
                    .unwrap()
            });
            definition_indices_max.sort_by(|x, y| {
                definitions[*x as usize]
                    .size_cache
                    .partial_cmp(&definitions[*y as usize].size_cache)
                    .unwrap()
            });

            while min_count + max_count > 0 && remaining_available_size > 0.0f32 {
                if remaining_star_weight < total_star_weight / 256.0f32 {
                    taken_star_weight = 0.0f32;
                    total_star_weight = 0.0f32;

                    for def in definitions.iter_mut() {
                        if let GridLength::Star(v) = def.user_size {
                            if def.measure_size > 0.0f32 {
                                total_star_weight += Self::get_star_weight(v, scale);
                            }
                        }
                    }

                    remaining_star_weight = total_star_weight - taken_star_weight;
                }

                let min_ratio = if min_count > 0 {
                    definitions[definition_indices_min[min_count - 1] as usize].measure_size
                } else {
                    f32::INFINITY
                };
                let max_ratio = if max_count > 0 {
                    definitions[definition_indices_max[max_count - 1] as usize].size_cache
                } else {
                    -1.0f32
                };
                let proportion = remaining_star_weight / remaining_available_size;

                let choose_min = if min_ratio < proportion {
                    if max_ratio > proportion {
                        let min_power = min_ratio.log2().floor();
                        let max_power = max_ratio.log2().floor();
                        let f = 2.0f32.powf(((min_power + max_power) / 2.0f32).floor());
                        if (proportion / f) * (proportion / f) > (min_ratio / f) * (min_ratio / f) {
                            true
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                } else if max_ratio > proportion {
                    false
                } else {
                    break;
                };

                let (mut resolved_def, resolved_size) = if choose_min {
                    let index = definition_indices_min[min_count - 1] as usize;
                    min_count -= 1;
                    let def = &mut definitions[index];
                    let size = def.get_min_size_for_arrange();
                    (def, size)
                } else {
                    let index = definition_indices_max[max_count - 1] as usize;
                    max_count -= 1;
                    let def = &mut definitions[index];
                    let size = def.get_min_size_for_arrange().max(def.user_max_size);
                    (def, size)
                };

                taken_size += resolved_size;
                resolved_def.measure_size = -resolved_size;
                if let GridLength::Star(v) = resolved_def.user_size {
                    taken_star_weight += Self::get_star_weight(v, scale);
                    star_count -= 1;
                }

                remaining_available_size = final_size - taken_size;
                remaining_star_weight = total_star_weight - taken_star_weight;

                while min_count > 0
                    && definitions[definition_indices_min[min_count - 1] as usize].measure_size
                        < 0.0
                {
                    min_count -= 1;
                    definition_indices_min[min_count] = -1;
                }
                while max_count > 0
                    && definitions[definition_indices_max[max_count - 1] as usize].measure_size
                        < 0.0
                {
                    max_count -= 1;
                    definition_indices_max[max_count] = -1;
                }
            }

            run_phase_2_and_3 = false;
            if star_count == 0 && taken_size < final_size {
                for i in min_count..min_count_phase2 {
                    let index = definition_indices_min[i];
                    if index >= 0 {
                        definitions[index as usize].measure_size = 1.0f32;
                        star_count += 1;
                        run_phase_2_and_3 = true;
                    }
                }
            }

            if taken_size > final_size {
                for i in max_count..max_count_phase2 {
                    let index = definition_indices_max[i];
                    if index >= 0 {
                        definitions[index as usize].measure_size = 1.0f32;
                        star_count += 1;
                        run_phase_2_and_3 = true;
                    }
                }
            }
        }

        // Phase 4.  Resolve the remaining defs proportionally.
        star_count = 0;
        definition_indices_min.truncate(0);
        for i in 0..def_count {
            let def = &mut definitions[i];

            if let GridLength::Star(v) = def.user_size {
                if def.measure_size < 0.0f32 {
                    def.size_cache = -def.measure_size;
                } else {
                    definition_indices_min.push(i as i32);
                    star_count += 1;
                    def.measure_size = Self::get_star_weight(v, scale);
                }
            }
        }

        if star_count > 0 {
            definition_indices_min.sort_by(|x, y| {
                definitions[*x as usize]
                    .measure_size
                    .partial_cmp(&definitions[*y as usize].measure_size)
                    .unwrap()
            });

            let mut total_star_weight = 0.0f32;
            for i in 0..star_count {
                let def = &mut definitions[definition_indices_min[i] as usize];
                total_star_weight += def.measure_size;
                def.size_cache = total_star_weight;
            }

            for i in (0..star_count).rev() {
                let def = &mut definitions[definition_indices_min[i] as usize];
                let mut resolved_size = if def.measure_size > 0.0f32 {
                    (final_size - taken_size).max(0.0f32) * (def.measure_size / def.size_cache)
                } else {
                    0.0f32
                };

                resolved_size = resolved_size.min(def.user_max_size);
                resolved_size = resolved_size.max(def.get_min_size_for_arrange());

                taken_size += resolved_size;
                def.size_cache = resolved_size;
            }
        }

        // Phase 5.  Apply layout rounding.
        let use_layout_rounding = true;
        if use_layout_rounding {
            // TODO: get dpi from system
            let dpi_scale_x = 1.0f32;
            let dpi_scale_y = 1.0f32;
            let dpi_scale = if columns { dpi_scale_x } else { dpi_scale_y };

            let mut rounding_errors = Vec::with_capacity(definitions.len());
            let mut rounded_taken_size = 0.0f32;
            for def in definitions.iter_mut() {
                let rounded_size = crate::high_dpi::round_layout_value(def.size_cache, dpi_scale);
                rounding_errors.push(rounded_size - def.size_cache);
                def.size_cache = rounded_size;
                rounded_taken_size += rounded_size;
            }

            if (rounded_taken_size - final_size).abs() > 0.0000015f32 {
                definition_indices_min.truncate(0);
                for i in 0..definitions.len() {
                    definition_indices_min.push(i as i32);
                }

                definition_indices_min.sort_by(|x, y| {
                    rounding_errors[*x as usize]
                        .partial_cmp(&rounding_errors[*y as usize])
                        .unwrap()
                });

                let mut adjusted_size = rounded_taken_size;
                let dpi_increment = 1.0f32 / dpi_scale;

                if rounded_taken_size > final_size {
                    let mut i = (definitions.len() - 1) as i32;
                    while adjusted_size > final_size
                        && adjusted_size - final_size > 0.0000015f32
                        && i >= 0
                    {
                        let definition =
                            &mut definitions[definition_indices_min[i as usize] as usize];
                        let mut final_size = definition.size_cache - dpi_increment;
                        final_size = final_size.max(definition.get_min_size_for_arrange());
                        if final_size < definition.size_cache {
                            adjusted_size -= dpi_increment;
                        }
                        definition.size_cache = final_size;
                        i -= 1;
                    }
                } else if rounded_taken_size < final_size {
                    let mut i = 0;
                    while adjusted_size < final_size
                        && final_size - adjusted_size > 0.0000015f32
                        && i < definitions.len()
                    {
                        let definition = &mut definitions[definition_indices_min[i] as usize];
                        let mut final_size = definition.size_cache + dpi_increment;
                        final_size = final_size.max(definition.get_min_size_for_arrange());
                        if final_size > definition.size_cache {
                            adjusted_size += dpi_increment;
                        }
                        definition.size_cache = final_size;
                        i += 1;
                    }
                }
            }
        }

        // Phase 6.  Compute final offsets
        definitions[0].final_offset = 0.0f32;
        let length = definitions.len();
        for i in 0..length {
            definitions[(i + 1) % length].final_offset =
                definitions[i].final_offset + definitions[i].size_cache;
        }
    }

    // Returns *-weight, adjusted for scale computed during Phase 1.
    fn get_star_weight(v: f32, scale: f32) -> f32 {
        if scale < 0.0f32 {
            if v.is_infinite() && v.is_sign_positive() {
                1.0f32
            } else {
                0.0f32
            }
        } else {
            v * scale
        }
    }

    fn get_final_size_for_range(
        definitions: &mut Vec<DefinitionBase>,
        start: usize,
        count: usize,
    ) -> f32 {
        definitions
            .iter()
            .skip(start)
            .take(count)
            .map(|def| def.size_cache)
            .sum()
    }

    fn cache_min_sizes(
        definitions: &Vec<DefinitionBase>,
        cells: &Vec<CellCache>,
        is_rows: bool,
    ) -> Vec<f32> {
        let mut min_sizes = Vec::with_capacity(definitions.len());

        for _ in 0..definitions.len() {
            min_sizes.push(-1.0f32);
        }

        for cell in cells {
            if is_rows {
                min_sizes[cell.row_index] = definitions[cell.row_index].min_size;
            } else {
                min_sizes[cell.column_index] = definitions[cell.column_index].min_size;
            }
        }

        min_sizes
    }

    fn apply_cached_min_sizes(definitions: &mut Vec<DefinitionBase>, min_sizes: &Vec<f32>) {
        for (i, min_size) in min_sizes.iter().enumerate() {
            if *min_size > 0.0f32 || min_size.abs() < 0.0000015f32 {
                definitions[i].min_size = *min_size;
            }
        }
    }

    fn calculate_desired_size(definitions: &Vec<DefinitionBase>) -> f32 {
        return definitions.iter().map(|d| d.min_size).sum()
    }
}

impl Style<Grid> for GridDefaultStyle {
    fn setup_dirty_watching(&mut self, _data: &mut Grid, _control: &Rc<RefCell<Control<Grid>>>) {}

    fn handle_event(
        &mut self,
        _data: &mut Grid,
        _children: &Vec<Rc<RefCell<ControlObject>>>,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &Grid,
        children: &Vec<Rc<RefCell<ControlObject>>>,
        drawing_context: &mut DrawingContext,
        size: Size,
    ) {
        let mut grid_desired_size = Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);

        if data.rows == 1 && data.columns == 1 {
            for child in children {
                let mut child = child.borrow_mut();
                child.measure(drawing_context, size);
                let child_rc = child.get_rect();
                grid_desired_size.width = grid_desired_size.width.max(child_rc.width);
                grid_desired_size.height = grid_desired_size.height.max(child_rc.height);
            }
        } else {
            let size_to_content_u = size.width == f32::INFINITY;
            let size_to_content_v = size.height == f32::INFINITY;

            self.prepare_definitions(&data, size_to_content_u, size_to_content_v);
            self.prepare_cell_cache(&data, &children);

            Self::measure_cells_group(
                drawing_context,
                &mut self.definitions_u,
                &mut self.definitions_v,
                &self.cell_group_1,
                &children,
                false,
                false,
            );

            if !self.has_group_3_cells_in_auto_rows {
                if self.has_star_cells_v {
                    Self::resolve_star(&mut self.definitions_v, size.height);
                }
                Self::measure_cells_group(
                    drawing_context,
                    &mut self.definitions_u,
                    &mut self.definitions_v,
                    &self.cell_group_2,
                    &children,
                    false,
                    false,
                );

                if self.has_star_cells_u {
                    Self::resolve_star(&mut self.definitions_u, size.width);
                }
                Self::measure_cells_group(
                    drawing_context,
                    &mut self.definitions_u,
                    &mut self.definitions_v,
                    &self.cell_group_3,
                    &children,
                    false,
                    false,
                );
            } else {
                if self.cell_group_2.len() == 0 {
                    if self.has_star_cells_u {
                        Self::resolve_star(&mut self.definitions_u, size.width);
                    }
                    Self::measure_cells_group(
                        drawing_context,
                        &mut self.definitions_u,
                        &mut self.definitions_v,
                        &self.cell_group_3,
                        &children,
                        false,
                        false,
                    );
                    if self.has_star_cells_v {
                        Self::resolve_star(&mut self.definitions_v, size.height);
                    }
                } else {
                    let mut has_desired_size_u_changed = false;
                    let mut cnt = 0;

                    let group_2_min_sizes =
                        Self::cache_min_sizes(&self.definitions_u, &self.cell_group_3, false);
                    let group_3_min_sizes =
                        Self::cache_min_sizes(&self.definitions_v, &self.cell_group_3, true);

                    Self::measure_cells_group(
                        drawing_context,
                        &mut self.definitions_u,
                        &mut self.definitions_v,
                        &self.cell_group_2,
                        &children,
                        false,
                        true,
                    );

                    loop {
                        if has_desired_size_u_changed {
                            Self::apply_cached_min_sizes(
                                &mut self.definitions_v,
                                &group_3_min_sizes,
                            );
                        }

                        if self.has_star_cells_u {
                            Self::resolve_star(&mut self.definitions_u, size.width);
                        }
                        Self::measure_cells_group(
                            drawing_context,
                            &mut self.definitions_u,
                            &mut self.definitions_v,
                            &self.cell_group_3,
                            &children,
                            false,
                            false,
                        );

                        Self::apply_cached_min_sizes(&mut self.definitions_u, &group_2_min_sizes);

                        if self.has_star_cells_v {
                            Self::resolve_star(&mut self.definitions_v, size.height);
                        }
                        has_desired_size_u_changed = Self::measure_cells_group(
                            drawing_context,
                            &mut self.definitions_u,
                            &mut self.definitions_v,
                            &self.cell_group_2,
                            &children,
                            cnt == 5,
                            false,
                        );

                        cnt += 1;
                        if !has_desired_size_u_changed || cnt > 5 {
                            break;
                        }
                    }
                }
            }

            Self::measure_cells_group(
                drawing_context,
                &mut self.definitions_u,
                &mut self.definitions_v,
                &self.cell_group_4,
                &children,
                false,
                false,
            );

            grid_desired_size.width = Self::calculate_desired_size(&self.definitions_u);
            grid_desired_size.height = Self::calculate_desired_size(&self.definitions_v);
        }

        self.rect = grid_desired_size;
    }

    fn set_rect(&mut self, data: &Grid, children: &Vec<Rc<RefCell<ControlObject>>>, rect: Rect) {
        self.rect = rect;

        if data.rows == 1 && data.columns == 1 {
            for child in children {
                child.borrow_mut().set_rect(rect);
            }
        } else {
            Self::set_final_size(&mut self.definitions_u, rect.width, true);
            Self::set_final_size(&mut self.definitions_v, rect.height, false);

            for cell in self
                .cell_group_1
                .iter()
                .chain(self.cell_group_2.iter())
                .chain(self.cell_group_3.iter())
                .chain(self.cell_group_4.iter())
            {
                let child = &children[cell.child_index];
                let column_index = cell.column_index;
                let row_index = cell.row_index;
                let column_span = cell.column_span;
                let row_span = cell.row_span;
                let rc = Rect::new(
                    if column_index == 0 {
                        0.0f32
                    } else {
                        self.definitions_u[column_index].final_offset
                    },
                    if row_index == 0 {
                        0.0f32
                    } else {
                        self.definitions_v[row_index].final_offset
                    },
                    Self::get_final_size_for_range(
                        &mut self.definitions_u,
                        column_index,
                        column_span,
                    ),
                    Self::get_final_size_for_range(&mut self.definitions_v, row_index, row_span),
                );
                child.borrow_mut().set_rect(rc);
            }
        }
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &Grid,
        children: &Vec<Rc<RefCell<ControlObject>>>,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            for child in children.iter() {
                let c = child.borrow();
                let rect = c.get_rect();
                if point.is_inside(&rect) {
                    let child_hit_test = c.hit_test(point);
                    match child_hit_test {
                        HitTestResult::Current => return HitTestResult::Child(child.clone()),
                        HitTestResult::Child(..) => return child_hit_test,
                        HitTestResult::Nothing => (),
                    }
                }
            }
            HitTestResult::Nothing
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        _data: &Grid,
        children: &Vec<Rc<RefCell<ControlObject>>>,
        drawing_context: &mut DrawingContext,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        for child in children {
            vec.append(&mut child.borrow().to_primitives(drawing_context));
        }

        vec
    }
}
