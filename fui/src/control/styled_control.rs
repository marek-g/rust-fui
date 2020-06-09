use crate::events::ControlEvent;
use crate::drawing::Resources;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::common::*;
use crate::control::*;
use crate::observable::*;
use crate::style::*;
use crate::{DrawingContext, view::ViewContext};

use drawing::primitive::Primitive;

pub struct StyledControl<D> {
    pub data: D,
    pub style: Box<dyn Style<D>>,
    pub control_context: ControlContext,
}

impl<D: 'static> StyledControl<D> {
    pub fn new(
        data: D,
        style: Box<dyn Style<D>>,
        view_context: ViewContext,
    ) -> Rc<RefCell<Self>> {
        let control = Rc::new(RefCell::new(StyledControl {
            data,
            style,
            control_context: ControlContext::new(view_context),
        }));

        let control_clone = control.clone();
        let handler = Box::new(
            move |changed_args: ObservableChangedEventArgs<Rc<RefCell<dyn ControlObject>>>| {
                if let ObservableChangedEventArgs::Insert { index: _, value: child } = changed_args {
                    let control_weak =
                        Rc::downgrade(&control_clone) as Weak<RefCell<dyn ControlObject>>;
                    child
                        .borrow_mut()
                        .get_context_mut()
                        .set_parent(control_weak);
                    let mut control_mut = control_clone.borrow_mut();
                    let (data, style) = control_mut.get_data_and_style_mut();
                    style.setup_dirty_watching(data, &control_clone);
                }
                control_clone
                    .borrow_mut()
                    .get_context_mut()
                    .set_is_dirty(true);
            }
        );
        let subscription = control
            .borrow_mut()
            .get_context_mut()
            .get_children()
            .on_changed(handler);

        control
            .borrow_mut()
            .get_context_mut()
            .set_children_collection_changed_event_subscription(subscription);

        for child in control
            .borrow_mut()
            .get_context_mut()
            .get_children()
            .into_iter()
        {
            let control_weak = Rc::downgrade(&control) as Weak<RefCell<dyn ControlObject>>;
            child
                .borrow_mut()
                .get_context_mut()
                .set_parent(control_weak);
        }

        {
            let mut control_mut = control.borrow_mut();
            let (data, style) = control_mut.get_data_and_style_mut();
            style.setup_dirty_watching(data, &control);
        }

        control
    }

    pub fn get_context(&self) -> &ControlContext {
        &self.control_context
    }

    pub fn get_context_mut(&mut self) -> &mut ControlContext {
        &mut self.control_context
    }

    fn get_data_and_style_mut(&mut self) -> (&mut D, &mut Box<dyn Style<D>>) {
        (&mut self.data, &mut self.style)
    }
}

pub trait ControlExtensions<D> {
    fn with_vm<V: 'static, F: 'static + Fn(&Rc<RefCell<V>>, &mut StyledControl<D>)>(
        self,
        vm: &Rc<RefCell<V>>,
        f: F,
    ) -> Self;

    fn with_binding<
        V: 'static,
        F: 'static + Fn(&mut V, &mut StyledControl<D>) -> EventSubscription,
    >(
        self,
        bindings: &mut Vec<EventSubscription>,
        vm: &Rc<RefCell<V>>,
        f: F,
    ) -> Rc<RefCell<StyledControl<D>>>;
}

impl<D: 'static> ControlExtensions<D> for Rc<RefCell<StyledControl<D>>> {
    fn with_vm<V: 'static, F: 'static + Fn(&Rc<RefCell<V>>, &mut StyledControl<D>)>(
        self,
        vm: &Rc<RefCell<V>>,
        f: F,
    ) -> Rc<RefCell<StyledControl<D>>> {
        {
            let mut control = self.borrow_mut();
            f(&vm, &mut control);
        }
        self
    }

    fn with_binding<
        V: 'static,
        F: 'static + Fn(&mut V, &mut StyledControl<D>) -> EventSubscription,
    >(
        self,
        bindings: &mut Vec<EventSubscription>,
        vm: &Rc<RefCell<V>>,
        f: F,
    ) -> Rc<RefCell<StyledControl<D>>> {
        {
            let mut vm = vm.borrow_mut();
            let mut control = self.borrow_mut();
            let binding = f(&mut vm, &mut control);
            bindings.push(binding);
        }
        self
    }
}

impl<D: 'static> ControlObject for StyledControl<D> {
    fn get_context(&self) -> &ControlContext {
        self.get_context()
    }

    fn get_context_mut(&mut self) -> &mut ControlContext {
        self.get_context_mut()
    }
}

impl<D: 'static> ControlBehavior for StyledControl<D> {
    fn handle_event(&mut self, drawing_context: &mut dyn DrawingContext, event: ControlEvent) {
        self.style
            .handle_event(&mut self.data, &mut self.control_context, drawing_context, event)
    }

    fn measure(&mut self, drawing_context: &mut dyn DrawingContext, size: Size) {
        self.style
            .measure(&mut self.data, &mut self.control_context, drawing_context, size)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.style.set_rect(&mut self.data, &mut self.control_context, rect);
    }

    fn get_rect(&self) -> Rect {
        self.style.get_rect(&self.control_context)
    }

    fn hit_test(&self, point: Point) -> HitTestResult {
        self.style.hit_test(&self.data, &self.control_context, point)
    }

    fn to_primitives(&self, drawing_context: &mut dyn DrawingContext) -> (Vec<Primitive>, Vec<Primitive>) {
        self.style
            .to_primitives(&self.data, &self.control_context, drawing_context)
    }
}
