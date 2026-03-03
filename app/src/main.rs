use milim_2d::{
    Base, Color, Component, Engine, EngineContext, GameObject, GameObjectBase, Keycode, Rect,
    Scene, Transform2D, TriggerEvent, Vector2,
    components::{body::Body2D, camera::Camera2D, collision::BoxCollider, sprite::Sprite2D},
};

#[derive(GameObject)]
#[game(connect(on_trigger: TriggerEvent))]
struct Player {
    #[game(base)]
    base: Base,
    #[game(component)]
    body: Body2D,
    #[game(component)]
    collision: BoxCollider,
    #[game(component)]
    camera: Camera2D,
    #[game(component)]
    sprite: Sprite2D,
}
impl Player {
    pub fn new(texture_id: usize) -> Self {
        Self {
            base: Base::new(Transform2D::new(0.0, 0.0)),
            collision: BoxCollider {
                key: 1,
                width: 32.0,
                height: 32.0,
                offset_x: 0.0,
                offset_y: 0.0,
                debug: true,
                layer: 1,
                mask: 1,
                is_sensor: false,
            },
            body: Body2D {
                velocity: Vector2::ZERO,
            },
            camera: Camera2D::new(Vector2::new(100.0, 100.0)),
            sprite: Sprite2D {
                texture_id,
                source: Rect::new(0, 0, 24, 24),
                offset: Vector2::ZERO,
                flip_h: false,
                flip_v: false,
                z_index: 6,
                color: Color::WHITE,
                scale: Vector2::new(1.0, 1.0),
            },
        }
    }
    pub fn on_trigger(&mut self, ctx: &mut EngineContext, event: &TriggerEvent) {
        println!("{:#?}", event);
    }
}
impl GameObject for Player {
    type Message = String;
    fn start(&mut self, ctx: &mut EngineContext) {
        println!("Hello")
    }
    fn fixed_update(&mut self, ctx: &mut EngineContext, delta: f32) {
        let direction = ctx.input.get_vetor("up", "down", "left", "right");
        let speed = 200.0;

        self.body.velocity = direction * speed;

        if direction.x > 0.0 {
            self.sprite.flip_h = false;
        } else if direction.x < 0.0 {
            self.sprite.flip_h = true
        }

        self.body.move_and_slide(ctx, &mut self.base, delta);
    }
    fn on_message(&mut self, ctx: &mut EngineContext, msg: &Self::Message) {
        // recebe um evento emitido com ctx.send(id, Self::Message)
    }
}

#[derive(GameObject)]
pub struct Wall {
    #[game(base)]
    base: Base,
    #[game(component)]
    collision: BoxCollider,
}
impl GameObject for Wall {
    type Message = ();
}

#[derive(GameObject)]
pub struct MainWorld {
    #[game(base)]
    base: Base,
    #[game(object)]
    wall: Wall,
    #[game(object)]
    player: Option<Player>,
}

impl GameObject for MainWorld {
    type Message = ();
    fn start(&mut self, ctx: &mut EngineContext) {
        let texture_id = ctx.resources.load_image("tilemap.png");
        self.player = Some(Player::new(texture_id))
    }
    fn fixed_update(&mut self, ctx: &mut EngineContext, delta: f32) {}
}

#[derive(Scene)]
enum GameScene {
    Main(MainWorld),
}

fn main() {
    let mut engine = Engine::<GameScene>::new("Milim Engine", 800, 600);

    engine.set_scene(GameScene::Main(MainWorld {
        base: Base::new(Transform2D::EMPTY),
        player: None,
        wall: Wall {
            base: Base::new(Transform2D::new(50.0, 200.0)),
            collision: BoxCollider {
                key: 1,
                width: 500.0,
                height: 50.0,
                offset_x: 0.0,
                offset_y: 0.0,
                debug: true,
                layer: 1,
                mask: 1,
                is_sensor: false,
            },
        },
    }));
    engine.input.map.bind_action("up", Keycode::Up);
    engine.input.map.bind_action("up", Keycode::W);
    engine.input.map.bind_action("down", Keycode::S);
    engine.input.map.bind_action("left", Keycode::A);
    engine.input.map.bind_action("right", Keycode::D);
    engine.run();
}
