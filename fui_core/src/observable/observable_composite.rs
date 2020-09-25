use crate::{Event, EventSubscription, ObservableChangedEventArgs, ObservableCollection};
use std::cell::RefCell;
use std::rc::Rc;

///
/// ObservableComposite.
///
/// This is a composite collection of any number of observable collections.
///
pub struct ObservableComposite<T: 'static + Clone> {
    sources: Rc<Vec<Box<dyn ObservableCollection<T>>>>,
    changed_event: Rc<RefCell<Event<ObservableChangedEventArgs<T>>>>,
    _source_changed_event_subscriptions: Vec<EventSubscription>,
}

impl<T: 'static + Clone> ObservableComposite<T> {
    pub fn from(sources: Vec<Box<dyn ObservableCollection<T>>>) -> Self {
        let sources_rc = Rc::new(sources);

        let changed_event = Rc::new(RefCell::new(Event::new()));
        let mut source_changed_event_subscriptions = Vec::new();

        for (index, source) in sources_rc.iter().enumerate() {
            let changed_event_clone = changed_event.clone();

            let sources_weak = Rc::downgrade(&sources_rc);
            let handler = Box::new(move |changed_args| {
                // calculate offset, which is sum of length of all previous sources
                let offset = if let Some(sources) = sources_weak.upgrade() {
                    sources.iter().take(index).map(|source| source.len()).sum()
                } else {
                    0
                };

                // apply offset to event args
                let updated_args = match changed_args {
                    ObservableChangedEventArgs::Insert { index, value } => {
                        ObservableChangedEventArgs::Insert {
                            index: offset + index,
                            value,
                        }
                    }
                    ObservableChangedEventArgs::Remove { index } => {
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
            sources: sources_rc,
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
