use milim_2d::{
    Base, Engine, EngineContext, GameObject, GameObjectBase, Keycode, Scene, Sdl2Adapter,
    Transform2D,
};

#[derive(GameObject)]
#[game(subscribe(on_string: String))]
struct Player {
    #[game(base)]
    base: Base,
}
impl Player {
    pub fn on_string(&mut self, ctx: &mut EngineContext, event: &String) {
        println!("Posição do mouse: {:?}", ctx.input.mouse_position());
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
            ctx.send(self.base().id, "Event".to_string());
        }
    }
    fn on_message(&mut self, ctx: &mut EngineContext, msg: &Self::Message) {
        println!("Evento proprio recebido: {}", msg);
    }
}

#[derive(Scene)]
enum GameScene {
    Player(Player),
}

fn main() {
    let mut engine = Engine::<Sdl2Adapter, GameScene>::new("Milim Engine", 800, 600);

    engine.set_scene(GameScene::Player(Player {
        base: Base::new(Transform2D::new(0.0, 0.0)),
    }));
    engine.run();
}
