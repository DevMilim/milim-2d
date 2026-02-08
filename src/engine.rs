use std::time::Instant;

use sdl2::keyboard::Keycode;

use crate::{
    Base, Color, EngineContext, GameObjectDispatch, GlobalEvent, InputState, Transform2D,
    WindowConfig, WindowGraphicsAdapter,
};

pub enum EngineCommands {
    KeyDown(Keycode),
    KeyUp(Keycode),
    MousePosition(f32, f32),
    Send(GlobalEvent),
    Quit,
}

pub struct Engine<B: WindowGraphicsAdapter> {
    objects: Vec<Box<dyn GameObjectDispatch>>,
    adapter: B,
    base: Base,
    is_running: bool,
    input: InputState,
    event_queue: Vec<EngineCommands>,
}

impl<B: WindowGraphicsAdapter> Engine<B> {
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
            event_queue: Vec::new(),
        }
    }
    pub fn add_scene(&mut self, mut scene: impl GameObjectDispatch + 'static) {
        let mut ctx = EngineContext::new(&mut self.adapter, &self.input, &mut self.event_queue);
        scene.dispatch_start(&mut ctx, &mut self.base);
        self.objects.push(Box::new(scene));
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
            while let Some(event) = self.event_queue.pop() {
                match event {
                    EngineCommands::Quit => self.is_running = false,
                    EngineCommands::KeyDown(keycode) => self.input.update_key(keycode, true),
                    EngineCommands::KeyUp(keycode) => self.input.update_key(keycode, false),
                    EngineCommands::MousePosition(x, y) => self.input.set_mouse_position(x, y),
                    EngineCommands::Send(event) => {
                        let mut ctx = EngineContext::new(
                            &mut self.adapter,
                            &self.input,
                            &mut self.event_queue,
                        );
                        self.objects.dispatch_event(&mut ctx, &event);
                    }
                }
            }
            self.event_queue.clear();
            {
                let mut ctx =
                    EngineContext::new(&mut self.adapter, &self.input, &mut self.event_queue);
                self.objects
                    .dispatch_update(&mut ctx, &self.base, delta_time);

                while accumulator > FIXED_DT {
                    self.objects.dispatch_fixed_update(&mut ctx, &self.base);
                    accumulator -= FIXED_DT;
                }
                self.objects
                    .dispatch_late_update(&mut ctx, &self.base, delta_time);
            }

            self.adapter.clear(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            });
            {
                let mut ctx =
                    EngineContext::new(&mut self.adapter, &self.input, &mut self.event_queue);
                self.objects.dispatch_draw(&mut ctx, &self.base);
            }
            self.adapter.preset();
        }
    }
}
