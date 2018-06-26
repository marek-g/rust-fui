use EventSubscription;

pub struct BindingData<TSrc> {
    pub subscription: EventSubscription<TSrc>,
}

pub trait Binding {

}

impl<T> Binding for BindingData<T> {}
