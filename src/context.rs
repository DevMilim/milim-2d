use std::{any::Any, collections::VecDeque};

use indexmap::IndexMap;

use crate::{
    CollisionWorld, EngineCommands, GlobalEvent, Id, InputState, Vector2, WindowGraphicsAdapter,
};

pub struct EngineContext<'a> {
    pub adapter: &'a mut dyn WindowGraphicsAdapter,
    pub input: &'a InputState,
    pub event_queue: &'a mut VecDeque<EngineCommands>,
    pub events: &'a mut VecDeque<GlobalEvent>,
    pub mailbox: &'a mut IndexMap<Id, Vec<Box<dyn Any>>>,
    pub collision: &'a mut CollisionWorld,
    pub camera_pos: &'a mut Vector2,
}
impl<'a> EngineContext<'a> {
    pub fn new(
        adapter: &'a mut dyn WindowGraphicsAdapter,
        input: &'a InputState,
        event_queue: &'a mut VecDeque<EngineCommands>,
        events: &'a mut VecDeque<GlobalEvent>,
        mailbox: &'a mut IndexMap<Id, Vec<Box<dyn Any>>>,
        collision: &'a mut CollisionWorld,
        camera_pos: &'a mut Vector2,
    ) -> Self {
        Self {
            adapter,
            input,
            event_queue,
            events,
            mailbox,
            collision,
            camera_pos,
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
