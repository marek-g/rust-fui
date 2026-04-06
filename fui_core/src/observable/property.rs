use futures_signals::signal::{Mutable, MutableLockMut, MutableLockRef, SignalExt};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::ObservableCollection;
use crate::{spawn_local, Subscription, VecDiff};
use fui_drawing::Color;

/// Type-erased subscription that can subscribe to any Property<V> and notify
/// a target property when the source changes.
pub struct PropertySubscription {
    subscribe: Rc<dyn Fn(Rc<dyn Fn()>) -> Subscription>,
}

impl PropertySubscription {
    /// Creates a new PropertySubscription from a source Property.
    pub fn from_property<V>(property: &Property<V>) -> Self
    where
        V: 'static + Clone + PartialEq,
    {
        let property = property.clone();
        PropertySubscription {
            subscribe: Rc::new(move |notify_fn| {
                property.on_changed(move |_| {
                    notify_fn();
                })
            }),
        }
    }

    /// Subscribes to the source property and calls the notify function when it changes.
    pub fn subscribe(&self, notify_fn: Rc<dyn Fn()>) -> Subscription {
        (self.subscribe)(notify_fn)
    }
}

#[repr(transparent)]
pub struct PropertyReadGuard<'a, T> {
    pub(crate) inner: MutableLockRef<'a, T>,
}

impl<'a, T> Deref for PropertyReadGuard<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
pub struct PropertyWriteGuard<'a, T> {
    pub(crate) inner: MutableLockMut<'a, T>,
}

impl<'a, T> Deref for PropertyWriteGuard<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for PropertyWriteGuard<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct Property<T> {
    data: Mutable<T>,
    bind_handles: Arc<RwLock<Vec<Subscription>>>,
}

impl<T: 'static + Clone + PartialEq> Property<T> {
    pub fn new<U: Into<T>>(val: U) -> Self {
        Property {
            data: Mutable::new(val.into()),
            bind_handles: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn binded_from(src_property: &Property<T>) -> Self {
        let new_property = Property {
            data: Mutable::new(src_property.get()),
            bind_handles: Arc::new(RwLock::new(Vec::new())),
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

    // &T like access
    pub fn read(&self) -> PropertyReadGuard<'_, T> {
        PropertyReadGuard {
            inner: self.data.lock_ref(),
        }
    }

    // &mut T like access
    pub fn write(&self) -> PropertyWriteGuard<'_, T> {
        PropertyWriteGuard {
            inner: self.data.lock_mut(),
        }
    }

    pub fn bind(&self, src_property: &Property<T>) {
        let handle = spawn_local(src_property.data.signal_cloned().for_each({
            let data = self.data.clone();
            move |v| {
                data.set_neq(v);
                async {}
            }
        }));
        self.bind_handles
            .write()
            .unwrap()
            .push(Subscription::SpawnLocal(handle));
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
        self.bind_handles
            .write()
            .unwrap()
            .push(Subscription::SpawnLocal(handle));
    }

    /// Creates a new Property with value computed from an expression
    /// that depends on multiple source properties.
    ///
    /// The resulting property automatically tracks changes to all source properties
    /// and recomputes its value when any of them changes.
    ///
    /// # Arguments
    ///
    /// * `expr_fn` - A closure that computes the value of the resulting property.
    ///               This closure must be `'static` and typically requires `move` semantics
    ///               with cloned source properties.
    /// * `subscriptions` - A vector of `PropertySubscription` objects created from
    ///                     source properties using `PropertySubscription::from_property()`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use fui_core::{Property, PropertySubscription};
    ///
    /// let first_name = Property::new("John".to_string());
    /// let last_name = Property::new("Doe".to_string());
    ///
    /// let full_name = Property::<String>::bind_from_expr(
    ///     {
    ///         let first_name = first_name.clone();
    ///         let last_name = last_name.clone();
    ///         move || format!("{} {}", first_name.get(), last_name.get())
    ///     },
    ///     vec![
    ///         PropertySubscription::from_property(&first_name),
    ///         PropertySubscription::from_property(&last_name),
    ///     ]
    /// );
    ///
    /// assert_eq!(full_name.get(), "John Doe");
    ///
    /// first_name.set("Jane".to_string());
    /// assert_eq!(full_name.get(), "Jane Doe");
    /// ```
    pub fn bind_from_expr<F, R>(expr_fn: F, subscriptions: Vec<PropertySubscription>) -> Property<R>
    where
        R: 'static + Clone + PartialEq,
        F: 'static + Fn() -> R,
    {
        let result_prop = Property::new(expr_fn());
        let f = Rc::new(expr_fn);

        // Subscribe to each source property
        for subscription in subscriptions {
            let f = f.clone();
            let target = result_prop.clone();
            let handle = subscription.subscribe(Rc::new(move || {
                target.set(f());
            }));
            result_prop.bind_handles.write().unwrap().push(handle);
        }

        result_prop
    }

    pub fn on_changed<F: 'static + FnMut(T)>(&self, mut f: F) -> Subscription {
        Subscription::SpawnLocal(spawn_local(self.data.signal_cloned().for_each(move |v| {
            f(v);
            async {}
        })))
    }

    /// Adds a subscription to the internal bind_handles collection.
    /// This is used by the ui! macro for automatic dependency tracking.
    pub fn add_bind_subscription(&self, subscription: Subscription) {
        self.bind_handles.write().unwrap().push(subscription);
    }
}

impl<T: 'static + Clone + PartialEq> Clone for Property<T> {
    fn clone(&self) -> Self {
        Property::<T> {
            data: self.data.clone(),
            bind_handles: self.bind_handles.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
        self.bind_handles.clone_from(&source.bind_handles);
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
        let mut old_value = Some(self.get());
        Some(Property::on_changed(self, move |v| {
            if let Some(old) = old_value.take() {
                f(VecDiff::RemoveAt { index: 0, value: old });
            }
            old_value = Some(v.clone());
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
        let mut old_value = self.get();
        Some(Property::on_changed(self, move |v| {
            f(VecDiff::Clear {
                values: old_value.take().map(|v| vec![v]).unwrap_or_default(),
            });
            old_value = v.clone();
            if let Some(v) = v {
                f(VecDiff::InsertAt { index: 0, value: v });
            }
        }))
    }
}
