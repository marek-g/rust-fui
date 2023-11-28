use std::cell::RefCell;
use std::rc::{Rc, Weak};
use typemap::TypeMap;

use crate::{control::ControlObject, Children};

pub struct ViewContext {
    pub attached_values: TypeMap,
    pub children: Children,
}

impl ViewContext {
    pub fn empty() -> ViewContext {
        ViewContext {
            attached_values: TypeMap::new(),
            children: Children::empty(),
        }
    }
}

///
/// Used to convert controls to views.
/// Controls can be consumed during conversion.
///
//pub trait Control: Sized {
//    fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<StyledControl<Self>>>;
//}
//
// Instead of using this trait we expect control structs to just implement this method (by convention).
// It will be called from ui!() macro.
// The reason why it is not a trait is that sometimes composite controls may not want
// to return StyledControl<Self> but the root control of the composition that may be
// for example StyledControl<Grid> or just dyn ControlObject.
// Also we cannot always return dyn ControlObject, because sometimes we may want to create controls
// with ui!() macro and later have access to its type, for example
// in a cases like creating radio buttons and passing their references to the radio button controller.
//

///
/// Used to convert view models to views.
///
/// This is a trait with static methods. In Rust 2018 you cannot
/// use Rc<RefCell<Self>> as a self type (feature: arbitrary self types).
/// Rc<Self> could work but it would give us more troubles with handling
/// mutable state in View Models.
/// So we use static methods here but we declare additional ViewModelObject
/// trait which is an object safe trait.
///
pub trait ViewModel {
    fn create_view(view_model: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>>;
}

///
/// Object safe version of ViewModel's trait.
///
pub trait ViewModelObject {
    fn create_view(&self) -> Rc<RefCell<dyn ControlObject>>;

    fn box_clone(&self) -> Box<dyn ViewModelObject>;
    fn downgrade(&self) -> Box<dyn WeakViewModelObject>;
}

impl<T: ViewModel + 'static> ViewModelObject for Rc<T> {
    fn create_view(&self) -> Rc<RefCell<dyn ControlObject>> {
        ViewModel::create_view(self)
    }

    fn box_clone(&self) -> Box<dyn ViewModelObject> {
        Box::new(std::clone::Clone::clone(self))
    }
    fn downgrade(&self) -> Box<dyn WeakViewModelObject> {
        Box::new(Rc::downgrade(self))
    }
}

impl Clone for Box<dyn ViewModelObject> {
    fn clone(&self) -> Self {
        (*self).box_clone()
    }
}

///
/// Object safe trait for weak ViewModel's reference.
///
pub trait WeakViewModelObject {
    fn box_clone(&self) -> Box<dyn WeakViewModelObject>;
    fn upgrade(&self) -> Option<Box<dyn ViewModelObject>>;
}

impl<T: ViewModel + 'static> WeakViewModelObject for Weak<T> {
    fn box_clone(&self) -> Box<dyn WeakViewModelObject> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn upgrade(&self) -> Option<Box<dyn ViewModelObject>> {
        self.upgrade()
            .map(|rc| Box::new(rc) as Box<(dyn ViewModelObject + 'static)>)
    }
}

impl Clone for Box<dyn WeakViewModelObject> {
    fn clone(&self) -> Self {
        (*self).box_clone()
    }
}
