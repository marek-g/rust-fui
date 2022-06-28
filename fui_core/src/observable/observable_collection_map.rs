use crate::{Event, EventSubscription, ObservableCollection};
use futures_signals::signal_vec::VecDiff;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

///
/// ObservableCollectionMap.
///
pub struct ObservableCollectionMap<TDst: 'static + Clone> {
    items: Rc<RefCell<Vec<TDst>>>,
    changed_event: Rc<RefCell<Event<VecDiff<TDst>>>>,
    _items_changed_event_subscription: Option<Box<dyn Drop>>,
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

    fn on_changed(&self, f: Box<dyn Fn(VecDiff<T>)>) -> Option<Box<dyn Drop>> {
        Some(Box::new(self.changed_event.borrow_mut().subscribe(f)))
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

        let items_rc_clone = items_rc.clone();
        let changed_event_rc_clone = changed_event_rc.clone();
        let handler = Box::new(move |changed_args| match changed_args {
            VecDiff::InsertAt { index, value } => {
                let mut vec: RefMut<'_, Vec<TDst>> = items_rc_clone.borrow_mut();
                let new_item = f(&value);
                let new_item_clone = new_item.clone();
                vec.insert(index, new_item);

                changed_event_rc_clone.borrow().emit(VecDiff::InsertAt {
                    index,
                    value: new_item_clone,
                });
            }

            VecDiff::RemoveAt { index } => {
                let mut vec: RefMut<'_, Vec<TDst>> = items_rc_clone.borrow_mut();
                vec.remove(index);

                changed_event_rc_clone
                    .borrow()
                    .emit(VecDiff::RemoveAt { index });
            }

            // TODO:
            _ => {}
        });
        let event_subscription = self.on_changed(handler);

        ObservableCollectionMap {
            items: items_rc,
            changed_event: changed_event_rc,
            _items_changed_event_subscription: event_subscription,
        }
    }
}
