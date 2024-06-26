use futures_signals::signal::{Mutable, SignalExt};
use std::sync::{Arc, RwLock};

use crate::ObservableCollection;
use crate::{spawn_local, Color, Subscription, VecDiff};

pub struct Property<T> {
    data: Mutable<T>,
    bind_handle: Arc<RwLock<Option<Subscription>>>,
}

impl<T: 'static + Clone + PartialEq> Property<T> {
    pub fn new<U: Into<T>>(val: U) -> Self {
        Property {
            data: Mutable::new(val.into()),
            bind_handle: Arc::new(RwLock::new(None)),
        }
    }

    pub fn binded_from(src_property: &Property<T>) -> Self {
        let new_property = Property {
            data: Mutable::new(src_property.get()),
            bind_handle: Arc::new(RwLock::new(None)),
        };
        new_property.bind(src_property);
        new_property
    }

    pub fn binded_c_from<TSrc: 'static + Clone + PartialEq, F: 'static + Fn(TSrc) -> T>(
        src_property: &Property<TSrc>,
        f: F,
    ) -> Self {
        let new_property = Property::new(f(src_property.get()));
        new_property.bind_c(src_property, f);
        new_property
    }

    pub fn binded_to(dst_property: &Property<T>, init_value: T) -> Self {
        let property = Property::new(init_value);
        dst_property.bind(&property);
        property
    }

    pub fn binded_c_to<TDst: 'static + Clone + PartialEq, F: 'static + Fn(T) -> TDst>(
        dst_property: &Property<TDst>,
        f: F,
        init_value: T,
    ) -> Self {
        let property = Property::new(init_value);
        dst_property.bind_c(&property, f);
        property
    }

    pub fn binded_two_way(other_property: &Property<T>) -> Self {
        other_property.clone()
    }

    pub fn binded_c_two_way<TOther, F1, F2>(
        other_property: &Property<TOther>,
        f1: F1,
        f2: F2,
    ) -> Self
    where
        TOther: 'static + Clone + PartialEq,
        F1: 'static + Fn(TOther) -> T,
        F2: 'static + Fn(T) -> TOther,
    {
        let property = Property::binded_c_from(other_property, f1);
        other_property.bind_c(&property, f2);
        property
    }

    pub fn set(&self, val: T) {
        self.data.set_neq(val);
    }

    pub fn change<F: 'static + Fn(T) -> T>(&self, f: F) {
        let val = self.data.get_cloned();
        self.data.set_neq(f(val));
    }

    pub fn get(&self) -> T {
        self.data.get_cloned()
    }

    pub fn bind(&self, src_property: &Property<T>) {
        let handle = spawn_local(src_property.data.signal_cloned().for_each({
            let data = self.data.clone();
            move |v| {
                data.set_neq(v);
                async {}
            }
        }));
        self.bind_handle
            .write()
            .unwrap()
            .replace(Subscription::SpawnLocal(handle));
    }

    pub fn bind_c<TSrc: 'static + Clone + PartialEq, F: 'static + Fn(TSrc) -> T>(
        &self,
        src_property: &Property<TSrc>,
        f: F,
    ) {
        let handle = spawn_local(src_property.data.signal_cloned().for_each({
            let data = self.data.clone();
            move |v| {
                data.set_neq(f(v));
                async {}
            }
        }));
        self.bind_handle
            .write()
            .unwrap()
            .replace(Subscription::SpawnLocal(handle));
    }

    pub fn on_changed<F: 'static + FnMut(T)>(&self, mut f: F) -> Subscription {
        Subscription::SpawnLocal(spawn_local(self.data.signal_cloned().for_each(move |v| {
            f(v);
            async {}
        })))
    }
}

impl<T: 'static + Clone + PartialEq> Clone for Property<T> {
    fn clone(&self) -> Self {
        Property::<T> {
            data: self.data.clone(),
            bind_handle: self.bind_handle.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
        self.bind_handle.clone_from(&source.bind_handle);
    }
}

///
/// Used to attribute types that can be
/// automatically converted to Property<T>.
///
pub trait IntoProperty {}

impl IntoProperty for String {}
impl IntoProperty for &String {}
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
impl IntoProperty for Color {}

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
    T: 'static + Clone + PartialEq,
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
    T: 'static + Clone + PartialEq,
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
    T: 'static + Clone + PartialEq,
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
    TSrc: 'static + Clone + PartialEq,
    TDest: 'static + Clone + PartialEq,
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
    TSrc: 'static + Clone + PartialEq,
    TDest: 'static + Clone + PartialEq,
    F1: 'static + Fn(TSrc) -> TDest,
    F2: 'static + Fn(TDest) -> TSrc,
{
    fn from(value: (&mut Property<TSrc>, F1, F2)) -> Property<TDest> {
        Property::binded_c_two_way(value.0, value.1, value.2)
    }
}

///
/// ObservableCollection for Property.
///
impl<T> ObservableCollection<T> for Property<T>
where
    T: 'static + Clone + PartialEq,
{
    fn len(&self) -> usize {
        1
    }

    fn get(&self, index: usize) -> Option<T> {
        if index == 0 {
            Some(Property::get(self))
        } else {
            None
        }
    }

    fn on_changed(&self, mut f: Box<dyn FnMut(VecDiff<T>)>) -> Option<Subscription> {
        Some(Property::on_changed(self, move |v| {
            f(VecDiff::RemoveAt { index: 0 });
            f(VecDiff::InsertAt { index: 0, value: v });
        }))
    }
}

impl<T> ObservableCollection<T> for Property<Option<T>>
where
    T: 'static + Clone + PartialEq,
{
    fn len(&self) -> usize {
        if self.get().is_some() {
            1
        } else {
            0
        }
    }

    fn get(&self, index: usize) -> Option<T> {
        if index == 0 {
            Property::get(&self)
        } else {
            None
        }
    }

    fn on_changed(&self, mut f: Box<dyn FnMut(VecDiff<T>)>) -> Option<Subscription> {
        Some(Property::on_changed(self, move |v| {
            f(VecDiff::Clear {});
            if let Some(v) = v {
                f(VecDiff::InsertAt { index: 0, value: v });
            }
        }))
    }
}
