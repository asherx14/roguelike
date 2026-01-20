//use bevy::{gizmos::grid, prelude::*, render::extract_component::ExtractComponent};
// use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::core::*;
use crate::player;
use crate::camera;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0));
        app.register_ldtk_entity::<HoleBundle>("Hole");
        app.register_ldtk_int_cell::<WallBundle>(1);
        app.init_resource::<LevelWalls>();
        app.init_resource::<Messages<LoadLevel>>();

        app.add_systems(Startup, load_level);
        // run wall caching during Update so LevelEvent messages emitted on spawn are readable
        app.add_systems(Update, (
            cache_wall_locations,
            check_goal,
            load_next_level,
        ));
    }
}

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}
#[derive(Component, Default)]
struct Portal;

#[derive(Default, Bundle, LdtkEntity)]
struct HoleBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    portal: Portal,
}

fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_messages: MessageReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
    ldtk_project_entities: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) -> Result {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single()?)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let wall_locations = walls.iter().copied().collect();

            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_walls = new_level_walls;
        }
    }
    Ok(())
}

use std::collections::HashSet;

#[derive(Default, Resource)]
pub struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelWalls {
    pub fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        // If level dimensions are zero the level hasn't been cached yet;
        // don't treat out-of-bounds as walls in that case (only explicit wall tiles).
        if self.level_width == 0 || self.level_height == 0 {
            return self.wall_locations.contains(grid_coords);
        }

        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}
// Obscure naming, should be something like init_level
fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut messages: ResMut<Messages<camera::CenterCamera>>
) {

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk").into(),
        ..Default::default()
    });

    messages.write(camera::CenterCamera);
}
#[derive(Message)]
struct LoadLevel {
    level_idx: usize,
}
fn check_goal(
    player_coords: Single<&GridCoords, (With<player::Player>, Changed<GridCoords>)>,
    portal_query: Query<(&GridCoords, &EntityInstance), With<Portal>>,
    mut messages: ResMut<Messages<LoadLevel>>,
) {
    for (grid_coords, entity_instance) in portal_query.iter() {
    println!("grid_coords: {:?}", grid_coords);
    println!("player_coords: {:?}", *player_coords);
    println!("field_instances: {:?}", entity_instance.field_instances);


        if grid_coords == *player_coords {
            println!("grid coords match player coords");

            entity_instance.field_instances.iter().for_each(|field_instance| {

                if field_instance.identifier == "level_idx" {

                    match field_instance.value
                    {  
                        FieldValue::Int(option_level_idx) => {
                            let level_message = LoadLevel { level_idx:
                                option_level_idx.expect("level_idx is None for portal") as usize};
                            messages.write(level_message);
                        }
                        _ => eprintln!("level_idx not implemented for portal {:?}", field_instance),
                    }
                }   
            });
        }
    }
}

fn load_next_level(
    mut level_selection: ResMut<LevelSelection>,
    messages: ResMut<Messages<LoadLevel>>,
) {
    let mut cursor = messages.get_cursor();

    for message in cursor.read(&messages) {
        match &mut *level_selection {
            LevelSelection::Indices(indices) => indices.level = message.level_idx,
            _ => panic!("Level selection lacks indeces"),
        }
    }
}


