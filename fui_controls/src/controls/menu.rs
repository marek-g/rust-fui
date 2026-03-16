use fui_core::{ControlObject, Property, Style, ViewContext};
use fui_drawing::Color;
use fui_macros::ui;
use std::cell::RefCell;
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
    ) -> Rc<RefCell<dyn ControlObject>> {
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
// Menu
// ============================================================================

/// Menu is a control that renders its first child as a trigger.
/// When clicked or hovered, it shows a popup with the remaining children.
///
/// The first child is typically a Text control showing the menu title.
/// Remaining children are shown in the popup when the menu is opened.
///
/// # Example (top-level menu)
/// ```ignore
/// Menu {
///     Text { text: "File" },  // trigger
///     MenuItem {
///         activated => self.open(),
///         Text { text: "Open" }
///     },
///     MenuSeparator {},
///     MenuItem {
///         activated => self.exit(),
///         Text { text: "Exit" }
///     }
/// }
/// ```
///
/// # Example (submenu)
/// ```ignore
/// Menu {
///     Text { text: "Export >" },  // trigger
///     MenuItem {
///         activated => self.export_pdf(),
///         Text { text: "PDF" }
///     },
///     MenuItem {
///         activated => self.export_png(),
///         Text { text: "PNG" }
///     }
/// }
/// ```
#[derive(TypedBuilder)]
pub struct Menu {}

impl Menu {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        // Shared state for this menu
        let is_open_prop = Property::new(false);
        let is_menu_active_prop = Property::new(false);

        // Callback to close sibling menus
        let close_siblings_callback: Rc<RefCell<Callback<()>>> =
            Rc::new(RefCell::new(Callback::empty()));

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
            let popup_placement = PopupPlacement::LeftOrRightParent;

            // Build popup content
            let popup_content_prop = ObservableVec::new();
            let mut close_item_popup_callbacks = Vec::new();
            let mut close_siblings_callbacks = Vec::new();

            let mut uncovered_controls = vec![];

            for child in popup_children.into_iter() {
                let close_sib_cb: Rc<RefCell<Callback<()>>> =
                    Rc::new(RefCell::new(Callback::empty()));

                popup_content_prop.push(child.clone());
                close_siblings_callbacks.push(close_sib_cb);
                close_item_popup_callbacks.push(Callback::empty());
            }

            // Setup sibling closing logic
            for i in 0..close_siblings_callbacks.len() {
                let callbacks_for_i: Vec<_> = close_item_popup_callbacks
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, cb)| cb.clone())
                    .collect();

                let close_sib_cb = close_siblings_callbacks[i].clone();
                close_sib_cb.borrow_mut().set_sync(move |_| {
                    for cb in &callbacks_for_i {
                        cb.emit(());
                    }
                });
            }

            let popup_content: Rc<RefCell<dyn ControlObject>> = ui!(
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
            // This is the standard behavior for menus
            let is_menu_active_for_autohide = is_menu_active_prop.clone();
            let auto_hide_occured_callback = {
                let background_property = background_property.clone();
                Callback::new_sync(move |_| {
                    is_menu_active_for_autohide.set(false);
                    background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.0));
                })
            };

            // Track is_menu_active to close popup
            let is_open_clone = is_open_prop.clone();
            let _menu_active_subscription = is_menu_active_prop.on_changed(move |value: bool| {
                if !value {
                    is_open_clone.set(false);
                }
            });

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

        // Setup close_siblings callback - close this menu when sibling is opened
        let is_open_clone = is_open_prop.clone();
        close_siblings_callback.borrow_mut().set_sync(move |_| {
            is_open_clone.set(false);
        });

        // On hover:
        // - If menu bar is active, close siblings and open this popup
        // - If menu bar is not active, just open this popup (first hover activation)
        let on_hover_callback = {
            let background_property = background_property.clone();
            let is_open_prop = is_open_prop.clone();
            let is_menu_active_prop = is_menu_active_prop.clone();
            let close_siblings_callback = close_siblings_callback.clone();
            Callback::new_sync(move |value: bool| {
                background_property.set(
                    if is_menu_active_prop.get() && (value || is_open_prop.get()) {
                        Color::rgba(0.0, 0.0, 0.0, 0.8)
                    } else {
                        Color::rgba(0.0, 0.0, 0.0, 0.0)
                    },
                );

                if
                /*is_menu_active_prop.get() &&*/
                value {
                    // Open this popup if it has content
                    if has_popup_content {
                        // Close siblings first (if menu bar is active)
                        // or activate menu bar (if not active yet)
                        if is_menu_active_prop.get() {
                            close_siblings_callback.borrow().emit(());
                        } else {
                            is_menu_active_prop.set(true);
                        }
                        is_open_prop.set(true);
                    }
                }
            })
        };

        // On tap: activate menu
        let on_tap_up_callback = {
            let is_menu_active_prop = is_menu_active_prop.clone();
            let is_open_prop = is_open_prop.clone();
            Callback::new_sync(move |_| {
                if has_popup_content {
                    is_menu_active_prop.set(true);
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
    ) -> Rc<RefCell<dyn ControlObject>> {
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
    ) -> Rc<RefCell<dyn ControlObject>> {
        ui!(Border {
            border_type: BorderType::None,
            Style: Default {
                background_color: Color::rgba(0.7, 0.7, 0.7, 1.0),
            },
            Margin: Thickness::new(5.0, 2.0, 5.0, 2.0),
        })
    }
}
