use godot::prelude::*;

use godot::classes::{AudioStreamPlayer2D, Marker2D, Node, PathFollow2D, Timer};

use crate::{Mob, Player, HUD};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    mob_scene: OnReady<Gd<PackedScene>>,
    player: OnReady<Gd<Player>>,
    hud: OnReady<Gd<HUD>>,

    score: u32,
    base: Base<Node>
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            mob_scene: OnReady::from_loaded("res://mob.tscn"),
            player: OnReady::from_node("Player"),
            hud: OnReady::from_node("HUD"),
            score: 0,
            base
        }
    }

    fn ready(&mut self) {
            
        let game = self.to_gd();

        self.player
            .signals()
            .hit()
            .connect_other(&game, Self::game_over);

        self.start_timer()
            .signals()
            .timeout()
            .connect_other(&game, Self::on_start_timer_timeout);

        self.mob_timer()
            .signals()
            .timeout()
            .connect_other(&game, Self::on_mob_timer_timeout);

        self.score_timer()
            .signals()
            .timeout()
            .connect_other(&game, Self::on_score_timer_timeout);

        self.hud
            .signals()
            .start_game()
            .connect_other(&game, Self::new_game);
    } 

}

#[godot_api]
impl Main {

    fn game_over(&mut self) {
        self.score_timer().stop();
        self.mob_timer().stop();

        self.music().stop();
        self.deathsound().play();

        self.hud.bind_mut().show_gameover();
    } 

    fn new_game(&mut self) {
        self.base().get_tree().unwrap().call_group("mobs", "queue_free", &[]);
        
        self.music().play();

        let start_pos = self.start_position().get_position();
        self.player.bind_mut().start(start_pos);
        
        self.start_timer().start();

        self.hud.bind_mut().set_score(self.score);
        self.hud.bind_mut().show_text("Get Ready");
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {

        let mut mob_spawn_location = self.base().get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");
        let mut mob_scene = self.mob_scene.instantiate_as::<Mob>();

        let progress = rand::random_range(u32::MIN..u32::MAX) as f32;
        mob_spawn_location.set_progress(progress);
        mob_scene.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + std::f32::consts::PI / 2.0;
        direction += rand::random_range((-std::f32::consts::PI / 4.0)..std::f32::consts::PI/4.0);
        mob_scene.set_rotation(direction);

        let velocity = Vector2::new(rand::random_range(150..200) as f32, 0.0);
        mob_scene.set_linear_velocity(velocity.rotated(direction));

        self.base_mut().add_child(&mob_scene);
    }

    #[func]
    fn on_start_timer_timeout(&mut self) {
        self.score_timer().start();
        self.mob_timer().start();
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
        self.hud.bind_mut().set_score(self.score);
    }

    fn music(&self) -> Gd<AudioStreamPlayer2D> {
        self.base().get_node_as("Music")
    }

    fn deathsound(&self) -> Gd<AudioStreamPlayer2D> {
        self.base().get_node_as("DeathSound")
    }

    fn start_position(&self) -> Gd<Marker2D> {
        self.base().get_node_as::<Marker2D>("StartPosition")
    }

    fn score_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("ScoreTimer")
    }

    fn start_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("StartTimer")
    }
    fn mob_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("MobTimer")
    }
}
