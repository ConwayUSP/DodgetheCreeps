use godot::prelude::*;
use godot::classes::{AnimatedSprite2D, IRigidBody2D, RigidBody2D};

use rand::seq::{IndexedRandom};

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {

    base: Base<RigidBody2D>
}

#[godot_api]
impl Mob {
    #[func]
    fn on_visibility_screen_exit(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IRigidBody2D for Mob {

    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn ready(&mut self) {
        
        let mut sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let anime_names = sprite.get_sprite_frames().unwrap().get_animation_names().to_vec();
        let mut rgn = rand::rng();
        let animation = anime_names.choose(&mut rgn).unwrap();

        sprite.set_animation(animation.arg());

        sprite.play();
    }
}
