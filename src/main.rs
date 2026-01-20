use bevy::prelude::*;
// use avian2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod player;
mod level;
mod core;
mod window;

use crate::level::LevelPlugin;
use crate::player::PlayerPlugin;
use crate::window::RoguelikeWindowPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins( ( LdtkPlugin,
                        LevelPlugin, 
                        PlayerPlugin,
                        RoguelikeWindowPlugin,
                    ))
        .run();
}

