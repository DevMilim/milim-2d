use uuid::Uuid;

use crate::{EngineCommands, GlobalEvent, InputState, WindowGraphicsAdapter};

pub struct EngineContext<'a> {
    pub adapter: &'a mut dyn WindowGraphicsAdapter,
    pub input: &'a InputState,
    pub event_queue: &'a mut Vec<EngineCommands>,
}
impl<'a> EngineContext<'a> {
    pub fn new(
        adapter: &'a mut dyn WindowGraphicsAdapter,
        input: &'a InputState,
        event_queue: &'a mut Vec<EngineCommands>,
    ) -> Self {
        Self {
            adapter,
            input,
            event_queue,
        }
    }
    pub fn send<T: 'static>(&mut self, id: Uuid, message: T) {
        let event = GlobalEvent::Send {
            id,
            message: Box::new(message),
        };
        self.event_queue.push(EngineCommands::Send(event));
    }
    pub fn emit<T: 'static>(&mut self, event: T) {
        let event = GlobalEvent::Broadcast(Box::new(event));
        self.event_queue.push(EngineCommands::Send(event));
    }
}
