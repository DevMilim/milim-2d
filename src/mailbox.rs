use std::any::Any;

use crossbeam_channel::{Receiver, Sender};
use uuid::Uuid;

pub type AnyMessage = Box<dyn Any>;

pub struct Mailbox<M> {
    receiver: Receiver<M>,
}

impl<M> Mailbox<M> {
    pub fn new(id: Uuid) -> (Sender<M>, Self) {
        let (sender, receiver) = crossbeam_channel::unbounded::<M>();
        let mailbox = Self { receiver: receiver };
        (sender, mailbox)
    }
    pub fn drain(&self) -> impl Iterator<Item = M> + '_ {
        self.receiver.try_iter()
    }
}
