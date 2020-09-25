use crate::{
    ControlObject, EventSubscription, ObservableChangedEventArgs, ObservableCollection, SubChildren,
};
use std::cell::RefCell;
use std::rc::Rc;

/// Children collection of a control.
///
/// The collection is an enum to make it optimized for the most common cases.
pub enum Children {
    /// The collection has no items.
    None,

    /// The collection has a single child.
    SingleStatic(Rc<RefCell<dyn ControlObject>>),

    /// The children comes from a single observable collection.
    SingleDynamic(Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>),

    /// The collection is a list of controls.
    MultipleStatic(Vec<Rc<RefCell<dyn ControlObject>>>),

    /// The collection is a mix of controls and observable collections.
    MultipleMixed(Vec<SubChildren>),
}

impl Children {
    /// Creates an empty children collection.
    pub fn new() -> Self {
        Children::None
    }

    /// Constructs Children collection from
    /// vector of Children collections.
    pub fn from(children_vec: Vec<Children>) -> Self {
        let mut iter = children_vec.into_iter();
        if let Some(next) = iter.next() {
            let mut result = next;
            while let Some(next) = iter.next() {
                result = result.append(next)
            }
            result
        } else {
            Children::None
        }
    }

    /// Returns number of controls in the children collection.
    pub fn len(&self) -> usize {
        match self {
            Children::None => 0,
            Children::SingleStatic(_) => 1,
            Children::SingleDynamic(x) => x.len(),
            Children::MultipleStatic(x) => x.len(),
            Children::MultipleMixed(x) => x.iter().map(|i| i.len()).sum(),
        }
    }

    /// Tries to get Rc reference to the control at the `index` position.
    pub fn get(&self, mut index: usize) -> Option<Rc<RefCell<dyn ControlObject>>> {
        match self {
            Children::None => None,
            Children::SingleStatic(x) => {
                if index == 0 {
                    Some(x.clone())
                } else {
                    None
                }
            }
            Children::SingleDynamic(x) => x.get(index),
            Children::MultipleStatic(x) => x.get(index),
            Children::MultipleMixed(x) => {
                for sub_children in x {
                    let len = sub_children.len();
                    if index < len {
                        return sub_children.get(index);
                    } else {
                        index -= len;
                    }
                }
                None
            }
        }
    }

    /*fn on_changed(
        &self,
        f: Box<dyn Fn(ObservableChangedEventArgs<Rc<RefCell<dyn ControlObject>>>)>,
    ) -> Option<EventSubscription> {
        Some(
            self.changed_event
                .borrow_mut()
                .subscribe(move |args| f(args)),
        )
    }*/

    /// Appends another children collection to self.
    /// Returns new instance of an enum.  
    fn append(self, children: Children) -> Self {
        match children {
            Children::None => self,
            Children::SingleStatic(x) => self.add(Children::SingleStatic(x)),
            Children::SingleDynamic(x) => self.add(Children::SingleDynamic(x)),
            Children::MultipleStatic(x) => {
                let mut result = self;
                for el in x.into_iter() {
                    result = result.add(Children::SingleStatic(el));
                }
                result
            }
            Children::MultipleMixed(x) => {
                let mut result = self;
                for el in x.into_iter() {
                    match el {
                        SubChildren::SingleStatic(x) => {
                            result = result.add(Children::SingleStatic(x))
                        }
                        SubChildren::SingleDynamic(x) => {
                            result = result.add(Children::SingleDynamic(x))
                        }
                        SubChildren::MultipleStatic(x) => {
                            for el in x.into_iter() {
                                result = result.add(Children::SingleStatic(el));
                            }
                        }
                    }
                }
                result
            }
        }
    }

    /// Adds a control entry (single control or single observable collection)
    /// to the children collection.
    /// Returns new instance of an enum.
    fn add(self, child: Children) -> Self {
        match self {
            Children::None => match child {
                Children::SingleStatic(c) => Children::SingleStatic(c),

                Children::SingleDynamic(c) => Children::SingleDynamic(c),

                _ => unreachable!(),
            },

            Children::SingleStatic(x) => match child {
                Children::SingleStatic(c) => Children::MultipleStatic(vec![x, c]),

                Children::SingleDynamic(c) => Children::MultipleMixed(vec![
                    SubChildren::SingleStatic(x),
                    SubChildren::SingleDynamic(c),
                ]),

                _ => unreachable!(),
            },

            Children::SingleDynamic(x) => match child {
                Children::SingleStatic(c) => Children::MultipleMixed(vec![
                    SubChildren::SingleDynamic(x),
                    SubChildren::SingleStatic(c),
                ]),

                Children::SingleDynamic(c) => Children::MultipleMixed(vec![
                    SubChildren::SingleDynamic(x),
                    SubChildren::SingleDynamic(c),
                ]),

                _ => unreachable!(),
            },

            Children::MultipleStatic(mut x) => match child {
                Children::SingleStatic(c) => {
                    x.push(c);
                    Children::MultipleStatic(x)
                }

                Children::SingleDynamic(c) => Children::MultipleMixed(vec![
                    SubChildren::MultipleStatic(x),
                    SubChildren::SingleDynamic(c),
                ]),

                _ => unreachable!(),
            },

            Children::MultipleMixed(mut x) => match child {
                Children::SingleStatic(c) => {
                    if let Some(last) = x.pop() {
                        match last {
                            SubChildren::SingleStatic(l) => {
                                x.push(SubChildren::MultipleStatic(vec![l, c]));
                                Children::MultipleMixed(x)
                            }

                            SubChildren::SingleDynamic(l) => {
                                x.push(SubChildren::SingleDynamic(l));
                                x.push(SubChildren::SingleStatic(c));
                                Children::MultipleMixed(x)
                            }

                            SubChildren::MultipleStatic(mut l) => {
                                l.push(c);
                                x.push(SubChildren::MultipleStatic(l));
                                Children::MultipleMixed(x)
                            }
                        }
                    } else {
                        Children::SingleStatic(c)
                    }
                }

                Children::SingleDynamic(c) => {
                    if let Some(last) = x.pop() {
                        match last {
                            SubChildren::SingleStatic(l) => {
                                x.push(SubChildren::SingleStatic(l));
                                x.push(SubChildren::SingleDynamic(c));
                                Children::MultipleMixed(x)
                            }

                            SubChildren::SingleDynamic(l) => {
                                x.push(SubChildren::SingleDynamic(l));
                                x.push(SubChildren::SingleDynamic(c));
                                Children::MultipleMixed(x)
                            }

                            SubChildren::MultipleStatic(l) => {
                                x.push(SubChildren::MultipleStatic(l));
                                x.push(SubChildren::SingleDynamic(c));
                                Children::MultipleMixed(x)
                            }
                        }
                    } else {
                        Children::SingleDynamic(c)
                    }
                }

                _ => unreachable!(),
            },
        }
    }
}

/// Converts a single control to Children collection.
impl From<Rc<RefCell<dyn ControlObject>>> for Children {
    fn from(item: Rc<RefCell<dyn ControlObject>>) -> Children {
        Children::SingleStatic(item)
    }
}

/// Converts a single control to ChildEntry.
impl<T: 'static + ControlObject> From<Rc<RefCell<T>>> for Children {
    fn from(item: Rc<RefCell<T>>) -> Children {
        Children::SingleStatic(item)
    }
}

/// Converts an observable collection to ChildEntry.
impl<T: Into<Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>> From<T> for Children {
    fn from(item: T) -> Children {
        Children::SingleDynamic(item.into())
    }
}

///
/// ObservableCollection for Children.
///
impl ObservableCollection<Rc<RefCell<dyn ControlObject>>> for Children {
    fn len(&self) -> usize {
        Children::len(self)
    }

    fn get(&self, index: usize) -> Option<Rc<RefCell<dyn ControlObject>>> {
        Children::get(self, index)
    }

    fn on_changed(
        &self,
        f: Box<dyn Fn(ObservableChangedEventArgs<Rc<RefCell<dyn ControlObject>>>)>,
    ) -> Option<EventSubscription> {
        match self {
            Children::None | Children::SingleStatic(_) | Children::MultipleStatic(_) => None,

            Children::SingleDynamic(x) => x.on_changed(f),

            Children::MultipleMixed(children) => {
                // TODO: implement!!
                for child in children {
                    match child {
                        SubChildren::SingleStatic(x) => {}
                        SubChildren::MultipleStatic(x) => {}
                        SubChildren::SingleDynamic(x) => {}
                    }
                }
                None
            }
        }
    }
}
