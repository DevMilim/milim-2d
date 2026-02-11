use std::any::Any;

pub enum GlobalEvent {
    Broadcast(Box<dyn Any>),
}
