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

use drawing::primitive::Primitive;
use fui_core::*;
use typed_builder::TypedBuilder;

//
// Length
//

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Length {
    /// Minimum size that fits all the children.
    Auto,

    /// User specified size.
    Exact(f32),

    /// The value is expressed as a weighted proportion of available space.
    Fill(f32),
}

//
// DefinitionBase
//

struct DefinitionBase {
    pub user_size: Length,
    pub size_type: Length,
    pub user_min_size: f32,
    pub user_max_size: f32,

    //  used during measure to accumulate size for "Auto" and "Fill" DefinitionBase's
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
        user_size: Length,
        user_min_size: f32,
        user_max_size: f32,
        treat_fill_as_auto: bool,
    ) -> DefinitionBase {
        let mut user_min_size = user_min_size;
        let user_size_value;
        let size_type = match user_size {
            Length::Exact(v) => {
                user_size_value = v;
                user_min_size = user_min_size.max(user_size_value.min(user_max_size));
                Length::Exact(v)
            }
            Length::Auto => {
                user_size_value = f32::INFINITY;
                Length::Auto
            }
            Length::Fill(v) => {
                user_size_value = f32::INFINITY;
                if treat_fill_as_auto {
                    Length::Auto
                } else {
                    Length::Fill(v)
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
        // may require change when SharedSizeGroup attribute is implemented
        if let Length::Auto = self.user_size {
            self.min_size
        } else {
            self.min_size.max(self.measure_size)
        }
    }

    pub fn get_min_size_for_arrange(&self) -> f32 {
        // may require change when SharedSizeGroup attribute is implemented
        self.min_size
    }

    pub fn is_shared(&self) -> bool {
        // may require change when SharedSizeGroup attribute is implemented
        false
    }
}

//
// CellCache
//

struct CellCache {
    pub child_index: usize,
    pub column_index: usize,
    pub row_index: usize,
    pub column_span: usize,
    pub row_span: usize,
    pub is_fill_u: bool,
    pub is_auto_u: bool,
    pub is_fill_v: bool,
    pub is_auto_v: bool,
}

//
// Attached values
//

pub struct Row;
impl typemap::Key for Row {
    type Value = i32;
}

pub struct RowSpan;
impl typemap::Key for RowSpan {
    type Value = i32;
}

pub struct Column;
impl typemap::Key for Column {
    type Value = i32;
}

pub struct ColumnSpan;
impl typemap::Key for ColumnSpan {
    type Value = i32;
}

//
// Grid
//

#[derive(TypedBuilder)]
pub struct Grid {
    #[builder(default = 0)]
    pub rows: i32,

    #[builder(default = 0)]
    pub columns: i32,

    #[builder(default = Length::Fill(1.0f32))]
    pub default_width: Length,

    #[builder(default = Length::Fill(1.0f32))]
    pub default_height: Length,

    #[builder(default = Vec::new())]
    pub widths: Vec<(i32, Length)>,

    #[builder(default = Vec::new())]
    pub heights: Vec<(i32, Length)>,

    #[builder(default = 0.0f32)]
    pub default_min_width: f32,

    #[builder(default = 0.0f32)]
    pub default_min_height: f32,

    #[builder(default = f32::INFINITY)]
    pub default_max_width: f32,

    #[builder(default = f32::INFINITY)]
    pub default_max_height: f32,

    #[builder(default = Vec::new())]
    pub min_widths: Vec<(i32, f32)>,

    #[builder(default = Vec::new())]
    pub min_heights: Vec<(i32, f32)>,

    #[builder(default = Vec::new())]
    pub max_widths: Vec<(i32, f32)>,

    #[builder(default = Vec::new())]
    pub max_heights: Vec<(i32, f32)>,
}

impl Grid {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultGridStyle::new(
                    DefaultGridStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default Grid Style
//

#[derive(TypedBuilder)]
pub struct DefaultGridStyleParams {}

pub struct DefaultGridStyle {
    rect: Rect,

    definitions_u: Vec<DefinitionBase>,
    definitions_v: Vec<DefinitionBase>,
    cell_group_1: Vec<CellCache>,
    cell_group_2: Vec<CellCache>,
    cell_group_3: Vec<CellCache>,
    cell_group_4: Vec<CellCache>,
    has_fill_cells_u: bool,
    has_fill_cells_v: bool,
    has_group_3_cells_in_auto_rows: bool,
}

impl DefaultGridStyle {
    pub fn new(_params: DefaultGridStyleParams) -> Self {
        DefaultGridStyle {
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
            has_fill_cells_u: false,
            has_fill_cells_v: false,
            has_group_3_cells_in_auto_rows: false,
        }
    }

    fn decide_number_of_rows_and_columns(data: &Grid, children: &Children) -> (usize, usize) {
        let mut max_row_from_attached = -1;
        let mut max_column_from_attached = -1;
        for child in children.into_iter() {
            let child = child.borrow();
            let map = child.get_context().get_attached_values();

            let max_row = if let Some(row) = map.get::<Row>() {
                if let Some(row_span) = map.get::<RowSpan>() {
                    row + row_span - 1
                } else {
                    *row
                }
            } else {
                -1
            };
            max_row_from_attached = max_row_from_attached.max(max_row);

            let max_column = if let Some(column) = map.get::<Column>() {
                if let Some(column_span) = map.get::<ColumnSpan>() {
                    column + column_span - 1
                } else {
                    *column
                }
            } else {
                -1
            };
            max_column_from_attached = max_column_from_attached.max(max_column);
        }

        let is_horizontal_flow = data.columns > 0;

        let number_of_columns;
        let number_of_rows;
        if max_row_from_attached >= 0 || max_column_from_attached >= 0 {
            number_of_rows = data.rows.max(max_row_from_attached + 1).max(0);
            number_of_columns = data.columns.max(max_column_from_attached + 1).max(0);
        } else {
            if is_horizontal_flow {
                number_of_columns = data.columns;
                number_of_rows = if data.rows > 0 {
                    data.rows
                } else {
                    (children.len() as i32 - 1) / number_of_columns + 1
                };
            } else if data.rows > 0 {
                number_of_rows = data.rows;
                number_of_columns = if data.columns > 0 {
                    data.columns
                } else {
                    (children.len() as i32 - 1) / number_of_rows + 1
                }
            } else {
                number_of_columns = 0;
                number_of_rows = 0;
            }
        }

        (number_of_rows as usize, number_of_columns as usize)
    }

    fn prepare_definitions(
        &mut self,
        data: &Grid,
        _children: &Children,
        number_of_rows: usize,
        number_of_columns: usize,
        size_to_content_u: bool,
        size_to_content_v: bool,
    ) {
        let mut widths = Vec::with_capacity(number_of_columns);
        let mut min_widths = Vec::with_capacity(number_of_columns);
        let mut max_widths = Vec::with_capacity(number_of_columns);
        let mut heights = Vec::with_capacity(number_of_rows);
        let mut min_heights = Vec::with_capacity(number_of_rows);
        let mut max_heights = Vec::with_capacity(number_of_rows);
        for _ in 0..number_of_columns {
            widths.push(data.default_width);
            min_widths.push(data.default_min_width);
            max_widths.push(data.default_max_width);
        }
        for _ in 0..number_of_rows {
            heights.push(data.default_height);
            min_heights.push(data.default_min_height);
            max_heights.push(data.default_max_height);
        }
        for (column, width) in &data.widths {
            if *column >= 0 && *column <= widths.len() as i32 {
                widths[*column as usize] = *width;
            }
        }
        for (column, min_width) in &data.min_widths {
            if *column >= 0 && *column <= min_widths.len() as i32 {
                min_widths[*column as usize] = *min_width;
            }
        }
        for (column, max_width) in &data.max_widths {
            if *column >= 0 && *column <= max_widths.len() as i32 {
                max_widths[*column as usize] = *max_width;
            }
        }
        for (row, height) in &data.heights {
            if *row >= 0 && *row <= heights.len() as i32 {
                heights[*row as usize] = *height;
            }
        }
        for (row, min_height) in &data.min_heights {
            if *row >= 0 && *row <= min_heights.len() as i32 {
                min_heights[*row as usize] = *min_height;
            }
        }
        for (row, max_height) in &data.max_heights {
            if *row >= 0 && *row <= max_heights.len() as i32 {
                max_heights[*row as usize] = *max_height;
            }
        }

        self.definitions_u = Vec::new();
        for i in 0..number_of_columns {
            let definition =
                DefinitionBase::new(widths[i], min_widths[i], max_widths[i], size_to_content_u);
            self.definitions_u.push(definition);
        }

        self.definitions_v = Vec::new();
        for i in 0..number_of_rows {
            let definition = DefinitionBase::new(
                heights[i],
                min_heights[i],
                max_heights[i],
                size_to_content_v,
            );
            self.definitions_v.push(definition);
        }
    }

    fn prepare_cell_cache(&mut self, data: &Grid, children: &Children) {
        self.has_fill_cells_u = false;
        self.has_fill_cells_v = false;
        self.has_group_3_cells_in_auto_rows = false;

        let mut child_index = 0;
        let mut column_index = 0;
        let mut row_index = 0;

        self.cell_group_1 = Vec::new();
        self.cell_group_2 = Vec::new();
        self.cell_group_3 = Vec::new();
        self.cell_group_4 = Vec::new();

        for child in children.into_iter() {
            let mut column_span = 1;
            let mut row_span = 1;

            let child = child.borrow();
            let map = child.get_context().get_attached_values();
            if let Some(row) = map.get::<Row>() {
                row_index = *row;
            }
            if let Some(column) = map.get::<Column>() {
                column_index = *column;
            }
            if let Some(rspan) = map.get::<RowSpan>() {
                row_span = *rspan;
            }
            if let Some(cspan) = map.get::<ColumnSpan>() {
                column_span = *cspan;
            }

            if column_index >= 0
                && column_index < self.definitions_u.len() as i32
                && row_index >= 0
                && row_index < self.definitions_v.len() as i32
                && column_span >= 1
                && row_span >= 1
                && column_index + column_span - 1 < self.definitions_u.len() as i32
                && row_index + row_span - 1 < self.definitions_v.len() as i32
            {
                let mut is_fill_u = false;
                let mut is_auto_u = false;
                let mut is_fill_v = false;
                let mut is_auto_v = false;

                for i in column_index..column_index + column_span {
                    match self.definitions_u[i as usize].user_size {
                        Length::Fill(_) => is_fill_u = true,
                        Length::Auto => is_auto_u = true,
                        _ => (),
                    }
                }

                for i in row_index..row_index + row_span {
                    match self.definitions_v[i as usize].user_size {
                        Length::Fill(_) => is_fill_v = true,
                        Length::Auto => is_auto_v = true,
                        _ => (),
                    }
                }

                self.has_fill_cells_u |= is_fill_u;
                self.has_fill_cells_v |= is_fill_v;

                let cell_cache = CellCache {
                    child_index: child_index,
                    column_index: column_index as usize,
                    row_index: row_index as usize,
                    column_span: column_span as usize,
                    row_span: row_span as usize,
                    is_fill_u: is_fill_u,
                    is_auto_u: is_auto_u,
                    is_fill_v: is_fill_v,
                    is_auto_v: is_auto_v,
                };

                if !is_fill_v {
                    if !is_fill_u {
                        self.cell_group_1.push(cell_cache);
                    } else {
                        self.cell_group_3.push(cell_cache);
                        self.has_group_3_cells_in_auto_rows |= is_auto_v;
                    }
                } else {
                    if is_auto_u && !is_fill_u {
                        self.cell_group_2.push(cell_cache);
                    } else {
                        self.cell_group_4.push(cell_cache);
                    }
                }
            }

            child_index += 1;

            let is_horizontal_flow = data.columns > 0;
            if is_horizontal_flow {
                column_index += 1;
                if column_index >= self.definitions_u.len() as i32 {
                    column_index = 0;
                    row_index += 1;
                }
            } else {
                row_index += 1;
                if row_index >= self.definitions_v.len() as i32 {
                    row_index = 0;
                    column_index += 1;
                }
            }
        }
    }

    fn measure_cells_group(
        drawing_context: &mut dyn DrawingContext,
        definitions_u: &mut Vec<DefinitionBase>,
        definitions_v: &mut Vec<DefinitionBase>,
        cells: &Vec<CellCache>,
        children: &Children,
        ignore_desired_size_u: bool,
        force_infinity_v: bool,
    ) -> bool {
        let mut has_desired_size_u_changed = false;

        let mut span_store = HashMap::new();
        let ignore_desired_size_v = force_infinity_v;

        for cell in cells {
            let child = children.get(cell.child_index).unwrap();

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
        drawing_context: &mut dyn DrawingContext,
        cell: &CellCache,
        child: &Rc<RefCell<dyn ControlObject>>,
        definitions_u: &mut Vec<DefinitionBase>,
        definitions_v: &mut Vec<DefinitionBase>,
        force_infinity_v: bool,
    ) {
        let cell_measure_width;
        let cell_measure_height;

        if cell.is_auto_u && !cell.is_fill_u {
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
        } else if cell.is_auto_v && !cell.is_fill_v {
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
            measure_size += if let Length::Auto = definitions[i].size_type {
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
            if let Length::Auto = definitions[i].user_size {
                auto_definitions_count += 1;
            }

            temp_definitions.push(i);
        }

        if requested_size <= range_min_size {
            return;
        }

        if requested_size <= range_preferred_size {
            temp_definitions.sort_by(|x, y| {
                if let Length::Auto = definitions[*x].user_size {
                    if let Length::Auto = definitions[*y].user_size {
                        definitions[*x]
                            .min_size
                            .partial_cmp(&definitions[*y].min_size)
                            .unwrap()
                    } else {
                        std::cmp::Ordering::Less
                    }
                } else {
                    if let Length::Auto = definitions[*y].user_size {
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
                if let Length::Auto = definitions[*x].user_size {
                    if let Length::Auto = definitions[*y].user_size {
                        definitions[*x]
                            .size_cache
                            .partial_cmp(&definitions[*y].size_cache)
                            .unwrap()
                    } else {
                        std::cmp::Ordering::Greater
                    }
                } else {
                    if let Length::Auto = definitions[*y].user_size {
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

    fn resolve_fill(definitions: &mut Vec<DefinitionBase>, available_size: f32) {
        let def_count = definitions.len();
        let mut definition_indices_min = Vec::<i32>::with_capacity(def_count);
        let mut definition_indices_max = Vec::<i32>::with_capacity(def_count);
        let mut taken_size = 0.0f32;
        let mut fill_count = 0;
        let mut scale = 1.0f32;

        // Phase 1.  Determine the maximum *-weight and prepare to adjust *-weights
        let mut max_fill = 0.0f32;
        for def in definitions.iter_mut() {
            if let Length::Fill(v) = def.size_type {
                fill_count += 1;
                def.measure_size = 1.0f32;
                max_fill = max_fill.max(v);
            }
        }

        if max_fill.is_infinite() && max_fill.is_sign_positive() {
            scale = -1.0f32;
        } else if fill_count > 0 {
            let power = (f32::MAX / max_fill / (fill_count as f32)).log2().floor();
            if power < 0.0 {
                scale = 2.0f32.powf(power - 4.0f32);
            }
        }

        // Phase 2 & 3
        let mut run_phase_2_and_3 = true;
        while run_phase_2_and_3 {
            // Phase 2.   Compute total *-weight W and available space S.
            let mut total_fill_weight = 0.0f32;
            taken_size = 0.0f32;
            let mut min_count = 0;
            let mut max_count = 0;
            definition_indices_min.truncate(0);
            definition_indices_max.truncate(0);

            for (i, def) in definitions.iter_mut().enumerate() {
                match def.size_type {
                    Length::Auto => {
                        taken_size += def.min_size;
                    }
                    Length::Exact(_) => {
                        taken_size += def.measure_size;
                    }
                    Length::Fill(v) => {
                        if def.measure_size < 0.0f32 {
                            taken_size += -def.measure_size;
                        } else {
                            let fill_weight = Self::get_fill_weight(v, scale);
                            total_fill_weight += fill_weight;

                            if def.min_size > 0.0f32 {
                                definition_indices_min.push(i as i32);
                                min_count += 1;
                                def.measure_size = fill_weight / def.min_size;
                            }

                            let effective_max_size = def.min_size.max(def.user_max_size);
                            if !effective_max_size.is_infinite()
                                || !effective_max_size.is_sign_positive()
                            {
                                definition_indices_max.push(i as i32);
                                max_count += 1;
                                def.size_cache = fill_weight / effective_max_size;
                            }
                        }
                    }
                };
            }

            // Phase 3.  Resolve *-items whose proportional sizes are too big or too small.
            let min_count_phase2 = min_count;
            let max_count_phase2 = max_count;
            let mut taken_fill_weight = 0.0f32;
            let mut remaining_available_size = available_size - taken_size;
            let mut remaining_fill_weight = total_fill_weight - taken_fill_weight;

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
                if remaining_fill_weight < total_fill_weight / 256.0f32 {
                    taken_fill_weight = 0.0f32;
                    total_fill_weight = 0.0f32;

                    for def in definitions.iter_mut() {
                        if let Length::Fill(v) = def.size_type {
                            if def.measure_size > 0.0f32 {
                                total_fill_weight += Self::get_fill_weight(v, scale);
                            }
                        }
                    }

                    remaining_fill_weight = total_fill_weight - taken_fill_weight;
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
                let proportion = remaining_fill_weight / remaining_available_size;

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
                if let Length::Fill(v) = resolved_def.user_size {
                    taken_fill_weight += Self::get_fill_weight(v, scale);
                    fill_count -= 1;
                }

                remaining_available_size = available_size - taken_size;
                remaining_fill_weight = total_fill_weight - taken_fill_weight;

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
            if fill_count == 0 && taken_size < available_size {
                for i in min_count..min_count_phase2 {
                    let index = definition_indices_min[i];
                    if index >= 0 {
                        definitions[index as usize].measure_size = 1.0f32;
                        fill_count += 1;
                        run_phase_2_and_3 = true;
                    }
                }
            }

            if taken_size > available_size {
                for i in max_count..max_count_phase2 {
                    let index = definition_indices_max[i];
                    if index >= 0 {
                        definitions[index as usize].measure_size = 1.0f32;
                        fill_count += 1;
                        run_phase_2_and_3 = true;
                    }
                }
            }
        }

        // Phase 4.  Resolve the remaining defs proportionally.
        fill_count = 0;
        definition_indices_min.truncate(0);
        for i in 0..def_count {
            let def = &mut definitions[i];

            if let Length::Fill(v) = def.size_type {
                if def.measure_size < 0.0f32 {
                    def.measure_size = -def.measure_size;
                } else {
                    definition_indices_min.push(i as i32);
                    fill_count += 1;
                    def.measure_size = Self::get_fill_weight(v, scale);
                }
            }
        }

        if fill_count > 0 {
            definition_indices_min.sort_by(|x, y| {
                definitions[*x as usize]
                    .measure_size
                    .partial_cmp(&definitions[*y as usize].measure_size)
                    .unwrap()
            });

            let mut total_fill_weight = 0.0f32;
            for i in 0..fill_count {
                let def = &mut definitions[definition_indices_min[i] as usize];
                total_fill_weight += def.measure_size;
                def.size_cache = total_fill_weight;
            }

            for i in (0..fill_count).rev() {
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
        let mut fill_count = 0;
        let mut scale = 1.0f32;

        // Phase 1.  Determine the maximum *-weight and prepare to adjust *-weights
        let mut max_fill = 0.0f32;
        for def in definitions.iter_mut() {
            if let Length::Fill(v) = def.user_size {
                fill_count += 1;
                def.measure_size = 1.0f32;
                max_fill = max_fill.max(v);
            }
        }

        if max_fill.is_infinite() && max_fill.is_sign_positive() {
            scale = -1.0f32;
        } else if fill_count > 0 {
            let power = (f32::MAX / (max_fill as f32) / (fill_count as f32))
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
            let mut total_fill_weight = 0.0f32;
            taken_size = 0.0f32;
            let mut min_count = 0;
            let mut max_count = 0;
            definition_indices_min.truncate(0);
            definition_indices_max.truncate(0);

            for (i, def) in definitions.iter_mut().enumerate() {
                if let Length::Fill(v) = def.user_size {
                    if def.measure_size < 0.0f32 {
                        taken_size += -def.measure_size;
                    } else {
                        let fill_weight = Self::get_fill_weight(v, scale);
                        total_fill_weight += fill_weight;

                        let min_size_for_arrange = def.get_min_size_for_arrange();
                        if min_size_for_arrange > 0.0f32 {
                            definition_indices_min.push(i as i32);
                            min_count += 1;
                            def.measure_size = fill_weight / min_size_for_arrange;
                        }

                        let effective_max_size = min_size_for_arrange.max(def.user_max_size);
                        if !effective_max_size.is_infinite()
                            || !effective_max_size.is_sign_positive()
                        {
                            definition_indices_max.push(i as i32);
                            max_count += 1;
                            def.size_cache = fill_weight / effective_max_size;
                        }
                    }
                } else {
                    let min_size_for_arrange = def.get_min_size_for_arrange();

                    let user_size = match def.user_size {
                        Length::Exact(v) => v,
                        Length::Auto => min_size_for_arrange,
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
            let mut taken_fill_weight = 0.0f32;
            let mut remaining_available_size = final_size - taken_size;
            let mut remaining_fill_weight = total_fill_weight - taken_fill_weight;

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
                if remaining_fill_weight < total_fill_weight / 256.0f32 {
                    taken_fill_weight = 0.0f32;
                    total_fill_weight = 0.0f32;

                    for def in definitions.iter_mut() {
                        if let Length::Fill(v) = def.user_size {
                            if def.measure_size > 0.0f32 {
                                total_fill_weight += Self::get_fill_weight(v, scale);
                            }
                        }
                    }

                    remaining_fill_weight = total_fill_weight - taken_fill_weight;
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
                let proportion = remaining_fill_weight / remaining_available_size;

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
                if let Length::Fill(v) = resolved_def.user_size {
                    taken_fill_weight += Self::get_fill_weight(v, scale);
                    fill_count -= 1;
                }

                remaining_available_size = final_size - taken_size;
                remaining_fill_weight = total_fill_weight - taken_fill_weight;

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
            if fill_count == 0 && taken_size < final_size {
                for i in min_count..min_count_phase2 {
                    let index = definition_indices_min[i];
                    if index >= 0 {
                        definitions[index as usize].measure_size = 1.0f32;
                        fill_count += 1;
                        run_phase_2_and_3 = true;
                    }
                }
            }

            if taken_size > final_size {
                for i in max_count..max_count_phase2 {
                    let index = definition_indices_max[i];
                    if index >= 0 {
                        definitions[index as usize].measure_size = 1.0f32;
                        fill_count += 1;
                        run_phase_2_and_3 = true;
                    }
                }
            }
        }

        // Phase 4.  Resolve the remaining defs proportionally.
        fill_count = 0;
        definition_indices_min.truncate(0);
        for i in 0..def_count {
            let def = &mut definitions[i];

            if let Length::Fill(v) = def.user_size {
                if def.measure_size < 0.0f32 {
                    def.size_cache = -def.measure_size;
                } else {
                    definition_indices_min.push(i as i32);
                    fill_count += 1;
                    def.measure_size = Self::get_fill_weight(v, scale);
                }
            }
        }

        if fill_count > 0 {
            definition_indices_min.sort_by(|x, y| {
                definitions[*x as usize]
                    .measure_size
                    .partial_cmp(&definitions[*y as usize].measure_size)
                    .unwrap()
            });

            let mut total_fill_weight = 0.0f32;
            for i in 0..fill_count {
                let def = &mut definitions[definition_indices_min[i] as usize];
                total_fill_weight += def.measure_size;
                def.size_cache = total_fill_weight;
            }

            for i in (0..fill_count).rev() {
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
                let rounded_size = round_layout_value(def.size_cache, dpi_scale);
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
    fn get_fill_weight(v: f32, scale: f32) -> f32 {
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
        return definitions.iter().map(|d| d.min_size).sum();
    }
}

impl Style<Grid> for DefaultGridStyle {
    fn setup(&mut self, _data: &mut Grid, _control_context: &mut ControlContext) {}

    fn handle_event(
        &mut self,
        _data: &mut Grid,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Grid,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
        let mut grid_desired_size = Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);

        let children = control_context.get_children();

        let (number_of_rows, number_of_columns) =
            Self::decide_number_of_rows_and_columns(data, children);

        if number_of_rows == 0 && number_of_columns == 0 {
            self.definitions_u = Vec::new();
            self.definitions_v = Vec::new();

            for child in children.into_iter() {
                let mut child = child.borrow_mut();
                child.measure(drawing_context, size);
                let child_rc = child.get_rect();
                grid_desired_size.width = grid_desired_size.width.max(child_rc.width);
                grid_desired_size.height = grid_desired_size.height.max(child_rc.height);
            }
        } else {
            let size_to_content_u = size.width == f32::INFINITY;
            let size_to_content_v = size.height == f32::INFINITY;

            self.prepare_definitions(
                &data,
                &children,
                number_of_rows,
                number_of_columns,
                size_to_content_u,
                size_to_content_v,
            );
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
                if self.has_fill_cells_v {
                    Self::resolve_fill(&mut self.definitions_v, size.height);
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

                if self.has_fill_cells_u {
                    Self::resolve_fill(&mut self.definitions_u, size.width);
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
                    if self.has_fill_cells_u {
                        Self::resolve_fill(&mut self.definitions_u, size.width);
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
                    if self.has_fill_cells_v {
                        Self::resolve_fill(&mut self.definitions_v, size.height);
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

                        if self.has_fill_cells_u {
                            Self::resolve_fill(&mut self.definitions_u, size.width);
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

                        if self.has_fill_cells_v {
                            Self::resolve_fill(&mut self.definitions_v, size.height);
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

    fn set_rect(&mut self, _data: &mut Grid, control_context: &mut ControlContext, rect: Rect) {
        self.rect = rect;

        let children = control_context.get_children();

        if self.definitions_u.len() == 0 && self.definitions_v.len() == 0 {
            for child in children.into_iter() {
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
                let child = children.get(cell.child_index).unwrap();
                let column_index = cell.column_index;
                let row_index = cell.row_index;
                let column_span = cell.column_span;
                let row_span = cell.row_span;
                let rc = Rect::new(
                    if column_index == 0 {
                        rect.x
                    } else {
                        rect.x + self.definitions_u[column_index].final_offset
                    },
                    if row_index == 0 {
                        rect.y
                    } else {
                        rect.y + self.definitions_v[row_index].final_offset
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

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &Grid,
        control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            let children = control_context.get_children();
            for child in children.into_iter().rev() {
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
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let children = control_context.get_children();
        for child in children.into_iter() {
            let (mut vec2, mut overlay2) = child.borrow().to_primitives(drawing_context);
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}
