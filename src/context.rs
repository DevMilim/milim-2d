use std::{any::Any, collections::VecDeque};

use indexmap::IndexMap;

use crate::{EngineCommands, GlobalEvent, Id, InputState, WindowGraphicsAdapter};

pub struct EngineContext<'a> {
    pub adapter: &'a mut dyn WindowGraphicsAdapter,
    pub input: &'a InputState,
    pub event_queue: &'a mut VecDeque<EngineCommands>,
    pub events: &'a mut VecDeque<GlobalEvent>,
    pub mailbox: &'a mut IndexMap<Id, Vec<Box<dyn Any>>>,
}
impl<'a> EngineContext<'a> {
    pub fn new(
        adapter: &'a mut dyn WindowGraphicsAdapter,
        input: &'a InputState,
        event_queue: &'a mut VecDeque<EngineCommands>,
        events: &'a mut VecDeque<GlobalEvent>,
        mailbox: &'a mut IndexMap<Id, Vec<Box<dyn Any>>>,
    ) -> Self {
        Self {
            adapter,
            input,
            event_queue,
            events,
            mailbox,
        }
    }
    pub fn send<T: 'static>(&mut self, id: Id, message: T) {
        let event = Box::new(message);
        self.mailbox.entry(id).or_default().push(event);
    }
    pub fn emit<T: 'static>(&mut self, event: T) {
        let event = GlobalEvent::Broadcast(Box::new(event));
        self.events.push_front(event);
    }
    pub fn quit(&mut self) {
        self.event_queue.push_front(EngineCommands::Quit);
    }
}
