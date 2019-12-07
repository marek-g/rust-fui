use std::cell::RefCell;
use std::cell::RefMut;
use std::ops::Index;
use std::rc::Rc;

use crate::control_object::ControlObject;
use crate::observable::Event;
use crate::observable::EventSubscription;
use crate::observable::ObservableChangedEventArgs;
use crate::observable::ObservableVec;
use crate::view::{RcView, ViewContext};

#[derive(Clone)]
pub enum ChildrenSourceChangedEventArgs {
    Insert(Rc<RefCell<dyn ControlObject>>),
    Remove(Rc<RefCell<dyn ControlObject>>),
}

pub trait ChildrenSource {
    fn len(&self) -> usize;
    fn index(&self, index: usize) -> Rc<RefCell<dyn ControlObject>>;
    fn get_changed_event(&self) -> Option<RefMut<'_, Event<ChildrenSourceChangedEventArgs>>>;
}

pub struct ChildrenSourceIterator<'a> {
    source: &'a dyn ChildrenSource,
    pos: usize,
    len: usize,
}

impl<'a> Iterator for ChildrenSourceIterator<'a> {
    type Item = Rc<RefCell<dyn ControlObject>>;

    fn next(&mut self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if self.pos < self.len {
            self.pos += 1;
            Some(self.source.index(self.pos - 1))
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for ChildrenSourceIterator<'a> {
    fn next_back(&mut self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if self.len > self.pos {
            self.len -= 1;
            Some(self.source.index(self.len))
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a dyn ChildrenSource {
    type Item = Rc<RefCell<dyn ControlObject>>;
    type IntoIter = ChildrenSourceIterator<'a>;

    fn into_iter(self) -> ChildrenSourceIterator<'a> {
        ChildrenSourceIterator {
            source: self,
            pos: 0,
            len: self.len(),
        }
    }
}

///
/// StaticChildrenSource.
///
pub struct StaticChildrenSource {
    children: Vec<Rc<RefCell<dyn ControlObject>>>,
}

impl StaticChildrenSource {
    pub fn new(children: Vec<Rc<RefCell<dyn ControlObject>>>) -> Self {
        StaticChildrenSource { children }
    }
}

impl ChildrenSource for StaticChildrenSource {
    fn len(&self) -> usize {
        self.children.len()
    }

    fn index(&self, index: usize) -> Rc<RefCell<dyn ControlObject>> {
        self.children.index(index).clone()
    }

    fn get_changed_event(&self) -> Option<RefMut<'_, Event<ChildrenSourceChangedEventArgs>>> {
        None
    }
}

///
/// DynamicChildrenSource.
///
pub struct DynamicChildrenSource {
    children: Rc<RefCell<Vec<Rc<RefCell<dyn ControlObject>>>>>,
    changed_event: Rc<RefCell<Event<ChildrenSourceChangedEventArgs>>>,
    _children_changed_event_subscription: EventSubscription,
}

impl DynamicChildrenSource {
    pub fn new<T>(children: &ObservableVec<Rc<RefCell<T>>>) -> Self
    where
        T: RcView,
    {
        let children_vec = children
            .into_iter()
            .map(|vm| RcView::to_view(&vm, ViewContext::empty()))
            .collect();

        let children_rc = Rc::new(RefCell::new(children_vec));
        let changed_event_rc = Rc::new(RefCell::new(Event::new()));

        let children_rc_clone = children_rc.clone();
        let changed_event_rc_clone = changed_event_rc.clone();
        let event_subscription =
            children
                .get_changed_event()
                .subscribe(move |changed_args| match changed_args {
                    ObservableChangedEventArgs::Insert { index, value } => {
                        let mut vec: RefMut<'_, Vec<Rc<RefCell<dyn ControlObject>>>> =
                            children_rc_clone.borrow_mut();
                        let control = RcView::to_view(&value, ViewContext::empty());
                        let control_clone = control.clone();
                        vec.insert(index, control);

                        changed_event_rc_clone
                            .borrow()
                            .emit(ChildrenSourceChangedEventArgs::Insert(control_clone));
                    }
                    ObservableChangedEventArgs::Remove {
                        index,
                        value: _value,
                    } => {
                        let mut vec: RefMut<'_, Vec<Rc<RefCell<dyn ControlObject>>>> =
                            children_rc_clone.borrow_mut();
                        let control = vec.remove(index);

                        changed_event_rc_clone
                            .borrow()
                            .emit(ChildrenSourceChangedEventArgs::Remove(control));
                    }
                });

        DynamicChildrenSource {
            children: children_rc,
            changed_event: changed_event_rc,
            _children_changed_event_subscription: event_subscription,
        }
    }
}

impl ChildrenSource for DynamicChildrenSource {
    fn len(&self) -> usize {
        self.children.borrow().len()
    }

    fn index(&self, index: usize) -> Rc<RefCell<dyn ControlObject>> {
        self.children.borrow().index(index).clone()
    }

    fn get_changed_event(&self) -> Option<RefMut<'_, Event<ChildrenSourceChangedEventArgs>>> {
        Some(self.changed_event.borrow_mut())
    }
}

///
/// AggregatedChildrenSource.
///
pub struct AggregatedChildrenSource {
    sources: Vec<Box<dyn ChildrenSource>>,
    changed_event: Option<Rc<RefCell<Event<ChildrenSourceChangedEventArgs>>>>,
    source_changed_event_subscriptions: Vec<EventSubscription>,
}

impl AggregatedChildrenSource {
    pub fn new(sources: Vec<Box<dyn ChildrenSource>>) -> Self {
        let mut changed_event = None;
        let mut source_changed_event_subscriptions = Vec::new();
        for source in &sources {
            if let Some(mut source_changed_event) = source.get_changed_event() {
                let dest_event_rc = changed_event
                    .get_or_insert_with(|| Rc::new(RefCell::new(Event::new())))
                    .clone();
                source_changed_event_subscriptions.push(
                    source_changed_event
                        .subscribe(move |changed_args| dest_event_rc.borrow().emit(changed_args)),
                );
            }
        }
        AggregatedChildrenSource {
            sources,
            changed_event,
            source_changed_event_subscriptions,
        }
    }
}

impl ChildrenSource for AggregatedChildrenSource {
    fn len(&self) -> usize {
        self.sources.iter().map(|s| s.len()).sum()
    }

    fn index(&self, index: usize) -> Rc<RefCell<dyn ControlObject>> {
        let mut new_index = index;
        for source in &self.sources {
            let len = source.len();
            if new_index < len {
                return source.index(new_index);
            } else {
                new_index -= len;
            }
        }

        panic!(format!(
            "index out of bounds: the len is {} but the index is {}",
            self.len(),
            index
        ))
    }

    fn get_changed_event(&self) -> Option<RefMut<'_, Event<ChildrenSourceChangedEventArgs>>> {
        match &self.changed_event {
            None => None,
            Some(ref changed_event) => Some(changed_event.borrow_mut()),
        }
    }
}
