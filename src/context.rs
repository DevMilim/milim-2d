use std::{any::Any, collections::VecDeque};

use indexmap::IndexMap;
use sdl2::pixels::Color;

use crate::{
    CollisionWorld, DrawCommand, DrawCommandType, DrawData, EngineCommands, GlobalEvent, Id,
    InputState, Resources, Vector2, WindowGraphicsAdapter,
};

pub struct EngineContext<'a> {
    pub adapter: &'a mut dyn WindowGraphicsAdapter,
    pub input: &'a InputState,
    pub event_queue: &'a mut VecDeque<EngineCommands>,
    pub events: &'a mut VecDeque<GlobalEvent>,
    pub mailbox: &'a mut IndexMap<Id, Vec<Box<dyn Any>>>,
    pub collision: &'a mut CollisionWorld,
    pub camera_pos: &'a mut Vector2,
    pub resources: &'a mut Resources,
}
impl<'a> EngineContext<'a> {
    pub fn new(
        adapter: &'a mut impl WindowGraphicsAdapter,
        input: &'a InputState,
        event_queue: &'a mut VecDeque<EngineCommands>,
        events: &'a mut VecDeque<GlobalEvent>,
        mailbox: &'a mut IndexMap<Id, Vec<Box<dyn Any>>>,
        collision: &'a mut CollisionWorld,
        camera_pos: &'a mut Vector2,
        resources: &'a mut Resources,
    ) -> Self {
        Self {
            adapter,
            input,
            event_queue,
            events,
            mailbox,
            collision,
            camera_pos,
            resources,
        }
    }
    pub fn send<T: 'static>(&mut self, id: Id, message: T) {
        let event = Box::new(message);
        self.mailbox.entry(id).or_default().push(event);
    }
    pub fn emit<T: 'static>(&mut self, event: T) {
        let event = GlobalEvent::Broadcast(Box::new(event));
        self.events.push_back(event);
    }
    pub fn emit_targeted<T: 'static>(&mut self, id: Id, event: T) {
        let event = GlobalEvent::Targeted(id, Box::new(event));
        self.events.push_back(event);
    }
    pub fn quit(&mut self) {
        self.event_queue.push_back(EngineCommands::Quit);
    }
    pub fn draw_sprite(&mut self, image: usize, pos: Vector2, z_index: i32) {
        let cmd = DrawCommand {
            cmd_type: DrawCommandType::Sprite,
            material: DrawData {
                image,
                pos,
                ..Default::default()
            },
        };
        self.adapter.draw(cmd, z_index);
    }
    pub fn draw_rect(&mut self, pos: Vector2, size: Vector2, color: Color, z_index: i32) {
        let cmd = DrawCommand {
            cmd_type: DrawCommandType::Rect,
            material: DrawData {
                pos,
                size,
                color,
                ..Default::default()
            },
        };
        self.adapter.draw(cmd, z_index);
    }
}
