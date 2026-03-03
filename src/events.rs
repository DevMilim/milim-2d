use std::any::Any;

use crate::{ColliderKey, Id};

pub enum GlobalEvent {
    Broadcast(Box<dyn Any>),
    Targeted(Id, Box<dyn Any>),
}
#[derive(Debug, Clone)]
pub enum TriggerKind {
    Enter,
    Exit,
}
#[derive(Debug, Clone)]
pub struct TriggerEvent {
    pub owner: Id,
    pub sensor: ColliderKey,
    pub kind: TriggerKind,
}
