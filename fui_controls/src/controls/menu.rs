use fui_core::{ControlObject, Property, Style, TypeMapKey, ViewContext};
use fui_drawing::Color;
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use typed_builder::TypedBuilder;

use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::controls::*;
use crate::GestureArea;
use fui_core::*;

// ============================================================================
// Attached Values for Menu State Tracking
// ============================================================================

pub struct ActiveMenu;

impl TypeMapKey for ActiveMenu {
    // 0 = brak aktywnego menu, > 0 = ID aktualnie otwartego menu
    type Value = Property<i32>;
}

// Global counters/state for robust menu bar handling
static MENU_ID_COUNTER: AtomicI32 = AtomicI32::new(1);
static LAST_ACTIVE_TIME: AtomicU64 = AtomicU64::new(0);

fn get_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
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
        let active_menu_prop = Property::new(0i32);
        let mut attached_values = context.attached_values;
        attached_values.insert::<ActiveMenu>(active_menu_prop.clone());

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
        menu_impl(context, true)
    }
}

// ============================================================================
// SubMenu
// ============================================================================

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
    let my_menu_id = MENU_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let is_open_prop = Property::new(false);

    let children: Vec<_> = context.children.into_iter().collect();
    if children.is_empty() {
        return ui!(Text { text: "" });
    }

    let trigger = children.first().unwrap().clone();
    let background_property = Property::new(Color::rgba(0.0, 0.0, 0.0, 0.0));

    let popup_children: Vec<_> = children.iter().skip(1).cloned().collect();
    let has_popup_content = !popup_children.is_empty();

    let self_weak: Rc<RefCell<Option<Weak<dyn ControlObject>>>> = Rc::new(RefCell::new(None));

    let get_active_menu = {
        let self_weak = self_weak.clone();
        move || -> Option<Property<i32>> {
            if !is_top_level {
                return None;
            }
            let weak = self_weak.borrow();
            weak.as_ref()
                .and_then(|w| w.upgrade())
                .and_then(|ctrl| ctrl.get_context().get_inherited_value::<ActiveMenu>())
        }
    };

    let mut popup_view = Children::None;

    if has_popup_content {
        let popup_placement = if is_top_level {
            PopupPlacement::BelowOrAboveParent
        } else {
            PopupPlacement::LeftOrRightParent
        };

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

        uncovered_controls.push(Rc::downgrade(&popup_content));

        let auto_hide_occured_callback = {
            let background_property = background_property.clone();
            let is_open_prop = is_open_prop.clone();
            let get_active_menu = get_active_menu.clone();

            Callback::new_sync(move |_| {
                is_open_prop.set(false);
                background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.0));

                if let Some(active_menu) = get_active_menu() {
                    // Tylko czyscimy ActiveMenu jesli to MY nim aktualnie zarzadzamy!
                    // Zapobiega to zamykaniu menu podczas przelaczania (Issue 2)
                    if active_menu.get() == my_menu_id {
                        active_menu.set(0);
                        LAST_ACTIVE_TIME.store(get_time_ms(), Ordering::Relaxed);
                    }
                }
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

    let trigger_with_gestures = {
        let on_hover_callback = {
            let background_property = background_property.clone();
            let is_open_prop = is_open_prop.clone();
            let get_active_menu = get_active_menu.clone();

            Callback::new_sync(move |value: bool| {
                if value && !is_top_level {
                    if has_popup_content {
                        is_open_prop.set(true);
                    }
                    background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.8));
                } else if !is_top_level {
                    background_property.set(if is_open_prop.get() {
                        Color::rgba(0.0, 0.0, 0.0, 0.8)
                    } else {
                        Color::rgba(0.0, 0.0, 0.0, 0.0)
                    });
                } else if is_top_level {
                    let mut is_active = false;

                    if let Some(active_menu) = get_active_menu() {
                        let current_active = active_menu.get();

                        // Zostawiamy niewielki zapas czasu po "auto_hide" poprzedniego menu, zeby rodzeństwo moglo prawidlowo sie otworzyc.
                        let recently_closed =
                            (get_time_ms() - LAST_ACTIVE_TIME.load(Ordering::Relaxed)) < 150;

                        // Jesli inna kontrolka jest aktywna LUB menu zostalo zaledwie chwile temu zamkniete:
                        if current_active > 0 || recently_closed {
                            is_active = true;
                            if value && !is_open_prop.get() && has_popup_content {
                                is_open_prop.set(true);
                                active_menu.set(my_menu_id);
                            }
                        }
                    }

                    // Tlo zmieniamy TYLKO wtedy, gdy menu bar jest w istocie aktywny albo klikniety!
                    // Rozwiazuje Issue 1.
                    background_property.set(if is_open_prop.get() || (value && is_active) {
                        Color::rgba(0.0, 0.0, 0.0, 0.8)
                    } else if value {
                        Color::rgba(0.0, 0.0, 0.0, 0.1) // Delikatny highlight dla nieaktywnego paska (opcjonalnie mozesz ustwić 0.0)
                    } else {
                        Color::rgba(0.0, 0.0, 0.0, 0.0)
                    });
                }
            })
        };

        let on_tap_up_callback = {
            let is_open_prop = is_open_prop.clone();
            let get_active_menu = get_active_menu.clone();
            let background_property = background_property.clone();

            Callback::new_sync(move |_| {
                if has_popup_content {
                    if is_top_level {
                        let currently_open = is_open_prop.get();
                        if currently_open {
                            is_open_prop.set(false);
                            background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.0));
                            if let Some(active_menu) = get_active_menu() {
                                if active_menu.get() == my_menu_id {
                                    active_menu.set(0);
                                    LAST_ACTIVE_TIME.store(get_time_ms(), Ordering::Relaxed);
                                }
                            }
                        } else {
                            is_open_prop.set(true);
                            background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.8));
                            if let Some(active_menu) = get_active_menu() {
                                active_menu.set(my_menu_id);
                            }
                        }
                    } else {
                        let currently_open = is_open_prop.get();
                        is_open_prop.set(!currently_open);
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

    let result = if is_top_level {
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
    };

    *self_weak.borrow_mut() = Some(Rc::downgrade(&result));

    result
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

        let children: Vec<_> = context.children.into_iter().collect();
        if children.is_empty() {
            return ui!(Text { text: "" });
        }

        let content = children.first().unwrap().clone();

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
