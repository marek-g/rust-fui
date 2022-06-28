use crate::{EventSubscription, JoinHandle};

pub enum Subscription {
    SpawnLocal(JoinHandle<()>),
    EventSubscription(EventSubscription),
}
