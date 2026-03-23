use fui_core::{ControlObject, Property, Style, ViewContext};
use fui_drawing::Color;
use fui_macros::ui;
use std::rc::Rc;
use typed_builder::TypedBuilder;

use crate::controls::*;
use crate::GestureArea;
use fui_core::*;

// ============================================================================
// MenuBar
// ============================================================================

/// MenuBar is a top-level horizontal menu bar that contains Menu controls.
///
/// # Example
/// ```ignore
/// MenuBar {
///     Menu {
///         Text { text: "File" },
///         MenuItem { activated => self.open_file(), Text { text: "Open" } },
///     },
///     Menu {
///         Text { text: "Edit" },
///         MenuItem { activated => self.cut(), Text { text: "Cut" } },
///     }
/// }
/// ```
#[derive(TypedBuilder)]
pub struct MenuBar {}

impl MenuBar {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<dyn ControlObject> {
        // Create StackPanel with the same context (preserves children and attached values)
        let stack_panel = StackPanel::builder()
            .orientation(Orientation::Horizontal)
            .build()
            .to_view(None, context);

        // Wrap StackPanel in Border and Shadow
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
// Menu (Top-level menu for MenuBar)
// ============================================================================

/// Menu is a top-level menu control for use in MenuBar.
/// Renders its first child as a trigger in a horizontal layout.
/// When clicked or hovered, it shows a popup with the remaining children.
///
/// The first child is typically a Text control showing the menu title.
/// Remaining children are shown in the popup when the menu is opened.
///
/// # Example
/// ```ignore
/// MenuBar {
///     Menu {
///         Text { text: "File" },
///         MenuItem { activated => self.open(), Text { text: "Open" } },
///         MenuSeparator {},
///         MenuItem { activated => self.exit(), Text { text: "Exit" } },
///     },
/// }
/// ```
#[derive(TypedBuilder)]
pub struct Menu {}

impl Menu {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<dyn ControlObject> {
        menu_impl(context, true)
    }
}

// ============================================================================
// SubMenu (Nested menu inside another Menu)
// ============================================================================

/// SubMenu is a nested menu control for use inside another Menu.
/// Renders its first child as a trigger with a submenu indicator.
/// When hovered, it shows a popup with the remaining children.
///
/// # Example
/// ```ignore
/// Menu {
///     Text { text: "File" },
///     SubMenu {
///         Text { text: "Export >" },
///         MenuItem { activated => self.export_pdf(), Text { text: "PDF" } },
///         MenuItem { activated => self.export_png(), Text { text: "PNG" } },
///     },
/// }
/// ```
#[derive(TypedBuilder)]
pub struct SubMenu {}

impl SubMenu {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<dyn ControlObject> {
        menu_impl(context, false)
    }
}

// ============================================================================
// Common Menu Implementation
// ============================================================================

fn menu_impl(context: ViewContext, is_top_level: bool) -> Rc<dyn ControlObject> {
    // Shared state for this menu
    let is_open_prop = Property::new(false);

    // Collect children
    let children: Vec<_> = context.children.into_iter().collect();
    if children.is_empty() {
        return ui!(Text { text: "" });
    }

    // First child is the trigger
    let trigger = children.first().unwrap().clone();
    let background_property = Property::new(Color::rgba(0.0, 0.0, 0.0, 0.0));

    // Remaining children go into the popup
    let popup_children: Vec<_> = children.iter().skip(1).cloned().collect();
    let has_popup_content = !popup_children.is_empty();

    // Create popup content if there are children after the trigger
    let mut popup_view = Children::None;

    if has_popup_content {
        let popup_placement = if is_top_level {
            PopupPlacement::BelowOrAboveParent
        } else {
            PopupPlacement::LeftOrRightParent
        };

        // Build popup content
        let popup_content_prop = ObservableVec::new();
        let mut uncovered_controls = vec![];

        for child in popup_children.into_iter() {
            popup_content_prop.push(child.clone());
        }

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

        // Add popup content to uncovered controls
        uncovered_controls.push(Rc::downgrade(&popup_content));

        // Use Menu auto-hide - closes when mouse leaves both trigger and popup
        let auto_hide_occured_callback = {
            let background_property = background_property.clone();
            let is_open_prop = is_open_prop.clone();
            Callback::new_sync(move |_| {
                is_open_prop.set(false);
                background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.0));
            })
        };

        let popup = ui!(Popup {
            is_open: is_open_prop.clone(),
            placement: popup_placement,
            auto_hide: PopupAutoHide::Menu,
            auto_hide_occured: auto_hide_occured_callback,
            uncovered_controls: uncovered_controls,

            popup_content,
        });

        popup_view = Children::SingleStatic(popup);
    }

    // On hover:
    // - For top-level Menu: hover does nothing (requires click to activate)
    // - For SubMenu: hover opens the submenu (parent is already open)
    let on_hover_callback = {
        let background_property = background_property.clone();
        let is_open_prop = is_open_prop.clone();
        Callback::new_sync(move |value: bool| {
            background_property.set(
                if value || is_open_prop.get() {
                    Color::rgba(0.0, 0.0, 0.0, 0.8)
                } else {
                    Color::rgba(0.0, 0.0, 0.0, 0.0)
                },
            );

            // Only respond to hover for submenus (top-level requires click)
            if value && !is_top_level {
                // Open this popup if it has content
                if has_popup_content {
                    is_open_prop.set(true);
                }
            }
        })
    };

    // On tap: open this menu
    let on_tap_up_callback = {
        let is_open_prop = is_open_prop.clone();
        Callback::new_sync(move |_| {
            if has_popup_content {
                is_open_prop.set(true);
            }
        })
    };

    let trigger_wrapped = ui!(
        GestureArea {
            hover_change: on_hover_callback,
            tap_up: on_tap_up_callback,

            Border {
                border_type: BorderType::None,
                Style: Default { background_color: background_property },

                trigger,
            },

            popup_view,
        }
    );

    // Build final content
    let content_prop = ObservableVec::new();
    content_prop.push(trigger_wrapped);

    if is_top_level {
        // Top-level menu: horizontal layout in MenuBar
        ui!(
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
        )
    } else {
        // Submenu: vertical layout inside popup
        ui!(
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
        )
    }
}

// ============================================================================
// MenuItem
// ============================================================================

/// MenuItem is a control that renders its first child and has an activated callback.
/// When clicked, the activated callback is called and the menu is closed.
///
/// # Example
/// ```ignore
/// MenuItem {
///     activated => self.open_file(),
///     Text { text: "Open" }
/// }
/// ```
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
        //let foreground_property = Property::new(Color::rgba(0.0, 0.0, 0.0, 1.0));

        // Collect children
        let children: Vec<_> = context.children.into_iter().collect();
        if children.is_empty() {
            return ui!(Text { text: "" });
        }

        let content = children.first().unwrap().clone();

        let on_hover_callback = {
            let background_property = background_property.clone();
            //let foreground_property = foreground_property.clone();
            Callback::new_sync(move |value: bool| {
                background_property.set(if value {
                    // || is_menu_active_prop_clone.get() {
                    Color::rgba(0.0, 0.0, 0.0, 0.8)
                } else {
                    Color::rgba(0.0, 0.0, 0.0, 0.0)
                });

                /*foreground_property.set(if value {
                    // || is_menu_active_prop_clone.get() {
                    Color::rgba(1.0, 1.0, 0.0, 1.0)
                } else {
                    Color::rgba(0.0, 0.0, 0.0, 1.0)
                });*/
            })
        };

        let on_tap_up_callback = Callback::new_sync(move |_| {
            activated_callback.emit(());
        });

        ui!(
            GestureArea {
                hover_change: on_hover_callback,
                tap_up: on_tap_up_callback,

                Border {
                    border_type: BorderType::None,
                    Style: Default { background_color: background_property },

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
    }
}

// ============================================================================
// MenuSeparator
// ============================================================================

/// MenuSeparator is a horizontal line that separates menu items.
///
/// # Example
/// ```ignore
/// MenuSeparator {}
/// ```
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
