use crate::{Event, ObservableCollection, Subscription, VecDiff};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

///
/// ObservableCollectionMap.
///
pub struct ObservableCollectionMap<TDst: 'static + Clone> {
    items: Rc<RefCell<Vec<TDst>>>,
    changed_event: Rc<RefCell<Event<VecDiff<TDst>>>>,
    _items_changed_event_subscription: Option<Subscription>,
}

impl<T: 'static + Clone> ObservableCollection<T> for ObservableCollectionMap<T> {
    fn len(&self) -> usize {
        self.items.borrow().len()
    }

    fn get(&self, index: usize) -> Option<T> {
        self.items
            .borrow()
            .as_slice()
            .get(index)
            .map(|el| el.clone())
    }

    fn on_changed(&self, f: Box<dyn FnMut(VecDiff<T>)>) -> Option<Subscription> {
        Some(Subscription::EventSubscription(
            self.changed_event.borrow_mut().subscribe(f),
        ))
    }
}

pub trait ObservableCollectionExt<T: 'static + Clone> {
    fn map<TDst, F>(&self, f: F) -> ObservableCollectionMap<TDst>
    where
        TDst: 'static + Clone,
        F: 'static + Fn(&T) -> TDst;
}

impl<T: 'static + Clone> ObservableCollectionExt<T> for dyn ObservableCollection<T> {
    /// Map creates new observable collection.
    ///
    /// It keeps mapped copy of every item.
    ///
    /// The only connection between it and original observable collection
    /// is by subscribing on the `on_changed` event of the source collection,
    /// so we don't have to keep implicit reference to the source collection.
    /// The `on_change` event of source collection keeps a weak reference to our handler.
    fn map<TDst, F>(&self, f: F) -> ObservableCollectionMap<TDst>
    where
        TDst: 'static + Clone,
        F: 'static + Fn(&T) -> TDst,
    {
        let items_vec = self.into_iter().map(|item| f(&item)).collect();
        let items_rc = Rc::new(RefCell::new(items_vec));

        let changed_event_rc = Rc::new(RefCell::new(Event::new()));

        let handler = Box::new({
            let items_rc = items_rc.clone();
            let changed_event_rc = changed_event_rc.clone();
            move |changed_args| match changed_args {
                VecDiff::Clear {} => {
                    (items_rc.borrow_mut() as RefMut<'_, Vec<TDst>>).clear();
                    changed_event_rc.borrow().emit(VecDiff::Clear {});
                }

                VecDiff::InsertAt { index, value } => {
                    let mut vec: RefMut<'_, Vec<TDst>> = items_rc.borrow_mut();
                    let new_item = f(&value);
                    let new_item_clone = new_item.clone();
                    vec.insert(index, new_item);

                    changed_event_rc.borrow().emit(VecDiff::InsertAt {
                        index,
                        value: new_item_clone,
                    });
                }

                VecDiff::RemoveAt { index } => {
                    items_rc.borrow_mut().remove(index);
                    changed_event_rc.borrow().emit(VecDiff::RemoveAt { index });
                }
            }
        });
        let event_subscription = self.on_changed(handler);

        ObservableCollectionMap {
            items: items_rc,
            changed_event: changed_event_rc,
            _items_changed_event_subscription: event_subscription,
        }
    }
}
