use std::{
    any::Any,
    collections::{HashMap, VecDeque},
    time::Instant,
};

use indexmap::IndexMap;
use sdl2::keyboard::Keycode;

use crate::{
    Base, Color, EngineContext, GameObjectDispatch, GlobalEvent, Id, InputState, Transform2D,
    WindowConfig, WindowGraphicsAdapter,
};

pub enum EngineCommands {
    KeyDown(Keycode),
    KeyUp(Keycode),
    MousePosition(f32, f32),
    Quit,
}

pub struct Engine<B: WindowGraphicsAdapter, S: GameObjectDispatch> {
    objects: Vec<S>,
    adapter: B,
    base: Base,
    is_running: bool,
    input: InputState,
    event_queue: VecDeque<EngineCommands>,
    events: VecDeque<GlobalEvent>,
    mailbox: IndexMap<Id, Vec<Box<dyn Any>>>,
}

impl<B: WindowGraphicsAdapter, S: GameObjectDispatch> Engine<B, S> {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            objects: Vec::new(),
            adapter: B::new(WindowConfig {
                title: title.to_string(),
                width,
                height,
            }),
            base: Base::new(Transform2D::new(0.0, 0.0)),
            is_running: true,
            input: InputState::new(),
            event_queue: VecDeque::new(),
            events: VecDeque::new(),
            mailbox: IndexMap::new(),
        }
    }
    pub fn push(&mut self, mut scene: S) {
        let mut ctx = EngineContext::new(
            &mut self.adapter,
            &self.input,
            &mut self.event_queue,
            &mut self.events,
            &mut self.mailbox,
        );
        scene.dispatch_start(&mut ctx, &mut self.base);
        self.objects.push(scene);
    }
    pub fn pop(&mut self) {
        let mut ctx = EngineContext::new(
            &mut self.adapter,
            &self.input,
            &mut self.event_queue,
            &mut self.events,
            &mut self.mailbox,
        );
        match self.objects.pop() {
            Some(mut scene) => scene.dispatch_destroy(&mut ctx),
            None => {}
        };
    }
    pub fn set_scene(&mut self, mut scene: S) {
        let mut ctx = EngineContext::new(
            &mut self.adapter,
            &self.input,
            &mut self.event_queue,
            &mut self.events,
            &mut self.mailbox,
        );
        scene.dispatch_start(&mut ctx, &mut self.base);
        self.objects.clear();
        self.objects.push(scene);
    }
    pub fn run(&mut self) {
        let mut last = Instant::now();
        let mut accumulator = 0.0_f32;

        const FIXED_DT: f32 = 1.0 / 60.0;
        const MAX_ACCUM: f32 = 0.5;

        while self.is_running {
            self.input.clear_frame_data();

            let now = Instant::now();
            let mut delta_time = (now - last).as_secs_f32();
            last = now;

            if delta_time >= MAX_ACCUM {
                delta_time = MAX_ACCUM;
            }

            accumulator += delta_time;

            let adapter_events = self.adapter.pool_events();
            self.event_queue.extend(adapter_events);
            self.process_commands();

            self.event_queue.clear();
            {
                let mut ctx = EngineContext::new(
                    &mut self.adapter,
                    &self.input,
                    &mut self.event_queue,
                    &mut self.events,
                    &mut self.mailbox,
                );

                Self::flush_messages_and_events(&mut self.objects, &mut ctx, 10);

                if let Some(obj) = self.objects.last_mut() {
                    obj.dispatch_update(&mut ctx, &self.base, delta_time)
                }
                Self::flush_messages_and_events(&mut self.objects, &mut ctx, 10);
                while accumulator > FIXED_DT {
                    if let Some(obj) = self.objects.last_mut() {
                        obj.dispatch_fixed_update(&mut ctx, &self.base)
                    }
                    accumulator -= FIXED_DT;
                    Self::flush_messages_and_events(&mut self.objects, &mut ctx, 10);
                }

                if let Some(obj) = self.objects.last_mut() {
                    obj.dispatch_late_update(&mut ctx, &self.base, delta_time)
                }
                Self::flush_messages_and_events(&mut self.objects, &mut ctx, 10);
            }
            self.adapter.clear(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            });
            {
                let mut ctx = EngineContext::new(
                    &mut self.adapter,
                    &self.input,
                    &mut self.event_queue,
                    &mut self.events,
                    &mut self.mailbox,
                );
                if let Some(obj) = self.objects.last_mut() {
                    obj.dispatch_draw(&mut ctx, &self.base)
                } else {
                    self.quit();
                }
            }
            self.process_commands();
            self.adapter.preset();
        }
    }
    pub fn quit(&mut self) {
        self.is_running = false
    }
    pub fn flush_messages_and_events(objects: &mut Vec<S>, ctx: &mut EngineContext, loops: i32) {
        for _ in 0..loops {
            let mut something_processed = false;
            while let Some(event) = &ctx.events.pop_back() {
                something_processed = true;
                if let Some(obj) = objects.last_mut() {
                    obj.dispatch_event(ctx, event);
                }
            }
            if !ctx.mailbox.is_empty() {
                something_processed = true;
                if let Some(obj) = objects.last_mut() {
                    obj.dispatch_message(ctx);
                }
            }
            if !something_processed {
                break;
            }
        }
    }
    pub fn process_commands(&mut self) {
        while let Some(event) = self.event_queue.pop_back() {
            match event {
                EngineCommands::Quit => self.is_running = false,
                EngineCommands::KeyDown(keycode) => self.input.update_key(keycode, true),
                EngineCommands::KeyUp(keycode) => self.input.update_key(keycode, false),
                EngineCommands::MousePosition(x, y) => self.input.set_mouse_position(x, y),
            }
        }
    }
}
