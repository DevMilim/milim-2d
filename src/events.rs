use std::any::Any;

use uuid::Uuid;

pub enum GlobalEvent {
    Broadcast(Box<dyn Any>),
    Send { id: Uuid, message: Box<dyn Any> },
}
