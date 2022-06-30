use crate::{ControlObject, ObservableCollection, ObservableComposite, Subscription, VecDiff};
use std::cell::RefCell;
use std::rc::Rc;

/// Children collection of a control.
///
/// The collection is an enum to make it optimized for the cases with
/// having static list of controls.
pub enum Children {
    /// The collection has no items.
    None,

    /// The collection has a single child.
    SingleStatic(Rc<RefCell<dyn ControlObject>>),

    /// The collection is a list of controls.
    MultipleStatic(Vec<Rc<RefCell<dyn ControlObject>>>),

    /// The children comes from a single observable collection.
    SingleDynamic(Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>),
}

impl Children {
    /// Creates an empty children collection.
    pub fn empty() -> Self {
        Children::None
    }

    /// Constructs Children collection from
    /// vector of Children collections.
    pub fn from(children_vec: Vec<Children>) -> Self {
        let mut sources_to_compose: Vec<
            Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>,
        > = Vec::new();
        let mut static_children: Vec<Rc<RefCell<dyn ControlObject>>> = Vec::new();

        for next in children_vec {
            match next {
                Children::None => (),
                Children::SingleStatic(item) => {
                    static_children.push(item);
                }
                Children::MultipleStatic(mut items) => {
                    static_children.append(&mut items);
                }
                Children::SingleDynamic(items) => {
                    if static_children.len() > 0 {
                        sources_to_compose.push(Box::new(static_children));
                        static_children = Vec::new();
                    }
                    sources_to_compose.push(items);
                }
            }
        }

        if sources_to_compose.len() > 0 && static_children.len() > 0 {
            sources_to_compose.push(Box::new(static_children));
            static_children = Vec::new();
        }

        if static_children.len() == 1 {
            Children::SingleStatic(static_children.into_iter().next().unwrap())
        } else if static_children.len() > 1 {
            Children::MultipleStatic(static_children)
        } else if sources_to_compose.len() == 1 {
            Children::SingleDynamic(sources_to_compose.into_iter().next().unwrap())
        } else if sources_to_compose.len() > 1 {
            Children::SingleDynamic(Box::new(ObservableComposite::from(sources_to_compose)))
        } else {
            Children::None
        }
    }

    /// Returns number of controls in the children collection.
    pub fn len(&self) -> usize {
        match self {
            Children::None => 0,
            Children::SingleStatic(_) => 1,
            Children::MultipleStatic(x) => x.len(),
            Children::SingleDynamic(x) => x.len(),
        }
    }

    /// Tries to get Rc reference to the control at the `index` position.
    pub fn get(&self, index: usize) -> Option<Rc<RefCell<dyn ControlObject>>> {
        match self {
            Children::None => None,
            Children::SingleStatic(x) => {
                if index == 0 {
                    Some(x.clone())
                } else {
                    None
                }
            }
            Children::MultipleStatic(x) => x.get(index),
            Children::SingleDynamic(x) => x.get(index),
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

/// Converts vector of controls to ChildEntry.
impl From<Vec<Rc<RefCell<dyn ControlObject>>>> for Children {
    fn from(items: Vec<Rc<RefCell<dyn ControlObject>>>) -> Children {
        Children::MultipleStatic(items)
    }
}

/// Converts vector of controls to ChildEntry.
impl<T: 'static + ControlObject> From<Vec<Rc<RefCell<T>>>> for Children {
    fn from(items: Vec<Rc<RefCell<T>>>) -> Children {
        Children::MultipleStatic(
            items
                .into_iter()
                .map(|item| item as Rc<RefCell<dyn ControlObject>>)
                .collect(),
        )
    }
}

/// Converts an observable collection to ChildEntry.
impl<T: Into<Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>> From<T> for Children {
    fn from(items: T) -> Children {
        Children::SingleDynamic(items.into())
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
        f: Box<dyn Fn(VecDiff<Rc<RefCell<dyn ControlObject>>>)>,
    ) -> Option<Subscription> {
        match self {
            Children::None | Children::SingleStatic(_) | Children::MultipleStatic(_) => None,

            Children::SingleDynamic(x) => x.on_changed(f),
        }
    }
}
