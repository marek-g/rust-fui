use crate::{Event, ObservableCollection, Subscription, VecDiff};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

///
/// ObservableCollectionFlatMap.
///
pub struct ObservableCollectionFlatMap<T: 'static + Clone> {
    items: Rc<RefCell<Vec<T>>>,

    _sub_collection_data: Rc<RefCell<Vec<SubCollectionData>>>,
    _sub_collection_data_indexes: Rc<RefCell<Vec<Rc<Cell<i32>>>>>,

    changed_event: Rc<RefCell<Event<VecDiff<T>>>>,
    _items_changed_event_subscription: Option<Subscription>,
}

struct SubCollectionData {
    pub pos: i32,  // starting position in the output collection
    pub size: i32, // number of elements
    _items_changed_event_subscription: Option<Subscription>,
}

impl<T: 'static + Clone> ObservableCollection<T> for ObservableCollectionFlatMap<T> {
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

pub trait ObservableCollectionFlatMapExt<TSrc>
where
    TSrc: Clone + 'static,
{
    fn flat_map<TDst, TDstColl, F>(&self, f: F) -> ObservableCollectionFlatMap<TDst>
    where
        TDst: Clone + 'static,
        TDstColl: ObservableCollection<TDst> + IntoIterator<Item = TDst>,
        F: 'static + FnMut(&TSrc) -> TDstColl;
}

impl<TSrc> ObservableCollectionFlatMapExt<TSrc> for dyn ObservableCollection<TSrc>
where
    TSrc: Clone + 'static,
{
    /// Flat map creates new observable collection.
    ///
    /// It keeps mapped copy of every item.
    ///
    /// The only connection between it and original observable collection
    /// is by subscribing on the `on_changed` event of the source collection,
    /// so we don't have to keep implicit reference to the source collection.
    /// The `on_change` event of source collection keeps a weak reference to our handler.
    fn flat_map<TDst, TDstColl, F>(&self, mut f: F) -> ObservableCollectionFlatMap<TDst>
    where
        TDst: Clone + 'static,
        TDstColl: ObservableCollection<TDst> + IntoIterator<Item = TDst>,
        F: FnMut(&TSrc) -> TDstColl + 'static,
    {
        let changed_event_rc = Rc::new(RefCell::new(Event::new()));

        // all the items from all sub-collections
        let items_rc = Rc::new(RefCell::new(Vec::new()));

        // sub-collections data
        let sub_collection_data_rc = Rc::new(RefCell::new(Vec::new()));
        let sub_collection_data_indexes_rc = Rc::new(RefCell::new(Vec::new()));

        // copy items from sub-collections and subscribe to each sub-collection's changes
        let mut pos = 0;
        let mut index = 0i32;
        for src_item in self as &dyn ObservableCollection<TSrc> {
            let dest_items = f(&src_item);
            let size = dest_items.len();

            // update sub_collection_data_indexes
            let sub_collection_data_index = Rc::new(Cell::new(index));
            let mut sub_collection_data_indexes = sub_collection_data_indexes_rc.borrow_mut();
            sub_collection_data_indexes.push(sub_collection_data_index.clone());

            // subscribe to changes
            let subscription = subscribe_to_subcollection(
                &dest_items,
                items_rc.clone(),
                sub_collection_data_rc.clone(),
                sub_collection_data_index,
                changed_event_rc.clone(),
            );

            sub_collection_data_rc.borrow_mut().push(SubCollectionData {
                pos,
                size: size as i32,
                _items_changed_event_subscription: subscription,
            });

            // insert items
            items_rc.borrow_mut().extend(dest_items);

            pos += size as i32;
            index += 1;
        }

        let handler = Box::new({
            let items_rc = items_rc.clone();
            let sub_collection_data_rc = sub_collection_data_rc.clone();
            let sub_collection_data_indexes_rc = sub_collection_data_indexes_rc.clone();
            let changed_event_rc = changed_event_rc.clone();
            move |changed_args| match changed_args {
                VecDiff::Clear {} => {
                    // we are removing all sub-collections
                    items_rc.borrow_mut().clear();
                    sub_collection_data_rc.borrow_mut().clear();
                    sub_collection_data_indexes_rc.borrow_mut().clear();
                    changed_event_rc.borrow().emit(VecDiff::Clear {});
                }

                VecDiff::InsertAt { index, value } => {
                    // we are inserting new sub-collection
                    insert_collection_at(
                        index,
                        f(&value),
                        sub_collection_data_rc.clone(),
                        sub_collection_data_indexes_rc.clone(),
                        items_rc.clone(),
                        changed_event_rc.clone(),
                    );
                }

                VecDiff::RemoveAt { index } => {
                    // we are removing a single sub-collection
                    remove_collection_at(
                        index,
                        sub_collection_data_rc.clone(),
                        sub_collection_data_indexes_rc.clone(),
                        items_rc.clone(),
                        changed_event_rc.clone(),
                    );
                }

                VecDiff::Move {
                    old_index,
                    new_index,
                } => {
                    // we are moving a single sub-collection
                    let mut sub_collection_data = sub_collection_data_rc.borrow_mut();
                    let data = sub_collection_data.remove(old_index);
                    let items_from = data.pos;
                    let items_len = data.size;
                    sub_collection_data.insert(new_index, data);

                    let mut sub_collection_data_indexes =
                        sub_collection_data_indexes_rc.borrow_mut();
                    let index = sub_collection_data_indexes.remove(old_index);
                    sub_collection_data_indexes.insert(new_index, index);

                    // update indexes
                    for i in old_index.min(new_index)..sub_collection_data_indexes.len() {
                        sub_collection_data_indexes[i].set(i as i32)
                    }

                    // update data
                    for i in old_index.min(new_index)..sub_collection_data_indexes.len() {
                        sub_collection_data[i].pos = if i > 0 {
                            sub_collection_data[i - 1].pos + sub_collection_data[i - 1].size
                        } else {
                            0
                        };
                    }

                    let items_to = sub_collection_data[new_index].pos;

                    // move items
                    let mut items = items_rc.borrow_mut();
                    let changed_event = changed_event_rc.borrow();
                    if items_from > items_to {
                        for _ in 0..items_len {
                            let old_index = (items_from + items_len - 1) as usize;
                            let new_index = items_to as usize;

                            let item = items.remove(old_index);
                            items.insert(new_index, item);

                            changed_event.emit(VecDiff::Move {
                                old_index,
                                new_index,
                            });
                        }
                    } else if items_from < items_to {
                        for _ in 0..items_len {
                            let old_index = items_from as usize;
                            let new_index = (items_to + items_len - 1) as usize;

                            let item = items.remove(old_index);
                            items.insert(new_index, item);

                            changed_event.emit(VecDiff::Move {
                                old_index,
                                new_index,
                            });
                        }
                    }
                }

                VecDiff::Pop {} => {
                    // we are popping a single sub-collection
                    let len = sub_collection_data_rc.borrow().len();
                    remove_collection_at(
                        len - 1,
                        sub_collection_data_rc.clone(),
                        sub_collection_data_indexes_rc.clone(),
                        items_rc.clone(),
                        changed_event_rc.clone(),
                    );
                }

                VecDiff::Push { value } => {
                    // we are pushing a single sub-collection
                    let len = sub_collection_data_rc.borrow().len();
                    insert_collection_at(
                        len,
                        f(&value),
                        sub_collection_data_rc.clone(),
                        sub_collection_data_indexes_rc.clone(),
                        items_rc.clone(),
                        changed_event_rc.clone(),
                    );
                }
            }
        });
        let event_subscription = self.on_changed(handler);

        ObservableCollectionFlatMap {
            items: items_rc,
            _sub_collection_data: sub_collection_data_rc,
            _sub_collection_data_indexes: sub_collection_data_indexes_rc,
            changed_event: changed_event_rc,
            _items_changed_event_subscription: event_subscription,
        }
    }
}

fn subscribe_to_subcollection<TDst, TDstColl>(
    new_items: &TDstColl,
    items_rc: Rc<RefCell<Vec<TDst>>>,
    sub_collection_data_rc: Rc<RefCell<Vec<SubCollectionData>>>,
    sub_collection_data_index_rc: Rc<Cell<i32>>,
    changed_event_rc: Rc<RefCell<Event<VecDiff<TDst>>>>,
) -> Option<Subscription>
where
    TDst: Clone + 'static,
    TDstColl: ObservableCollection<TDst> + IntoIterator<Item = TDst>,
{
    let handler = Box::new({
        move |changed_args: VecDiff<TDst>| match changed_args {
            VecDiff::Clear {} => {
                // clear all elements from current sub-collection
                // (but not remove it)

                let collection_index = sub_collection_data_index_rc.get() as usize;

                // update sub_collection_data
                let mut sub_collection_data = sub_collection_data_rc.borrow_mut();
                let pos = sub_collection_data[collection_index].pos;
                let size = sub_collection_data[collection_index].size;
                sub_collection_data[collection_index].size = 0;
                for i in collection_index + 1..sub_collection_data.len() {
                    sub_collection_data[i].pos -= size;
                }

                // remove items
                let mut items = items_rc.borrow_mut();
                let changed_event = changed_event_rc.borrow();
                for i in (pos..pos + size).rev() {
                    items.remove(i as usize);
                    changed_event.emit(VecDiff::RemoveAt { index: i as usize });
                }
            }

            VecDiff::InsertAt { index, value } => insert_element_at(
                index,
                value,
                items_rc.clone(),
                sub_collection_data_rc.clone(),
                sub_collection_data_index_rc.clone(),
                changed_event_rc.clone(),
            ),

            VecDiff::RemoveAt { index } => {
                remove_element_at(
                    index,
                    items_rc.clone(),
                    sub_collection_data_rc.clone(),
                    sub_collection_data_index_rc.clone(),
                    changed_event_rc.clone(),
                );
            }

            VecDiff::Move {
                old_index,
                new_index,
            } => {
                let value = remove_element_at(
                    old_index,
                    items_rc.clone(),
                    sub_collection_data_rc.clone(),
                    sub_collection_data_index_rc.clone(),
                    changed_event_rc.clone(),
                );
                insert_element_at(
                    new_index,
                    value,
                    items_rc.clone(),
                    sub_collection_data_rc.clone(),
                    sub_collection_data_index_rc.clone(),
                    changed_event_rc.clone(),
                )
            }

            VecDiff::Pop {} => {
                let sub_collection_data = sub_collection_data_rc.borrow_mut();
                let collection_index = sub_collection_data_index_rc.get() as usize;
                let index = (sub_collection_data[collection_index].size - 1) as usize;
                remove_element_at(
                    index,
                    items_rc.clone(),
                    sub_collection_data_rc.clone(),
                    sub_collection_data_index_rc.clone(),
                    changed_event_rc.clone(),
                );
            }

            VecDiff::Push { value } => {
                let sub_collection_data = sub_collection_data_rc.borrow_mut();
                let collection_index = sub_collection_data_index_rc.get() as usize;
                let index = sub_collection_data[collection_index].size as usize;
                insert_element_at(
                    index,
                    value,
                    items_rc.clone(),
                    sub_collection_data_rc.clone(),
                    sub_collection_data_index_rc.clone(),
                    changed_event_rc.clone(),
                );
            }
        }
    });
    new_items.on_changed(handler)
}

fn remove_collection_at<T: 'static + Clone>(
    index: usize,
    sub_collection_data_rc: Rc<RefCell<Vec<SubCollectionData>>>,
    sub_collection_data_indexes_rc: Rc<RefCell<Vec<Rc<Cell<i32>>>>>,
    items_rc: Rc<RefCell<Vec<T>>>,
    changed_event_rc: Rc<RefCell<Event<VecDiff<T>>>>,
) {
    let mut sub_collection_data = sub_collection_data_rc.borrow_mut();

    // remove subscription
    let removed_data = sub_collection_data.remove(index);

    // update sub_collection_data_indexes
    let mut sub_collection_data_indexes = sub_collection_data_indexes_rc.borrow_mut();
    sub_collection_data_indexes.remove(index);
    for i in index..sub_collection_data_indexes.len() {
        sub_collection_data_indexes[i].set(i as i32)
    }

    // fix indexes of other sub-collections
    for i in index..sub_collection_data.len() {
        sub_collection_data[i].pos -= removed_data.size;
    }

    // remove elements
    let mut items = items_rc.borrow_mut();
    let changed_event = changed_event_rc.borrow();
    for i in (removed_data.pos..removed_data.pos + removed_data.size).rev() {
        items.remove(i as usize);
        changed_event.emit(VecDiff::RemoveAt { index: i as usize });
    }
}

fn insert_collection_at<TDst, TDstColl>(
    index: usize,
    value: TDstColl,
    sub_collection_data_rc: Rc<RefCell<Vec<SubCollectionData>>>,
    sub_collection_data_indexes_rc: Rc<RefCell<Vec<Rc<Cell<i32>>>>>,
    items_rc: Rc<RefCell<Vec<TDst>>>,
    changed_event_rc: Rc<RefCell<Event<VecDiff<TDst>>>>,
) where
    TDst: Clone + 'static,
    TDstColl: ObservableCollection<TDst> + IntoIterator<Item = TDst>,
{
    let mut sub_collection_data = sub_collection_data_rc.borrow_mut();
    let new_pos = if index > 0 {
        sub_collection_data[index - 1].pos + sub_collection_data[index - 1].size
    } else {
        0
    };

    // update sub_collection_data_indexes
    let sub_collection_data_index = Rc::new(Cell::new(index as i32));
    let mut sub_collection_data_indexes = sub_collection_data_indexes_rc.borrow_mut();
    sub_collection_data_indexes.insert(index, sub_collection_data_index.clone());
    for i in index + 1..sub_collection_data_indexes.len() {
        sub_collection_data_indexes[i].set(i as i32)
    }

    // get new items
    let new_items = value;

    // subscribe to changes
    let subscription = subscribe_to_subcollection(
        &new_items,
        items_rc.clone(),
        sub_collection_data_rc.clone(),
        sub_collection_data_index,
        changed_event_rc.clone(),
    );

    // update sub_collection_data
    let size = new_items.len() as i32;
    sub_collection_data.insert(
        index,
        SubCollectionData {
            pos: new_pos,
            size,
            _items_changed_event_subscription: subscription,
        },
    );
    for index in index + 1..sub_collection_data.len() {
        sub_collection_data[index].pos += size;
    }

    // insert new items
    for (index, new_item) in new_items.into_iter().enumerate() {
        items_rc
            .borrow_mut()
            .insert(new_pos as usize + index, new_item.clone());
        changed_event_rc.borrow().emit(VecDiff::InsertAt {
            index: new_pos as usize + index,
            value: new_item,
        });
    }
}

fn remove_element_at<T: Clone + 'static>(
    index: usize,
    items_rc: Rc<RefCell<Vec<T>>>,
    sub_collection_data_rc: Rc<RefCell<Vec<SubCollectionData>>>,
    sub_collection_data_index_rc: Rc<Cell<i32>>,
    changed_event_rc: Rc<RefCell<Event<VecDiff<T>>>>,
) -> T {
    let collection_index = sub_collection_data_index_rc.get() as usize;

    // update sub_collection_data
    let mut sub_collection_data = sub_collection_data_rc.borrow_mut();
    let pos = sub_collection_data[collection_index].pos as usize;
    sub_collection_data[collection_index].size -= 1;
    for i in collection_index + 1..sub_collection_data.len() {
        sub_collection_data[i].pos -= 1;
    }

    // remove item
    let mut items = items_rc.borrow_mut();
    let value = items.remove(pos + index);
    changed_event_rc
        .borrow()
        .emit(VecDiff::RemoveAt { index: pos + index });
    value
}

fn insert_element_at<T: Clone + 'static>(
    index: usize,
    value: T,
    items_rc: Rc<RefCell<Vec<T>>>,
    sub_collection_data_rc: Rc<RefCell<Vec<SubCollectionData>>>,
    sub_collection_data_index_rc: Rc<Cell<i32>>,
    changed_event_rc: Rc<RefCell<Event<VecDiff<T>>>>,
) {
    let collection_index = sub_collection_data_index_rc.get() as usize;

    // update sub_collection_data
    let mut sub_collection_data = sub_collection_data_rc.borrow_mut();
    let pos = sub_collection_data[collection_index].pos as usize;
    sub_collection_data[collection_index].size += 1;
    for i in collection_index + 1..sub_collection_data.len() {
        sub_collection_data[i].pos += 1;
    }

    // insert item
    let mut items = items_rc.borrow_mut();
    items.insert(pos + index, value.clone());
    changed_event_rc.borrow().emit(VecDiff::InsertAt {
        index: pos + index,
        value,
    });
}

impl<TSrc, TSrcColl> ObservableCollectionFlatMapExt<TSrc> for TSrcColl
where
    TSrc: Clone + 'static,
    TSrcColl: ObservableCollection<TSrc> + 'static,
{
    /// Flat map creates new observable collection.
    ///
    /// It keeps mapped copy of every item.
    ///
    /// The only connection between it and original observable collection
    /// is by subscribing on the `on_changed` event of the source collection,
    /// so we don't have to keep implicit reference to the source collection.
    /// The `on_change` event of source collection keeps a weak reference to our handler.
    fn flat_map<TDst, TDstColl, F>(&self, f: F) -> ObservableCollectionFlatMap<TDst>
    where
        TDst: Clone + 'static,
        TDstColl: ObservableCollection<TDst> + IntoIterator<Item = TDst>,
        F: FnMut(&TSrc) -> TDstColl + 'static,
    {
        (self as &dyn ObservableCollection<TSrc>).flat_map(f)
    }
}
