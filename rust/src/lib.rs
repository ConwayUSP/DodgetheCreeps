use godot::prelude::*;


mod entity;
mod main_game;
mod hud;

pub use entity::*;
pub use hud::*;

struct GameExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}
