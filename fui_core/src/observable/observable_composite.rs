use crate::{Event, ObservableCollection, Subscription, VecDiff};
use std::cell::RefCell;
use std::rc::Rc;

///
/// ObservableComposite.
///
/// This is a composite collection of any number of observable collections.
///
pub struct ObservableComposite<T: 'static + Clone> {
    sources: Vec<Box<dyn ObservableCollection<T>>>,
    changed_event: Rc<RefCell<Event<VecDiff<T>>>>,
    _source_changed_event_subscriptions: Vec<Subscription>,
}

impl<T: 'static + Clone> ObservableComposite<T> {
    pub fn from(sources: Vec<Box<dyn ObservableCollection<T>>>) -> Self {
        let changed_event = Rc::new(RefCell::new(Event::new()));
        let mut source_changed_event_subscriptions = Vec::new();

        // The collection that keeps track of the lengths of all source collections.
        //
        // Please note that we need to manually update lengths. We cannot just look
        // directly into `sources` to get them. Events in `fui` are asynchronous, so
        // we could get different results (different index values in generated events).
        // For example in a case of quickly adding two items one by one, we receive
        // the event of adding the first one later (when both are already added).
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
                match changed_args {
                    VecDiff::Clear {} => {
                        let number_of_sources = lengths_clone.borrow().len();
                        let mut lengths = lengths_clone.borrow_mut();
                        let other_collections_size: usize = (0..number_of_sources)
                            .filter(|i| *i != source_index)
                            .map(|i| lengths[i])
                            .sum();
                        if other_collections_size == 0 {
                            changed_event_clone.borrow().emit(VecDiff::Clear {});
                        } else {
                            for i in (0..lengths[source_index]).rev() {
                                changed_event_clone
                                    .borrow()
                                    .emit(VecDiff::RemoveAt { index: offset + i });
                            }
                            lengths[source_index] = 0;
                        }
                    }

                    VecDiff::InsertAt { index, value } => {
                        lengths_clone.borrow_mut()[source_index] += 1;
                        changed_event_clone.borrow().emit(VecDiff::InsertAt {
                            index: offset + index,
                            value,
                        });
                    }

                    VecDiff::RemoveAt { index } => {
                        let mut lengths = lengths_clone.borrow_mut();
                        if lengths[source_index] > 0 {
                            lengths[source_index] -= 1;
                        }

                        changed_event_clone.borrow().emit(VecDiff::RemoveAt {
                            index: offset + index,
                        });
                    }
                };
            });

            if let Some(subscription) = source.on_changed(handler) {
                source_changed_event_subscriptions.push(subscription);
            }
        }
        ObservableComposite {
            sources,
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

    fn on_changed(&self, f: Box<dyn Fn(VecDiff<T>)>) -> Option<Subscription> {
        Some(Subscription::EventSubscription(
            self.changed_event
                .borrow_mut()
                .subscribe(move |args| f(args)),
        ))
    }
}
