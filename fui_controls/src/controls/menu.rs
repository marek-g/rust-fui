use fui_core::{ControlObject, Property, Style, ViewContext};
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

use crate::controls::*;
use crate::layout::*;
use crate::{DataHolder, GestureArea};
use fui_core::*;

pub enum MenuItem {
    Separator,
    Text {
        text: String,
        shortcut: Option<String>,
        icon: Option<Rc<RefCell<dyn ControlObject>>>,
        callback: Callback<()>,
        sub_items: Vec<MenuItem>,
    },
    Custom {
        content: Rc<RefCell<dyn ControlObject>>,
        callback: Callback<()>,
        sub_items: Vec<MenuItem>,
    },
}

impl MenuItem {
    pub fn folder(text: &str, sub_items: Vec<MenuItem>) -> Self {
        MenuItem::Text {
            text: text.into(),
            shortcut: None,
            icon: None,
            callback: Callback::empty(),
            sub_items,
        }
    }

    pub fn simple(text: &str, callback: Callback<()>) -> Self {
        MenuItem::Text {
            text: text.into(),
            shortcut: None,
            icon: None,
            callback,
            sub_items: Vec::new(),
        }
    }

    pub fn full(
        text: &str,
        shortcut: Option<String>,
        icon: Option<Rc<RefCell<dyn ControlObject>>>,
        callback: Callback<()>,
    ) -> Self {
        MenuItem::Text {
            text: text.into(),
            shortcut,
            icon,
            callback,
            sub_items: Vec::new(),
        }
    }
}

#[derive(TypedBuilder)]
pub struct Menu {
    #[builder(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    pub items: Vec<MenuItem>,
}

impl Menu {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let content: Vec<_> = self
            .items
            .into_iter()
            .map(|item| item.to_view(true))
            .collect();

        let menu = ui!(
            Border {
                border_type: BorderType::None,
                Style: Default { background_color: [1.0f32, 1.0f32, 1.0f32, 0.8f32], },

                StackPanel {
                    orientation: self.orientation,

                    content,
                }
            }
        );

        let data_holder = DataHolder { data: () };
        data_holder.to_view(
            None,
            ViewContext {
                attached_values: context.attached_values,
                children: Children::SingleStatic(menu),
            },
        )
    }
}

impl MenuItem {
    pub fn to_view(self, is_top: bool) -> Rc<RefCell<dyn ControlObject>> {
        match self {
            MenuItem::Separator => {
                let separator: Rc<RefCell<dyn ControlObject>> = ui! {
                    Text {
                        Style: Default { color: [0.0f32, 0.0f32, 0.0f32, 1.0f32] },
                        text: "---------"
                    }
                };
                separator
            }

            MenuItem::Text {
                text,
                shortcut,
                icon,
                callback,
                sub_items,
            } => {
                let mut background_property = Property::new([0.0f32, 0.0f32, 0.0f32, 0.0f32]);
                let mut foreground_property = Property::new([0.0f32, 0.0f32, 0.0f32, 1.0f32]);

                let mut on_hover_callback = Callback::empty();
                let mut background_property_clone = background_property.clone();
                let mut foreground_property_clone = foreground_property.clone();
                on_hover_callback.set(move |value| {
                    background_property_clone.set(if value {
                        [0.0f32, 0.0f32, 0.0f32, 0.8f32]
                    } else {
                        [0.0f32, 0.0f32, 0.0f32, 0.0f32]
                    });
                    foreground_property_clone.set(if value {
                        [1.0f32, 1.0f32, 0.0f32, 1.0f32]
                    } else {
                        [0.0f32, 0.0f32, 0.0f32, 1.0f32]
                    });
                });

                let title_content: Rc<RefCell<dyn ControlObject>> = if is_top {
                    ui!(Text {
                        Row: 0,
                        Column: 1,
                        Margin: Thickness::new(5.0f32, 0.0f32, 5.0f32, 0.0f32),
                        Style: Dynamic {
                            color: foreground_property
                        },
                        text: text
                    })
                } else {
                    ui!(
                        Grid {
                            columns: 4,
                            widths: vec![
                                (0, Length::Exact(25.0f32)),
                                (1, Length::Fill(1.0f32)),
                                (2, Length::Auto),
                                (3, Length::Exact(25.0f32)),
                            ],

                            Text {
                                Row: 0, Column: 1,
                                HorizontalAlignment: Alignment::Start,
                                Style: Dynamic { color: foreground_property.clone() },

                                text: text
                            },

                            Text {
                                Row: 0, Column: 3,
                                Style: Dynamic { color: foreground_property },
                                text: if sub_items.len() > 0 { ">" } else { "" },
                            }
                        }
                    )
                };

                let title = ui!(
                    GestureArea {
                        hover_change: on_hover_callback,

                        Border {
                            border_type: BorderType::None,
                            Style: Default { background_color: background_property },

                            title_content,
                        }
                    }
                );

                if sub_items.len() == 0 {
                    return title;
                }

                let mut is_open_prop = Property::new(false);
                let mut is_open_prop_clone = is_open_prop.clone();
                /*let mut tap_down_callback: Callback<()> = Callback::empty();
                tap_down_callback.set(move |_| {
                    is_open_prop_clone.set(true);
                });*/
                let mut hover_change_callback: Callback<bool> = Callback::empty();
                hover_change_callback.set(move |value| {
                    is_open_prop_clone.set(true);
                });

                let sub_content: Vec<_> = sub_items
                    .into_iter()
                    .map(|item| item.to_view(false))
                    .collect();

                let popup_placement = if is_top {
                    PopupPlacement::BelowOrAboveParent
                } else {
                    PopupPlacement::LeftOrRightParent
                };

                ui!(
                    GestureArea {
                        //tap_down: tap_down_callback,
                        hover_change: hover_change_callback,

                        title,

                        Popup {
                            is_open: is_open_prop,
                            placement: popup_placement,
                            auto_hide: PopupAutoHide::Menu,

                            Border {
                                Style: Default { background_color: [1.0f32, 1.0f32, 1.0f32, 0.8f32], },

                                Grid {
                                    columns: 1,
                                    default_width: Length::Fill(1.0f32),
                                    default_height: Length::Auto,

                                    sub_content
                                }
                            }
                        }
                    }
                )
            }

            MenuItem::Custom {
                content,
                callback,
                sub_items,
            } => content,
        }
    }
}
