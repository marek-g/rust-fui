use fui_core::{
    Children, CompositeControl, ControlContext, ControlObject, ObservableVec, Property, Style,
    TypeMap, TypeMapKey, ViewContext,
};
use fui_drawing::Color;
use fui_macros::ui;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use typed_builder::TypedBuilder;

use crate::GestureArea;
use crate::{controls::*, DataHolder};
use fui_core::*;

// ============================================================================
// Attached Values for Menu State Tracking
// ============================================================================

/// Shared state for a MenuBar and all its nested children.
pub struct MenuData {
    /// Holds the ID of the currently open top-level menu. 0 if none are active.
    pub active_menu_id: Property<i32>,
    /// References to all top-level triggers to exclude them from auto-hide hits.
    pub top_level_triggers: Rc<RefCell<Vec<Weak<dyn ControlObject>>>>,
    /// Internal counter to assign unique IDs to top-level menus.
    pub menu_id_counter: Cell<i32>,
    /// Signal used to close all open menus in this bar.
    pub close_all: Callback<()>,
}

/// Context key to access shared MenuData.
pub struct ActiveMenu;
impl TypeMapKey for ActiveMenu {
    type Value = Rc<MenuData>;
}

/// Context key used to detect if a Menu is nested inside another Menu.
struct IsInsideMenu;
impl TypeMapKey for IsInsideMenu {
    type Value = bool;
}

// ============================================================================
// MenuBar
// ============================================================================

#[derive(TypedBuilder)]
pub struct MenuBar {}

impl MenuBar {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<dyn ControlObject> {
        let active_menu_id = Property::new(0i32);
        let active_menu_id_clone = active_menu_id.clone();

        // Initialize shared state for the entire menu bar
        let menu_data = Rc::new(MenuData {
            active_menu_id,
            top_level_triggers: Rc::new(RefCell::new(Vec::new())),
            menu_id_counter: Cell::new(1),
            // Logic to reset the menu bar state
            close_all: Callback::new_sync(move |_| {
                active_menu_id_clone.set(0);
            }),
        });

        let mut attached_values = context.attached_values;
        attached_values.insert::<ActiveMenu>(menu_data);

        let updated_context = ViewContext {
            attached_values,
            children: context.children,
        };

        let stack_panel = StackPanel::builder()
            .orientation(Orientation::Horizontal)
            .build()
            .to_view(None, updated_context);

        ui!(
            Shadow {
                Style: Default { size: 12.0f32 },
                Border {
                    border_type: BorderType::None,
                    Style: Default { background_color: Color::rgba(1.0, 1.0, 1.0, 0.8) },
                    stack_panel,
                }
            }
        )
    }
}

// ============================================================================
// Menu
// ============================================================================

#[derive(TypedBuilder)]
pub struct Menu {}

impl Menu {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<dyn ControlObject> {
        CompositeControl::new(context, move |ctx: &ControlContext| {
            // Retrieve shared state from MenuBar
            let menu_data = ctx.get_inherited_value::<ActiveMenu>();

            // Automatic Top-Level detection:
            // It's a top-level menu if we have MenuData but we aren't marked as "inside" another menu yet.
            let is_top_level =
                menu_data.is_some() && ctx.get_inherited_value::<IsInsideMenu>().is_none();

            // Prepare attached values for children so they know they are nested
            let mut inner_values = TypeMap::new();
            inner_values.insert::<IsInsideMenu>(true);

            menu_impl(ctx.get_children(), menu_data, is_top_level, inner_values)
        })
    }
}

// ============================================================================
// Common Menu Implementation
// ============================================================================

fn menu_impl(
    children_collection: &Children,
    menu_data: Option<Rc<MenuData>>,
    is_top_level: bool,
    attached_values: TypeMap,
) -> Rc<dyn ControlObject> {
    // Assign a unique ID if this is a top-level trigger on the bar
    let my_menu_id = if let Some(md) = &menu_data {
        if is_top_level {
            let id = md.menu_id_counter.get();
            md.menu_id_counter.set(id + 1);
            id
        } else {
            0
        }
    } else {
        0
    };

    let is_open_prop = Property::new(false);
    let children: Vec<_> = children_collection.into_iter().collect();

    if children.is_empty() {
        return ui!(Text { text: "" });
    }

    // First child is the label/trigger, subsequent children are the popup content
    let trigger = children.first().unwrap().clone();
    let background_property = Property::new(Color::rgba(0.0, 0.0, 0.0, 0.0));

    // Listen for global active menu changes to auto-close when another menu opens or all menus are closed
    let mut subscription = None;
    if let Some(md) = &menu_data {
        let is_open_prop_clone = is_open_prop.clone();
        let background_property_clone = background_property.clone();
        let my_id = my_menu_id;

        subscription = Some(md.active_menu_id.on_changed(move |new_active_id: i32| {
            // Updated logic: if it's a top-level, close if another one opens or if all close (0).
            // If it's a sub-menu, close if all close (0).
            let should_close = if is_top_level {
                new_active_id != my_id
            } else {
                new_active_id == 0
            };

            if should_close && is_open_prop_clone.get() {
                is_open_prop_clone.set(false);
                background_property_clone.set(Color::rgba(0.0, 0.0, 0.0, 0.0));
            }
        }));
    }

    let popup_children: Vec<_> = children.iter().skip(1).cloned().collect();
    let has_popup_content = !popup_children.is_empty();

    // Register this trigger in the global MenuBar state for "uncovered" hit testing
    if is_top_level {
        if let Some(md) = &menu_data {
            md.top_level_triggers
                .borrow_mut()
                .push(Rc::downgrade(&trigger));
        }
    }

    let mut popup_view = Children::None;
    let uncovered_controls = Rc::new(RefCell::new(Vec::new()));
    let popup_content_weak: Rc<RefCell<Option<Weak<dyn ControlObject>>>> =
        Rc::new(RefCell::new(None));

    if has_popup_content {
        let popup_placement = if is_top_level {
            PopupPlacement::BelowOrAboveParent
        } else {
            PopupPlacement::LeftOrRightParent
        };

        let popup_content_prop = ObservableVec::new();
        for child in popup_children.iter() {
            popup_content_prop.push(child.clone());
        }

        // The container for the menu items inside the popup
        let popup_content: Rc<dyn ControlObject> = ui!(
            Shadow {
                Style: Default { size: 12.0f32 },
                Border {
                    border_type: BorderType::Raisen,
                    Style: Default { background_color: Color::rgba(1.0, 1.0, 1.0, 0.8) },
                    Grid {
                        columns: 1,
                        default_width: Length::Fill(1.0f32),
                        default_height: Length::Auto,
                        &popup_content_prop,
                    }
                }
            }
        );

        *popup_content_weak.borrow_mut() = Some(Rc::downgrade(&popup_content));

        let auto_hide_occured_callback = {
            let background_property = background_property.clone();
            let is_open_prop = is_open_prop.clone();
            let menu_data = menu_data.clone();

            Callback::new_sync(move |_| {
                is_open_prop.set(false);
                background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.0));

                // Reset global state so clicking is required again
                if let Some(md) = &menu_data {
                    if is_top_level && md.active_menu_id.get() == my_menu_id {
                        md.active_menu_id.set(0);
                    }
                }
            })
        };

        let auto_hide = if is_top_level {
            PopupAutoHide::MenuTopLevel
        } else {
            PopupAutoHide::Menu
        };

        let popup = ui!(Popup {
            is_open: is_open_prop.clone(),
            placement: popup_placement,
            auto_hide: auto_hide,
            auto_hide_occured: auto_hide_occured_callback,
            uncovered_controls: uncovered_controls.clone(),
            popup_content,
        });

        popup_view = Children::SingleStatic(popup);
    }

    // Helper: Syncs the list of controls that shouldn't close the popup when clicked
    let sync_uncovered_controls = {
        let menu_data = menu_data.clone();
        let uncovered_controls = uncovered_controls.clone();
        let popup_content_weak = popup_content_weak.clone();

        move || {
            let mut unc = uncovered_controls.borrow_mut();
            unc.clear();

            // Add all sibling top-level triggers
            if let Some(md) = &menu_data {
                for t in md.top_level_triggers.borrow().iter() {
                    unc.push(t.clone());
                }
            }
            // Add the popup content itself
            if let Some(pc) = popup_content_weak.borrow().as_ref() {
                unc.push(pc.clone());
            }
        }
    };

    let trigger_with_gestures = {
        let on_hover_callback = {
            let background_property = background_property.clone();
            let is_open_prop = is_open_prop.clone();
            let menu_data = menu_data.clone();
            let sync_uncovered_controls = sync_uncovered_controls.clone();

            Callback::new_sync(move |is_hovered: bool| {
                if !is_top_level {
                    // Sub-menu logic: open on hover if any parent is open
                    if is_hovered && has_popup_content {
                        sync_uncovered_controls();
                        is_open_prop.set(true);
                    }
                    background_property.set(if is_hovered || is_open_prop.get() {
                        Color::rgba(0.0, 0.0, 0.0, 0.8)
                    } else {
                        Color::rgba(0.0, 0.0, 0.0, 0.0)
                    });
                } else if let Some(md) = &menu_data {
                    // Top-level logic: only switch on hover if another menu is already active
                    let current_active = md.active_menu_id.get();

                    if is_hovered && current_active != 0 && current_active != my_menu_id {
                        sync_uncovered_controls();
                        is_open_prop.set(true);
                        md.active_menu_id.set(my_menu_id);
                    }

                    background_property.set(
                        if is_open_prop.get() || (is_hovered && current_active != 0) {
                            Color::rgba(0.0, 0.0, 0.0, 0.8)
                        } else if is_hovered {
                            Color::rgba(0.0, 0.0, 0.0, 0.1)
                        } else {
                            Color::rgba(0.0, 0.0, 0.0, 0.0)
                        },
                    );
                }
            })
        };

        let on_tap_up_callback = {
            let is_open_prop = is_open_prop.clone();
            let menu_data = menu_data.clone();
            let background_property = background_property.clone();
            let sync_uncovered_controls = sync_uncovered_controls.clone();

            Callback::new_sync(move |_| {
                if has_popup_content {
                    let currently_open = is_open_prop.get();
                    if currently_open {
                        // Close menu and reset global active state
                        is_open_prop.set(false);
                        background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.0));
                        if let Some(md) = &menu_data {
                            if is_top_level && md.active_menu_id.get() == my_menu_id {
                                md.active_menu_id.set(0);
                            }
                        }
                    } else {
                        // Open menu and set global active state
                        sync_uncovered_controls();
                        is_open_prop.set(true);
                        background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.8));
                        if let Some(md) = &menu_data {
                            if is_top_level {
                                md.active_menu_id.set(my_menu_id);
                            }
                        }
                    }
                }
            })
        };

        ui!(
            GestureArea {
                hover_change: on_hover_callback,
                tap_up: on_tap_up_callback,
                Border {
                    border_type: BorderType::None,
                    Style: Default { background_color: background_property.clone() },
                    trigger,
                },
                popup_view,
            }
        )
    };

    let content_prop = ObservableVec::new();
    content_prop.push(trigger_with_gestures);

    if is_top_level {
        let content = ui!(
            Shadow {
                Style: Default { size: 12.0f32 },
                Border {
                    border_type: BorderType::None,
                    Style: Default { background_color: Color::rgba(1.0, 1.0, 1.0, 0.8) },
                    StackPanel {
                        orientation: Orientation::Horizontal,
                        &content_prop,
                    }
                }
            }
        );

        let data_holder = DataHolder { data: subscription };
        data_holder.to_view(
            None,
            ViewContext {
                attached_values,
                children: Children::SingleStatic(content),
            },
        )
    } else {
        // Wrap sub-menu in DataHolder to keep the subscription alive
        let content = ui!(
            Shadow {
                Style: Default { size: 12.0f32 },
                Border {
                    border_type: BorderType::None,
                    Style: Default { background_color: Color::rgba(1.0, 1.0, 1.0, 0.8) },
                    StackPanel {
                        orientation: Orientation::Vertical,
                        &content_prop,
                    }
                }
            }
        );

        let data_holder = DataHolder { data: subscription };
        data_holder.to_view(
            None,
            ViewContext {
                attached_values,
                children: Children::SingleStatic(content),
            },
        )
    }
}

// ============================================================================
// MenuItem
// ============================================================================

#[derive(TypedBuilder)]
pub struct MenuItem {
    #[builder(default = Callback::empty())]
    pub activated: Callback<()>,
}

impl MenuItem {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<dyn ControlObject> {
        let activated_callback = self.activated.clone();
        let background_property = Property::new(Color::rgba(0.0, 0.0, 0.0, 0.0));

        // Use CompositeControl to access MenuData via inherited context
        CompositeControl::new(context, move |ctx: &ControlContext| {
            let menu_data = ctx.get_inherited_value::<ActiveMenu>();
            let activated_callback = activated_callback.clone();

            let children: Vec<_> = ctx.get_children().into_iter().collect();
            let content = children
                .first()
                .cloned()
                .unwrap_or_else(|| ui!(Text { text: "" }));

            let on_hover_callback = {
                let background_property = background_property.clone();
                Callback::new_sync(move |value: bool| {
                    background_property.set(if value {
                        Color::rgba(0.0, 0.0, 0.0, 0.8)
                    } else {
                        Color::rgba(0.0, 0.0, 0.0, 0.0)
                    });
                })
            };

            let on_tap_up_callback = Callback::new_sync(move |_| {
                activated_callback.emit(());
                // Signal MenuBar to close all open popups
                if let Some(md) = &menu_data {
                    md.close_all.emit(());
                }
            });

            ui!(
                GestureArea {
                    hover_change: on_hover_callback,
                    tap_up: on_tap_up_callback,
                    Border {
                        border_type: BorderType::None,
                        Style: Default { background_color: background_property.clone() },
                        Grid {
                            columns: 3,
                            widths: vec![
                                (0, Length::Exact(25.0f32)),
                                (1, Length::Fill(1.0f32)),
                                (2, Length::Exact(25.0f32)),
                            ],
                            content,
                        }
                    }
                }
            )
        })
    }
}

// ============================================================================
// MenuSeparator
// ============================================================================

#[derive(TypedBuilder)]
pub struct MenuSeparator {}

impl MenuSeparator {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        _context: ViewContext,
    ) -> Rc<dyn ControlObject> {
        ui!(Border {
            border_type: BorderType::None,
            Style: Default {
                background_color: Color::rgba(0.7, 0.7, 0.7, 1.0),
            },
            Margin: Thickness::new(5.0, 2.0, 5.0, 2.0),
        })
    }
}
