use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use control_object::*;
use observable::*;
use style::*;

pub enum HitTestResult {
    Nothing,
    Current,
    Child(Rc<RefCell<ControlObject>>)
}

pub struct Control<D> {
    pub data: D,
    pub style: Box<Style<D>>,
    pub children: Vec<Rc<RefCell<ControlObject>>>,

    parent: Option<Weak<RefCell<ControlObject>>>,
    is_dirty: bool,
}

impl<D: 'static> Control<D> {
    pub fn new<S: 'static + Style<D>>(data: D, style: S, children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<Self>> {
        let control = Rc::new(RefCell::new(Control {
            data: data,
            style: Box::new(style),
            children: children,
            parent: None,
            is_dirty: true,
        }));

        for child in control.borrow_mut().get_children().iter() {
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

    pub fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        self.children.clone()
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

pub trait ControlExtensions<D> {
    fn with_vm<V: 'static, F: 'static + Fn(&Rc<RefCell<V>>, &mut Control<D>)>(self, vm: &Rc<RefCell<V>>, f: F) -> Self;

    fn with_binding<V: 'static, F: 'static + Fn(&mut V, &mut Control<D>) -> EventSubscription>(self,
        bindings: &mut Vec<EventSubscription>, vm: &Rc<RefCell<V>>, f: F) -> Rc<RefCell<Control<D>>>;
}

impl<D: 'static> ControlExtensions<D> for Rc<RefCell<Control<D>>> {
    fn with_vm<V: 'static, F: 'static + Fn(&Rc<RefCell<V>>, &mut Control<D>)>(self, vm: &Rc<RefCell<V>>, f: F)
        -> Rc<RefCell<Control<D>>> {
        {
            let mut control = self.borrow_mut();
            f(&vm, &mut control);
        }
        self
    }

    fn with_binding<V: 'static, F: 'static + Fn(&mut V, &mut Control<D>) -> EventSubscription>(self,
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