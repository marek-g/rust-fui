use std::cell::RefCell;
use std::rc::Rc;

use crate::Event;
use crate::EventSubscription;

pub struct Property<T> {
    data: Rc<RefCell<PropertyData<T>>>,
    binding_subscription: Option<EventSubscription>,
}

impl<T: 'static + Clone + PartialEq + Default> Property<T> {
    pub fn new<U: Into<T>>(val: U) -> Self {
        Property {
            data: Rc::new(RefCell::new(PropertyData::new(val.into()))),
            binding_subscription: None,
        }
    }

    pub fn binded_from(src_property: &Property<T>) -> Self {
        let mut property = Property::new(T::default());
        property.bind(src_property);
        property
    }

    pub fn binded_c_from<
        TSrc: 'static + Clone + PartialEq + Default,
        F: 'static + Fn(TSrc) -> T,
    >(
        src_property: &Property<TSrc>,
        f: F,
    ) -> Self {
        let mut property = Property::new(T::default());
        property.bind_c(src_property, f);
        property
    }

    pub fn binded_to(dst_property: &mut Property<T>) -> Self {
        let property = Property::new(T::default());
        dst_property.bind(&property);
        property
    }

    pub fn binded_c_to<TDst: 'static + Clone + PartialEq + Default, F: 'static + Fn(T) -> TDst>(
        dst_property: &mut Property<TDst>,
        f: F,
    ) -> Self {
        let mut property = Property::new(T::default());
        dst_property.bind_c(&mut property, f);
        property
    }

    pub fn binded_two_way(other_property: &mut Property<T>) -> Self {
        let mut property = Property::binded_from(&other_property);
        other_property.bind(&mut property);
        property
    }

    pub fn binded_c_two_way<TOther, F1, F2>(
        other_property: &mut Property<TOther>,
        f1: F1,
        f2: F2,
    ) -> Self
    where
        TOther: 'static + Clone + PartialEq + Default,
        F1: 'static + Fn(TOther) -> T,
        F2: 'static + Fn(T) -> TOther,
    {
        let mut property = Property::binded_c_from(other_property, f1);
        other_property.bind_c(&mut property, f2);
        property
    }

    pub fn set(&mut self, val: T) {
        self.data.borrow_mut().set(val);
    }

    pub fn change<F: 'static + Fn(T) -> T>(&mut self, f: F) {
        let val = self.data.borrow().get();
        self.data.borrow_mut().set(f(val));
    }

    pub fn get(&self) -> T {
        self.data.borrow().get()
    }

    pub fn bind(&mut self, src_property: &Property<T>) {
        self.set(src_property.get());

        let weak_data_dest = Rc::downgrade(&self.data);
        self.binding_subscription = Some(src_property.data.borrow_mut().changed.subscribe(
            move |src_val| {
                if let Some(dest_property_data) = weak_data_dest.upgrade() {
                    dest_property_data.borrow_mut().set(src_val.clone());
                }
            },
        ))
    }

    pub fn bind_c<TSrc: 'static + Clone + PartialEq + Default, F: 'static + Fn(TSrc) -> T>(
        &mut self,
        src_property: &Property<TSrc>,
        f: F,
    ) {
        self.set(f(src_property.get()));

        let weak_data_dest = Rc::downgrade(&self.data);
        let boxed_f = Box::new(f);
        self.binding_subscription = Some(src_property.data.borrow_mut().changed.subscribe(
            move |src_val| {
                if let Some(dest_property_data) = weak_data_dest.upgrade() {
                    dest_property_data.borrow_mut().set(boxed_f(src_val));
                }
            },
        ))
    }

    pub fn on_changed<F: 'static + Fn(T)>(&mut self, f: F) -> EventSubscription {
        self.data.borrow_mut().changed.subscribe(f)
    }
}

struct PropertyData<T> {
    value: T,
    changed: Event<T>,
}

impl<T: 'static + Clone + PartialEq + Default> PropertyData<T> {
    fn new(val: T) -> Self {
        PropertyData {
            value: val,
            changed: Event::new(),
        }
    }

    fn set(&mut self, val: T) {
        if self.value != val {
            self.value = val.clone();
            self.changed.emit(val);
        }
    }

    fn get(&self) -> T {
        self.value.clone()
    }
}

///
/// Used to attribute types that can be
/// automatically converted to Property<T>.
///
pub trait IntoProperty {}

impl IntoProperty for String {}
impl IntoProperty for &str {}
impl IntoProperty for bool {}
impl IntoProperty for char {}
impl IntoProperty for i8 {}
impl IntoProperty for i16 {}
impl IntoProperty for i32 {}
impl IntoProperty for i64 {}
impl IntoProperty for i128 {}
impl IntoProperty for isize {}
impl IntoProperty for u8 {}
impl IntoProperty for u16 {}
impl IntoProperty for u32 {}
impl IntoProperty for u64 {}
impl IntoProperty for u128 {}
impl IntoProperty for usize {}
impl IntoProperty for f32 {}
impl IntoProperty for f64 {}

///
/// Allows to convert types attributed with IntoProperty to Property<T>.
///
/// Cannot do it for all types at once, because then there is a conflict
/// with binding conversions (tuples and properties used as source).
///
/// Example:
///
/// ui! { Control { int_property: 10, text_property: "My Text" }}
///
impl<T, U> From<U> for Property<T>
where
    T: 'static + Clone + PartialEq + Default,
    U: Into<T> + IntoProperty,
{
    fn from(value: U) -> Property<T> {
        Property::new(value.into())
    }
}

///
/// Allows to easily write one-way binding.
///
/// Example:
///
/// ui! { Control { text_property: &vm.text }}
///
impl<T> From<&Property<T>> for Property<T>
where
    T: 'static + Clone + PartialEq + Default,
{
    fn from(value: &Property<T>) -> Property<T> {
        Property::binded_from(value)
    }
}

///
/// Allows to easily write two-way bindings.
///
/// Example:
///
/// ui! { Control { text_property: &mut vm.text }}
///
impl<T> From<&mut Property<T>> for Property<T>
where
    T: 'static + Clone + PartialEq + Default,
{
    fn from(value: &mut Property<T>) -> Property<T> {
        Property::binded_two_way(value)
    }
}

///
/// Allows to easily write one-way binding with converter.
///
/// Example:
///
/// ui! { Control { text_property: (&vm.count, |c| c.to_string()) }}
///
impl<TSrc, TDest, F> From<(&Property<TSrc>, F)> for Property<TDest>
where
    TSrc: 'static + Clone + PartialEq + Default,
    TDest: 'static + Clone + PartialEq + Default,
    F: 'static + Fn(TSrc) -> TDest,
{
    fn from(value: (&Property<TSrc>, F)) -> Property<TDest> {
        Property::binded_c_from(value.0, value.1)
    }
}

///
/// Allows to easily write two-way binding with converter.
///
/// Example:
///
/// ui! { Control { text_property: (&mut vm.count,
///     |c| c.to_string(), |c| c.parse().unwrap()) }}
///
impl<TSrc, TDest, F1, F2> From<(&mut Property<TSrc>, F1, F2)> for Property<TDest>
where
    TSrc: 'static + Clone + PartialEq + Default,
    TDest: 'static + Clone + PartialEq + Default,
    F1: 'static + Fn(TSrc) -> TDest,
    F2: 'static + Fn(TDest) -> TSrc,
{
    fn from(value: (&mut Property<TSrc>, F1, F2)) -> Property<TDest> {
        Property::binded_c_two_way(value.0, value.1, value.2)
    }
}
