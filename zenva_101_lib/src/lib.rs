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
            godot_print!("In right");
            self.velocity.x += self.speed;
        } else if input.is_action_pressed("move_left", false) {
            godot_print!("In left");
            self.velocity.x -= self.speed;
        }

        self.velocity = kb2d_move_and_slide(base, self.velocity, None);

        self.velocity.y += GRAVITY as f32 * delta;

        // if input.is_action_just_pressed("jump", false) {
        if input.is_action_just_pressed("jump", false) && base.is_on_floor() {
            godot_print!("In Jump");
            self.velocity.y -= JUMP_FORCE as f32;
        }

        let sprite = unsafe { base.get_node_as::<Sprite>("Sprite").unwrap() };
        if self.velocity.x < 0.0 {
            sprite.set_flip_h(true);
        } else if self.velocity.x > 0.0 {
            sprite.set_flip_h(false);
        }
    }
}

// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

// Creates entry-points of dyn lib.
godot_init!(init);
