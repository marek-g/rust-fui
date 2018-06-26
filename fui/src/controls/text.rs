use std::cell::{ RefCell, RefMut };
use std::rc::{ Rc, Weak };

use control::*;
use control_object::*;
use common::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use events::*;
use Property;

pub struct TextProperties {
    pub text: Property<String>,
}

pub struct TextData {
    pub properties: TextProperties,
    pub parent: Option<Weak<RefCell<ControlObject>>>,
    is_dirty: bool,
}

pub struct Text {
    pub data: TextData,
    style: Box<Style<TextData>>,   
}

impl Text {
    pub fn new(text: String) -> Rc<RefCell<Self>> {
        let text = Rc::new(RefCell::new(Text {
            data: TextData {
                properties: TextProperties { text: Property::new(text) },
                parent: None,
                is_dirty: true,
            },
            style: Box::new(TextDefaultStyle {
                rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
                font_name: "OpenSans-Regular.ttf",
                font_size: 20u8
            }),
        }));

        let weak_text = Rc::downgrade(&text);
        text.borrow_mut().data.properties.text.on_changed_without_subscription(move |_| {
            weak_text.upgrade().map(|text| (text.borrow_mut() as RefMut<Control<Data = TextData>>).set_is_dirty(true));
        });

        text
    }
}

impl Control for Text {
    type Data = TextData;

    fn get_data(&self) -> &Self::Data {
        &self.data
    }

    fn get_style(&self) -> &Box<Style<Self::Data>> {
        &self.style
    }

    fn get_style_and_data_mut(&mut self) -> (&mut Box<Style<Self::Data>>, &Self::Data) {
        (&mut self.style, &mut self.data)
    }

    fn is_dirty(&self) -> bool {
        self.data.is_dirty
    }
    
    fn set_is_dirty(&mut self, is_dirty: bool) {
        self.data.is_dirty = is_dirty;
        if is_dirty {
            if let Some(ref parent) = (self as &mut Control<Data = TextData>).get_parent() {
                parent.borrow_mut().set_is_dirty(is_dirty)
            }
        }
    }

    fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>> {
        if let Some(ref test) = self.data.parent {
            test.upgrade()
        } else {
            None
        }
    }

    fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>) {
        self.data.parent = Some(parent);
    }

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        Vec::new()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        true
    }
}


//
// Text Default Style
//

pub struct TextDefaultStyle {
    rect: Rect,
    font_name: &'static str,
    font_size: u8,
}

impl Style<TextData> for TextDefaultStyle {
    fn get_preferred_size(&self, data: &TextData, drawing_context: &mut DrawingContext, _size: Size) -> Size {
        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &data.properties.text.get());
        Size::new(text_width as f32, text_height as f32)
    }

    fn set_rect(&mut self, _data: &TextData, rect: Rect) {    
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &TextData, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) { HitTestResult::Current } else { HitTestResult::Nothing }
    }

    fn to_primitives(&self, data: &TextData,
        drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &data.properties.text.get());

        vec.push(Primitive::Text {
            resource_key: self.font_name.to_string(),
            color: [1.0, 1.0, 1.0, 1.0],
            position: UserPixelPoint::new(x + (width - text_width as f32) / 2.0, y + (height - text_height as f32) / 2.0),
            size: self.font_size as u16,
            text: data.properties.text.get(),
        });

        vec
    }
}
