use fui_core::{ControlObject, Property, Style, TypeMapKey, ViewContext};
use fui_drawing::Color;
use fui_macros::ui;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use typed_builder::TypedBuilder;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::GestureArea;
use crate::{controls::*, DataHolder};
use fui_core::*;

// ============================================================================
// Attached Values for Menu State Tracking
// ============================================================================

pub struct MenuData {
    pub active_menu_id: Property<i32>,
    pub top_level_triggers: Rc<RefCell<Vec<Weak<dyn ControlObject>>>>,
    pub menu_id_counter: Cell<i32>,
    pub last_active_time: Cell<u64>,
}

pub struct ActiveMenu;

impl TypeMapKey for ActiveMenu {
    type Value = Rc<MenuData>;
}

// TODO: move it to attached value somehow
thread_local! {
    static MENU_DATA: Rc<MenuData> = Rc::new(MenuData {
            active_menu_id: Property::new(0i32),
            top_level_triggers: Rc::new(RefCell::new(Vec::new())),
            menu_id_counter: Cell::new(1),
            last_active_time: Cell::new(0),
        });
}

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
        /*// Inicjalizujemy współdzielony stan dla całego paska menu
            let menu_data = Rc::new(MenuData {
                active_menu_id: Property::new(0i32),
                top_level_triggers: Rc::new(RefCell::new(Vec::new())),
                menu_id_counter: Cell::new(1),
                last_active_time: Cell::new(0),
        });*/
        let menu_data = MENU_DATA.with(|menu_data| menu_data.clone());

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
    // Pobieramy współdzielony stan MenuBar
    //let menu_data = context.get_inherited_value::<ActiveMenu>();
    let menu_data = Some(MENU_DATA.with(|menu_data| menu_data.clone()));

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

    let children: Vec<_> = context.children.into_iter().collect();
    if children.is_empty() {
        return ui!(Text { text: "" });
    }

    let trigger = children.first().unwrap().clone();
    let background_property = Property::new(Color::rgba(0.0, 0.0, 0.0, 0.0));

    // listen for changes of the active menu, so the old one can close
    let mut subscription = None;
    if is_top_level {
        if let Some(md) = &menu_data {
            let is_open_prop_clone = is_open_prop.clone();
            let background_property_clone = background_property.clone();
            let my_id = my_menu_id;

            subscription = Some(md.active_menu_id.on_changed(move |new_active_id: i32| {
                // Jeśli aktywowane zostało inne menu, a my nadal jesteśmy otwarci - zamykamy się
                if new_active_id != my_id && is_open_prop_clone.get() {
                    is_open_prop_clone.set(false);
                    background_property_clone.set(Color::rgba(0.0, 0.0, 0.0, 0.0));
                }
            }));
        }
    }

    let popup_children: Vec<_> = children.iter().skip(1).cloned().collect();
    let has_popup_content = !popup_children.is_empty();

    // Rejestrujemy ten trigger w głównym stanie paska menu
    if is_top_level {
        if let Some(md) = &menu_data {
            md.top_level_triggers
                .borrow_mut()
                .push(Rc::downgrade(&trigger));
        }
    }

    let mut popup_view = Children::None;

    // Lista modyfikowalna po utworzeniu Popup
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

                if let Some(md) = &menu_data {
                    if md.active_menu_id.get() == my_menu_id {
                        md.active_menu_id.set(0);
                        md.last_active_time.set(get_time_ms());
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

    // Funkcja pomocnicza: buduje pełną listę uncovered tuż przed otwarciem
    let sync_uncovered_controls = {
        let menu_data = menu_data.clone();
        let uncovered_controls = uncovered_controls.clone();
        let popup_content_weak = popup_content_weak.clone();

        move || {
            let mut unc = uncovered_controls.borrow_mut();
            unc.clear();

            // Dodajemy WSZYSTKIE triggery top-level (również sąsiadów wyrenderowanych po nas)
            if let Some(md) = &menu_data {
                for t in md.top_level_triggers.borrow().iter() {
                    unc.push(t.clone());
                }
            }
            // Dodajemy własną zawartość (aby kliknięcie w rozwinięte menu go nie zamykało)
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

            Callback::new_sync(move |value: bool| {
                if value && !is_top_level {
                    if has_popup_content {
                        sync_uncovered_controls();
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

                    if let Some(md) = &menu_data {
                        let current_active = md.active_menu_id.get();
                        let recently_closed = (get_time_ms() - md.last_active_time.get()) < 150;

                        if current_active > 0 || recently_closed {
                            is_active = true;
                            if value && !is_open_prop.get() && has_popup_content {
                                sync_uncovered_controls();
                                is_open_prop.set(true);
                                md.active_menu_id.set(my_menu_id);
                            }
                        }
                    }

                    background_property.set(if is_open_prop.get() || (value && is_active) {
                        Color::rgba(0.0, 0.0, 0.0, 0.8)
                    } else if value {
                        Color::rgba(0.0, 0.0, 0.0, 0.1)
                    } else {
                        Color::rgba(0.0, 0.0, 0.0, 0.0)
                    });
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
                    if is_top_level {
                        let currently_open = is_open_prop.get();
                        if currently_open {
                            is_open_prop.set(false);
                            background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.0));
                            if let Some(md) = &menu_data {
                                if md.active_menu_id.get() == my_menu_id {
                                    md.active_menu_id.set(0);
                                    md.last_active_time.set(get_time_ms());
                                }
                            }
                        } else {
                            sync_uncovered_controls();
                            is_open_prop.set(true);
                            background_property.set(Color::rgba(0.0, 0.0, 0.0, 0.8));
                            if let Some(md) = &menu_data {
                                md.active_menu_id.set(my_menu_id);
                            }
                        }
                    } else {
                        let currently_open = is_open_prop.get();
                        if !currently_open {
                            sync_uncovered_controls();
                        }
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
                attached_values: context.attached_values,
                children: Children::SingleStatic(content),
            },
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
