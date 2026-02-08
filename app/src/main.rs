use milim_2d::{
    Base, Component, DrawCommands, Engine, EngineContext, GameObject, GameObjectBase, Keycode,
    Sdl2Adapter, Transform2D, Vector2,
};

pub struct Sprite {
    scale: f32,
}

impl Component for Sprite {
    type Message = ();
    fn start(&mut self, ctx: &mut EngineContext, base: &mut Base) {
        ctx.adapter.load_image("player", "./tilemap.png");
    }
    fn draw(&mut self, ctx: &mut EngineContext, base: &Base) {
        ctx.adapter.draw(
            DrawCommands::DrawImage {
                name: "player",
                x: base.transform.global_position.x,
                y: base.transform.global_position.y,
                scale: self.scale,
                image_x: 2.0,
                image_y: 6.0,
                image_width: 24.0,
                image_height: 24.0,
                angle: 0.0,
                flip_h: false,
                flip_v: false,
            },
            base.z_index,
        );
    }
}

#[derive(GameObject)]
#[game(subscribe(on_string: String))]
struct Player {
    #[game(base)]
    base: Base,
    #[game(component)]
    sprite: Sprite,
}
impl Player {
    fn on_string(&mut self, ctx: &mut EngineContext, event: &String) {
        println!("Evento global recebido: {}", event)
    }
}
impl GameObject for Player {
    type Message = String;
    fn start(&mut self, ctx: &mut EngineContext) {
        println!("Hello")
    }
    fn update(&mut self, ctx: &mut EngineContext, delta: f32) {
        let vel = 100.0 * delta;
        let base = &mut self.base_mut().transform.position;
        if ctx.input.is_key_pressed(Keycode::D) {
            base.x += vel;
        }
        if ctx.input.is_key_pressed(Keycode::A) {
            base.x -= vel;
        }
        if ctx.input.is_key_pressed(Keycode::W) {
            base.y -= vel;
        }
        if ctx.input.is_key_pressed(Keycode::S) {
            base.y += vel;
        }
        if ctx.input.is_key_just_pressed(Keycode::Space) {
            ctx.emit("Mensagem".to_string());
        }
    }
    fn on_event(&mut self, ctx: &mut EngineContext, msg: &Self::Message) {
        println!("Evento proprio recebido: {}", msg);
    }
}

fn main() {
    let mut engine = Engine::<Sdl2Adapter>::new("Milim Engine", 800, 600);

    engine.add_scene(Player {
        base: Base::new(Transform2D::new(0.0, 0.0)),
        sprite: Sprite { scale: 10.0 },
    });
    engine.run();
}
