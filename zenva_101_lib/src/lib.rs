use gdnative::api::{Area2D, Camera2D};
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
        godot_print!("Hello, Godot, from Player!")
    }

    #[method]
    fn _physics_process(&mut self, #[base] base: &KinematicBody2D, delta: f32) {
        self.velocity.x = 0.0;

        let input = Input::godot_singleton();

        if input.is_action_pressed("move_right", false) {
            self.velocity.x += self.speed;
        } else if input.is_action_pressed("move_left", false) {
            self.velocity.x -= self.speed;
        }

        self.velocity = kb2d_move_and_slide(base, self.velocity, None);

        self.velocity.y += GRAVITY as f32 * delta;

        if input.is_action_just_pressed("jump", false) && base.is_on_floor() {
            self.velocity.y -= JUMP_FORCE as f32;
        }

        let sprite = unsafe { base.get_node_as::<Sprite>("Sprite").unwrap() };
        if self.velocity.x < 0.0 {
            sprite.set_flip_h(true);
        } else if self.velocity.x > 0.0 {
            sprite.set_flip_h(false);
        }
    }

    #[method]
    fn die(&self, #[base] base: &KinematicBody2D) {
        let st = unsafe { base.get_tree().unwrap().assume_safe() };
        st.reload_current_scene().unwrap();
    }
}

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Enemy {
    #[property(default = 100.0)]
    speed: f32,
    #[property(default = 100.0)]
    move_distance: f32,
    #[property(no_editor)]
    start_position: f32,
    #[property(no_editor)]
    target_position: f32,
}

#[methods]
impl Enemy {
    fn new(_base: &Area2D) -> Self {
        Enemy {
            speed: 0.0,
            move_distance: 0.0,
            start_position: 0.0,
            target_position: 0.0,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &Area2D) {
        self.start_position = base.position().x;
        self.target_position = self.start_position + self.move_distance;
        godot_print!("Hello, Godot, from Enemy!")
    }

    fn move_to(current: f32, to: f32, step: f32) -> f32 {
        let mut next = current;
        if next < to {
            next += step;
            if next > to {
                next = to;
            }
        } else {
            next -= step;
            if next < to {
                next = to;
            }
        }

        next
    }

    #[method]
    fn _process(&mut self, #[base] base: &Area2D, delta: f32) {
        base.set_position(Vector2::new(
            Enemy::move_to(base.position().x, self.target_position, self.speed * delta),
            base.position().y,
        ));

        if base.position().x == self.target_position {
            if self.target_position == self.start_position {
                self.target_position = base.position().x + self.move_distance;
            } else {
                self.target_position = self.start_position;
            }
        }
    }

    // TODO: get around snake case issues.
    #[method]
    fn _on_Enemy_body_entered(&self, #[base] base: &Area2D, body: Ref<KinematicBody2D>) {
        let body = unsafe { body.assume_safe() };
        if body.name() == "Player".into() {
            unsafe {
                body.call("die", &[]);
            }
        }
    }
}

#[derive(NativeClass)]
#[inherit(Camera2D)]
pub struct CameraController {}

#[methods]
impl CameraController {
    fn new(_base: &Camera2D) -> Self {
        CameraController {}
    }

    fn get_player(base: &Camera2D) -> Option<TRef<'static, KinematicBody2D>> {
        unsafe { base.get_node_as::<KinematicBody2D>("/root/MainScene/Player") }
    }

    #[method]
    fn _process(&mut self, #[base] base: &Camera2D, delta: f32) {
        let x = CameraController::get_player(base).unwrap().position().x;
        base.set_position(Vector2::new(x, base.position().y))
    }

    #[method]
    fn _ready(&self, #[base] _base: &Camera2D) {
        godot_print!("Hello, Godot, from Camera!")
    }
}

// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Enemy>();
    handle.add_class::<CameraController>();
}

// Creates entry-points of dyn lib.
godot_init!(init);
