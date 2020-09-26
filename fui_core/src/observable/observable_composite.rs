use crate::{Event, EventSubscription, ObservableChangedEventArgs, ObservableCollection};
use std::cell::RefCell;
use std::rc::Rc;

///
/// ObservableComposite.
///
/// This is a composite collection of any number of observable collections.
///
pub struct ObservableComposite<T: 'static + Clone> {
    sources: Vec<Box<dyn ObservableCollection<T>>>,
    lengths: Rc<RefCell<Vec<usize>>>,

    changed_event: Rc<RefCell<Event<ObservableChangedEventArgs<T>>>>,
    _source_changed_event_subscriptions: Vec<EventSubscription>,
}

impl<T: 'static + Clone> ObservableComposite<T> {
    pub fn from(sources: Vec<Box<dyn ObservableCollection<T>>>) -> Self {
        let changed_event = Rc::new(RefCell::new(Event::new()));
        let mut source_changed_event_subscriptions = Vec::new();

        let lengths_rc = Rc::new(RefCell::new(Vec::with_capacity(sources.len())));

        for (source_index, source) in sources.iter().enumerate() {
            // store source length
            lengths_rc.borrow_mut().push(source.len());

            let lengths_clone = lengths_rc.clone();
            let changed_event_clone = changed_event.clone();
            let handler = Box::new(move |changed_args| {
                // calculate offset, which is sum of length of all previous sources
                let offset: usize = lengths_clone.borrow().iter().take(source_index).sum();

                // apply offset to event args and update lengths collection
                let updated_args = match changed_args {
                    ObservableChangedEventArgs::Insert { index, value } => {
                        lengths_clone.borrow_mut()[source_index] += 1;
                        ObservableChangedEventArgs::Insert {
                            index: offset + index,
                            value,
                        }
                    }
                    ObservableChangedEventArgs::Remove { index } => {
                        let mut lengths = lengths_clone.borrow_mut();
                        if lengths[source_index] > 0 {
                            lengths[source_index] -= 1;
                        }
                        ObservableChangedEventArgs::Remove {
                            index: offset + index,
                        }
                    }
                };

                changed_event_clone.borrow().emit(updated_args)
            });

            if let Some(subscription) = source.on_changed(handler) {
                source_changed_event_subscriptions.push(subscription);
            }
        }
        ObservableComposite {
            sources,
            lengths: lengths_rc,
            changed_event,
            _source_changed_event_subscriptions: source_changed_event_subscriptions,
        }
    }
}

impl<T: 'static + Clone> ObservableCollection<T> for ObservableComposite<T> {
    fn len(&self) -> usize {
        self.sources.iter().map(|s| s.len()).sum()
    }

    fn get(&self, index: usize) -> Option<T> {
        let mut new_index = index;
        for source in self.sources.iter() {
            let len = source.len();
            if new_index < len {
                return source.get(new_index);
            } else {
                new_index -= len;
            }
        }

        None
    }

    fn on_changed(
        &self,
        f: Box<dyn Fn(ObservableChangedEventArgs<T>)>,
    ) -> Option<EventSubscription> {
        Some(
            self.changed_event
                .borrow_mut()
                .subscribe(move |args| f(args)),
        )
    }
}
