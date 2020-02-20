use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::children_source::*;
use crate::control_context::*;
use crate::control_object::*;
use crate::observable::*;
use crate::style::*;
use crate::view::ViewContext;

pub enum HitTestResult {
    Nothing,
    Current,
    Child(Rc<RefCell<dyn ControlObject>>),
}

pub struct Control<D> {
    pub data: D,
    pub style: Box<dyn Style<D>>,
    pub context: ControlContext,
}

impl<D: 'static> Control<D> {
    pub fn new<S: 'static + Style<D>>(
        data: D,
        style: S,
        view_context: ViewContext,
    ) -> Rc<RefCell<Self>> {
        let control_context = ControlContext {
            attached_values: view_context.attached_values,
            children: view_context.children,
            parent: None,
            is_dirty: true,
            children_collection_changed_event_subscription: None,
        };

        let control = Rc::new(RefCell::new(Control {
            data: data,
            style: Box::new(style),
            context: control_context,
        }));

        let subscription = if let Some(mut changed_event) =
            control.borrow_mut().get_context_mut().get_children().get_changed_event()
        {
            let control_clone = control.clone();
            Some(changed_event.subscribe(move |changed_args| {
                if let ChildrenSourceChangedEventArgs::Insert(child) = changed_args {
                    let control_weak =
                        Rc::downgrade(&control_clone) as Weak<RefCell<dyn ControlObject>>;
                    child.borrow_mut().set_parent(control_weak);
                    let mut control_mut = control_clone.borrow_mut();
                    let (data, style) = control_mut.get_data_and_style_mut();
                    style.setup_dirty_watching(data, &control_clone);
                }
                control_clone.borrow_mut().get_context_mut().set_is_dirty(true);
            }))
        } else {
            None
        };
        control
            .borrow_mut()
            .context
            .children_collection_changed_event_subscription = subscription;

        for child in control.borrow_mut().get_context_mut().get_children().into_iter() {
            let control_weak = Rc::downgrade(&control) as Weak<RefCell<dyn ControlObject>>;
            child.borrow_mut().set_parent(control_weak);
        }

        {
            let mut control_mut = control.borrow_mut();
            let (data, style) = control_mut.get_data_and_style_mut();
            style.setup_dirty_watching(data, &control);
        }

        control
    }

    pub fn get_context(&self) -> &ControlContext {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut ControlContext {
        &mut self.context
    }

    fn get_data_and_style_mut(&mut self) -> (&mut D, &mut Box<dyn Style<D>>) {
        (&mut self.data, &mut self.style)
    }
}

pub trait ControlExtensions<D> {
    fn with_vm<V: 'static, F: 'static + Fn(&Rc<RefCell<V>>, &mut Control<D>)>(
        self,
        vm: &Rc<RefCell<V>>,
        f: F,
    ) -> Self;

    fn with_binding<V: 'static, F: 'static + Fn(&mut V, &mut Control<D>) -> EventSubscription>(
        self,
        bindings: &mut Vec<EventSubscription>,
        vm: &Rc<RefCell<V>>,
        f: F,
    ) -> Rc<RefCell<Control<D>>>;
}

impl<D: 'static> ControlExtensions<D> for Rc<RefCell<Control<D>>> {
    fn with_vm<V: 'static, F: 'static + Fn(&Rc<RefCell<V>>, &mut Control<D>)>(
        self,
        vm: &Rc<RefCell<V>>,
        f: F,
    ) -> Rc<RefCell<Control<D>>> {
        {
            let mut control = self.borrow_mut();
            f(&vm, &mut control);
        }
        self
    }

    fn with_binding<V: 'static, F: 'static + Fn(&mut V, &mut Control<D>) -> EventSubscription>(
        self,
        bindings: &mut Vec<EventSubscription>,
        vm: &Rc<RefCell<V>>,
        f: F,
    ) -> Rc<RefCell<Control<D>>> {
        {
            let mut vm = vm.borrow_mut();
            let mut control = self.borrow_mut();
            let binding = f(&mut vm, &mut control);
            bindings.push(binding);
        }
        self
    }
}
