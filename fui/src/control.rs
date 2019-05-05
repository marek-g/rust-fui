use std::cell::{ RefCell, RefMut };
use std::rc::{ Rc, Weak };

use common::*;
use control_object::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use events::*;
use observable::*;

pub enum HitTestResult {
    Nothing,
    Current,
    Child(Rc<RefCell<ControlObject>>)
}

pub trait Style<D> {
    fn setup_dirty_watching(&mut self, data: &mut D, control: &Rc<RefCell<Control<D>>>);

    fn get_preferred_size(&self, data: &D, drawing_context: &mut DrawingContext, size: Size) -> Size;
    fn set_rect(&mut self, data: &D, rect: Rect);
    fn get_rect(&self) -> Rect;

    fn hit_test(&self, data: &D, point: Point) -> HitTestResult;

    fn to_primitives(&self, data: &D,
        drawing_context: &mut DrawingContext) -> Vec<Primitive>;
}

pub trait ControlBehaviour {
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>>;
    fn handle_event(&mut self, event: ControlEvent);
}

pub struct Control<D> {
    pub data: D,
    pub style: Box<Style<D>>,

    parent: Option<Weak<RefCell<ControlObject>>>,
    is_dirty: bool,
}

impl<D: 'static> Control<D> where Control<D>: ControlBehaviour {
    pub fn new<S: 'static + Style<D>>(style: S, data: D) -> Rc<RefCell<Self>> {
        let control = Rc::new(RefCell::new(Control {
            data: data,
            style: Box::new(style),
            parent: None,
            is_dirty: true,
        }));

        for child in (control.borrow_mut() as RefMut<ControlBehaviour>).get_children().iter() {
            let control_weak = Rc::downgrade(&control) as Weak<RefCell<ControlObject>>;
            child.borrow_mut().set_parent(control_weak);
        }

        {
            let mut control_mut = control.borrow_mut();
            let (data, style) = control_mut.get_data_and_style_mut();
            style.setup_dirty_watching(data, &control);
        }

        control
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>> {
        if let Some(ref test) = self.parent {
            test.upgrade()
        } else {
            None
        }
    }

    pub fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>) {
        self.parent = Some(parent);
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    
    pub fn set_is_dirty(&mut self, is_dirty: bool) {
        self.is_dirty = is_dirty;
        if is_dirty {
            if let Some(ref parent) = self.get_parent() {
                parent.borrow_mut().set_is_dirty(is_dirty)
            }
        }
    }

    fn get_data_and_style_mut(&mut self) -> (&mut D, &mut Box<Style<D>>) {
        (&mut self.data, &mut self.style)
    }
}

pub trait PropertyDirtyExtension<D> {
    fn dirty_watching(&mut self, control: &Rc<RefCell<Control<D>>>) -> EventSubscription;
}

impl<D: 'static, T> PropertyDirtyExtension<D> for Property<T>
    where Control<D>: ControlBehaviour,
    T: 'static + Clone + PartialEq {
    fn dirty_watching(&mut self, control: &Rc<RefCell<Control<D>>>) -> EventSubscription {
        let weak_control = Rc::downgrade(control);
        self.on_changed(move |_| {
            weak_control.upgrade().map(|control| (control.borrow_mut() as RefMut<Control<D>>).set_is_dirty(true));
        })
    }
}

pub trait ControlExtensions<D> {
    fn with_vm<V: 'static, F: 'static + Fn(&Rc<RefCell<V>>, &mut Control<D>)>(mut self, vm: &Rc<RefCell<V>>, f: F) -> Self;

    fn with_binding<V: 'static, F: 'static + Fn(&mut V, &mut Control<D>) -> EventSubscription>(mut self,
        bindings: &mut Vec<EventSubscription>, vm: &Rc<RefCell<V>>, f: F) -> Rc<RefCell<Control<D>>>;
}

impl<D: 'static> ControlExtensions<D> for Rc<RefCell<Control<D>>> where Control<D>: ControlBehaviour {
    fn with_vm<V: 'static, F: 'static + Fn(&Rc<RefCell<V>>, &mut Control<D>)>(mut self, vm: &Rc<RefCell<V>>, f: F)
        -> Rc<RefCell<Control<D>>> {
        {
            let mut control = self.borrow_mut();
            f(&vm, &mut control);
        }
        self
    }

    fn with_binding<V: 'static, F: 'static + Fn(&mut V, &mut Control<D>) -> EventSubscription>(mut self,
        bindings: &mut Vec<EventSubscription>, vm: &Rc<RefCell<V>>, f: F) -> Rc<RefCell<Control<D>>> {
        {
            let mut vm = vm.borrow_mut();
            let mut control = self.borrow_mut();
            let binding = f(&mut vm, &mut control);
            bindings.push(binding);
        }
        self
    }
}