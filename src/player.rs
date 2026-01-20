use bevy::prelude::*;
// use avian2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::camera;
use crate::level;
use crate::core::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let mut initial_timer = Timer::from_seconds(0.5, TimerMode::Once);
        initial_timer.tick(std::time::Duration::from_secs_f32(0.5));

        app.register_ldtk_entity::<PlayerBundle>("Player");
        app.insert_resource(PlayerMoveTimer { timer: initial_timer });

        app.add_systems(Update, ( handle_player_movement,
                                                    translate_grid_coords_entities,
                                                ));
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}


#[derive(Default, Component)]
pub struct Player;

#[derive(Resource)]
struct PlayerMoveTimer {
    timer: Timer,
}

fn handle_player_movement(
    player_single: Single<(&mut GridCoords, &mut Transform), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut move_timer: ResMut<PlayerMoveTimer>,
    level_walls: Res<level::LevelWalls>,
    mut messages: ResMut<Messages<camera::CenterCamera>>,
) {
    // advance the cooldown timer each frame
    move_timer.timer.tick(time.delta());
    
    // only allow movement when the timer has finished
    if !move_timer.timer.is_finished() {
        return;
    }
    
    let movement_direction = if input.pressed(KeyCode::KeyW) {
        GridCoords::new(0, 1)
    } else if input.pressed(KeyCode::KeyA) {
        GridCoords::new(-1, 0)
    } else if input.pressed(KeyCode::KeyS) {
        GridCoords::new(0, -1)
    } else if input.pressed(KeyCode::KeyD) {
        GridCoords::new(1, 0)
    } else {
        return;
    };

    let (mut player_grid_coords, player_transform) = player_single.into_inner();
    let destination = *player_grid_coords + movement_direction;

    if !level_walls.in_wall(&destination) {
        *player_grid_coords = destination;
        update_translation_from_grid_coords(player_transform.into_inner(), player_grid_coords.into_inner());

        messages.write(camera::CenterCamera);

        move_timer.timer.reset();
}
    
}

fn translate_grid_coords_entities(
    grid_coords_entities: Query<(&mut Transform, &mut GridCoords), (Without<Player>, Without<Camera2d>, Changed<GridCoords>)>,
) {
    grid_coords_entities.into_iter().for_each(|(transform, grid_coords)| {
        update_translation_from_grid_coords(transform.into_inner(), grid_coords.into_inner());
    });
}

fn update_translation_from_grid_coords(transform: &mut Transform, grid_coords: &GridCoords)  {
    transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
}
