use gdnative::api::{Area2D, AudioStream, AudioStreamPlayer2D, Camera2D};
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

    #[method]
    fn collect_coin(&mut self, #[base] base: &KinematicBody2D, value: i32) {
        self.score += value;
        let ui = Player::get_ui(base).unwrap();
        unsafe {
            ui.call("set_score_text", &[self.score.to_variant()]);
        }

        let audio_player = Player::get_audio_player(base).unwrap();
        unsafe {
            audio_player.call("play_coin_sfx", &[]);
        }
    }

    fn get_ui(base: &KinematicBody2D) -> Option<TRef<'static, Control>> {
        unsafe { base.get_node_as::<Control>("/root/MainScene/CanvasLayer/UI") }
    }

    fn get_audio_player(base: &KinematicBody2D) -> Option<TRef<'static, AudioStreamPlayer2D>> {
        unsafe { base.get_node_as::<AudioStreamPlayer2D>("/root/MainScene/AudioPlayer") }
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

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Coin {
    #[property(default = 90.0)]
    rotation_speed: f32,
    #[property(default = 1)]
    value: i32,
}

#[methods]
impl Coin {
    fn new(_base: &Area2D) -> Self {
        Coin {
            rotation_speed: 90.0,
            value: 1,
        }
    }

    #[method]
    fn _ready(&self, #[base] _base: &Area2D) {
        godot_print!("Hello, Godot, from Coin!")
    }

    #[method]
    fn _process(&self, #[base] base: &Area2D, delta: f32) {
        let next = self.rotation_speed * delta;
        base.set_rotation_degrees(base.rotation_degrees() + next as f64);
    }

    #[method]
    fn _on_coin_body_entered(&self, #[base] base: &Area2D, body: Ref<KinematicBody2D>) {
        godot_print!("In Coin Entered");
        let body = unsafe { body.assume_safe() };
        if body.name() == "Player".into() {
            unsafe {
                body.call("collect_coin", &[self.value.to_variant()]);
            }
        }

        base.queue_free();
    }
}

#[derive(NativeClass)]
#[inherit(Control)]
pub struct UI {}

#[methods]
impl UI {
    fn new(base: &Control) -> Self {
        UI {}
    }

    fn get_score_text(base: &Control) -> Option<TRef<'static, Label>> {
        unsafe { base.get_node_as::<Label>("ScoreText") }
    }

    #[method]
    fn _ready(&self, #[base] base: &Control) {
        godot_print!("Hello from UI");
        let score_text = UI::get_score_text(base).unwrap();
        score_text.set_text("0");
    }

    #[method]
    fn set_score_text(&self, #[base] base: &Control, score: i32) {
        let score_text = UI::get_score_text(base).unwrap();
        score_text.set_text(format!("{}", score));
    }
}

#[derive(NativeClass)]
#[inherit(AudioStreamPlayer2D)]
pub struct AudioPlayer {}

#[methods]
impl AudioPlayer {
    fn new(_base: &AudioStreamPlayer2D) -> Self {
        AudioPlayer {}
    }

    fn get_coin() -> Ref<AudioStream> {
        ResourceLoader::godot_singleton()
            .load("res://Audio/coin.tres", "AudioStream", false)
            .unwrap()
            .cast::<AudioStream>()
            .unwrap()
    }

    #[method]
    fn play_coin_sfx(&self, #[base] base: &AudioStreamPlayer2D) {
        base.set_stream(AudioPlayer::get_coin());
        base.play(0.0);
    }
}

// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Enemy>();
    handle.add_class::<CameraController>();
    handle.add_class::<Coin>();
    handle.add_class::<UI>();
    handle.add_class::<AudioPlayer>();
}

// Creates entry-points of dyn lib.
godot_init!(init);
