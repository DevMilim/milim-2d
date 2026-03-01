use std::{any::Any, collections::VecDeque, time::Instant};

use indexmap::IndexMap;
use sdl2::{keyboard::Keycode, pixels::Color};

use crate::{
    Base, CollisionWorld, EngineContext, GameObjectDispatch, GlobalEvent, Id, InputState,
    Resources, Sdl2Adapter, Transform2D, TriggerEvent, TriggerKind, Vector2, WindowConfig,
    WindowGraphicsAdapter,
};

const FIXED_DT: f32 = 1.0 / 60.0;

#[derive(Debug, Clone)]
pub enum EngineCommands {
    KeyDown(Keycode),
    KeyUp(Keycode),
    MousePosition(f32, f32),
    Quit,
}

pub struct Engine<S: GameObjectDispatch> {
    objects: Vec<S>,
    adapter: Sdl2Adapter,
    base: Base,
    is_running: bool,
    pub input: InputState,
    event_queue: VecDeque<EngineCommands>,
    events: VecDeque<GlobalEvent>,
    mailbox: IndexMap<Id, Vec<Box<dyn Any>>>,
    physics: CollisionWorld,
    camera_pos: Vector2,
    resources: Resources,
    fixed_delta_time: f32,
}

impl<S: GameObjectDispatch> Engine<S> {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let adapter = Sdl2Adapter::new(WindowConfig {
            title: title.to_string(),
            width,
            height,
        });
        Self {
            resources: Resources::new(adapter.canvas.texture_creator()),
            objects: Vec::new(),
            adapter,
            base: Base::new(Transform2D::new(0.0, 0.0)),
            is_running: true,
            input: InputState::new(),
            event_queue: VecDeque::new(),
            events: VecDeque::new(),
            mailbox: IndexMap::new(),
            physics: CollisionWorld::new(),
            camera_pos: Vector2::ZERO,
            fixed_delta_time: FIXED_DT,
        }
    }
    pub fn push(&mut self, mut scene: S) {
        let mut ctx = EngineContext::new(
            &mut self.adapter,
            &self.input,
            &mut self.event_queue,
            &mut self.events,
            &mut self.mailbox,
            &mut self.physics,
            &mut self.camera_pos,
            &mut self.resources,
            &mut self.fixed_delta_time,
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
            &mut self.physics,
            &mut self.camera_pos,
            &mut self.resources,
            &mut self.fixed_delta_time,
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
            &mut self.physics,
            &mut self.camera_pos,
            &mut self.resources,
            &mut self.fixed_delta_time,
        );
        scene.dispatch_start(&mut ctx, &mut self.base);
        self.objects.clear();
        self.objects.push(scene);
    }
    pub fn run(&mut self) {
        let mut last = Instant::now();
        let mut accumulator = 0.0_f32;

        const MAX_ACCUM: f32 = 0.5;

        while self.is_running {
            self.input.clear_frame_data();

            self.adapter.pool_events(&mut self.event_queue);
            self.process_commands();

            let now = Instant::now();
            let mut delta_time = (now - last).as_secs_f32();
            last = now;

            if delta_time >= MAX_ACCUM {
                delta_time = MAX_ACCUM;
            }

            accumulator += delta_time;
            let mut ctx = EngineContext::new(
                &mut self.adapter,
                &self.input,
                &mut self.event_queue,
                &mut self.events,
                &mut self.mailbox,
                &mut self.physics,
                &mut self.camera_pos,
                &mut self.resources,
                &mut self.fixed_delta_time,
            );

            if let Some(obj) = self.objects.last_mut() {
                obj.dispatch_update(&mut ctx, &self.base, delta_time)
            }
            while accumulator > FIXED_DT {
                if let Some(obj) = self.objects.last_mut() {
                    obj.dispatch_fixed_update(&mut ctx, &self.base)
                }
                accumulator -= FIXED_DT;
            }

            if let Some(obj) = self.objects.last_mut() {
                obj.dispatch_late_update(&mut ctx, &self.base, delta_time)
            }
            Self::flush_messages_and_events(&mut self.objects, &mut ctx);
            ctx.collision.step();

            for (a, b) in ctx.collision.get_entered_pairs() {
                let da = ctx.collision.colliders.get(&a).unwrap();
                let db = ctx.collision.colliders.get(&b).unwrap();

                if da.is_sensor {
                    let ev = TriggerEvent {
                        owner: b.id,
                        sensor: a.clone(),
                        kind: TriggerKind::Enter,
                    };
                    ctx.events.push_back(GlobalEvent::Broadcast(Box::new(ev)));
                }
                if db.is_sensor {
                    let ev = TriggerEvent {
                        owner: a.id,
                        sensor: b.clone(),
                        kind: TriggerKind::Enter,
                    };
                    ctx.events.push_back(GlobalEvent::Broadcast(Box::new(ev)));
                }
            }
            for (a, b) in ctx.collision.get_exited_pairs() {
                let da = ctx.collision.colliders.get(&a).unwrap();
                let db = ctx.collision.colliders.get(&b).unwrap();

                if da.is_sensor {
                    let ev = TriggerEvent {
                        owner: b.id,
                        sensor: a.clone(),
                        kind: TriggerKind::Exit,
                    };
                    ctx.events.push_back(GlobalEvent::Broadcast(Box::new(ev)));
                }
                if db.is_sensor {
                    let ev = TriggerEvent {
                        owner: a.id,
                        sensor: b.clone(),
                        kind: TriggerKind::Exit,
                    };
                    ctx.events.push_back(GlobalEvent::Broadcast(Box::new(ev)));
                }
            }

            ctx.adapter.set_camera_pos(&ctx.camera_pos);

            ctx.adapter.clear(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            });
            {
                if let Some(obj) = self.objects.last_mut() {
                    obj.dispatch_draw(&mut ctx, &self.base)
                } else {
                    self.quit();
                }
            }

            self.process_commands();
            self.adapter.present(&mut self.resources.textures);
            self.physics.commit();
        }
    }

    pub fn quit(&mut self) {
        self.is_running = false
    }
    pub fn flush_messages_and_events(objects: &mut Vec<S>, ctx: &mut EngineContext) {
        for _ in 0..10 {
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
        while let Some(event) = self.event_queue.pop_front() {
            match event {
                EngineCommands::Quit => self.is_running = false,
                EngineCommands::KeyDown(keycode) => self.input.update_key(keycode, true),
                EngineCommands::KeyUp(keycode) => self.input.update_key(keycode, false),
                EngineCommands::MousePosition(x, y) => self.input.set_mouse_position(x, y),
            }
        }
    }
}
