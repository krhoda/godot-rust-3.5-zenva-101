use gdnative::prelude::*;
use godot_sane_defaults::kb2d_move_and_slide;

const INITIAL_SPEED: f32 = 200.0;
const JUMP_FORCE: i32 = 600;
const GRAVITY: i32 = 800;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    #[property]
    pub score: i32,
    #[property]
    pub speed: f32,
    #[property]
    pub velocity: Vector2,
}

#[methods]
impl Player {
    fn new(_base: &KinematicBody2D) -> Self {
        Player {
            score: 0,
            speed: INITIAL_SPEED,
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    #[method]
    fn _ready(&self, #[base] _base: &KinematicBody2D) {
        godot_print!("Hello, Godot, from Rust!")
    }

    #[method]
    fn _physics_process(&mut self, #[base] base: &KinematicBody2D, delta: f32) {
        self.velocity.x = 0.0;

        let input = Input::godot_singleton();

        if input.is_action_pressed("move_right", false) {
            self.velocity.x += self.speed
        } else if input.is_action_pressed("move_left", false) {
            self.velocity.x -= self.speed
        };

        self.velocity = kb2d_move_and_slide(base, self.velocity, None);
    }
}

// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    // Register HelloWorld
    // handle.add_class::<HelloWorld>();
    handle.add_class::<Player>();
}

// Creates entry-points of dyn lib.
godot_init!(init);
