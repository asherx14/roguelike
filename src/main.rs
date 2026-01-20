use bevy::prelude::*;
// use avian2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod player;
mod level;
mod core;
mod camera;

use crate::level::LevelPlugin;
use crate::player::PlayerPlugin;
use crate::camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins( ( LdtkPlugin,
                        CameraPlugin,
                        LevelPlugin, 
                        PlayerPlugin,
                    ))
        .run();
}

