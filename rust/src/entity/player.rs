use godot::prelude::*;
use godot::classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, Input};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    #[export]
    speed: f32,

    screen_size: Vector2,
    base: Base<Area2D>
}


#[godot_api]
impl Player {

    #[signal]
    pub fn hit();

    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<Node2D>) {
        self.base_mut().hide();

        self.signals().hit().emit();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_deferred("disabled", &true.to_variant());
    }

    #[func]
    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut().show();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    } 
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            speed: 400.0,
            screen_size: Vector2::ZERO,
            base
        }
    }

    fn ready(&mut self) {
        if let Some(viewport) = self.base().get_viewport() {
            self.screen_size = viewport.get_visible_rect().size;
        }
        // self.base_mut().hide();
    
        // conectar body_entered com nossa função customizada
        self.signals()
            .body_entered()
            .connect_self(Self::on_player_body_entered);
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut velocity = Vector2::ZERO;

        if input.is_action_pressed("move_right") {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left") {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("move_up") {
            velocity += Vector2::UP;
        }
        if input.is_action_pressed("move_down") {
            velocity += Vector2::DOWN;
        }

        let mut sprite = self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;

            
            let animated;
            if velocity.x != 0.0 {

                animated = "walk";
                sprite.set_flip_v(false);
                sprite.set_flip_h(velocity.x < 0.0)
            } else {
                animated = "up";

                sprite.set_flip_v(velocity.y > 0.0);
            }

            sprite.play_ex().name(animated).done();

        } else {
            sprite.stop();
        }
        
        let mut position = self.base().get_global_position();
    
        position += velocity * delta as f32;
        position = position.clamp(Vector2::ZERO, self.screen_size);

        self.base_mut().set_position(position);

    }

}
