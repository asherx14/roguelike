use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::core::*;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages<CenterCamera>>();

        app.add_systems(Startup, init_camera);
        app.add_systems(Update, center_camera);
}

}

#[derive(Message)]
pub struct CenterCamera;

pub fn center_camera(
    mut camera2d_transform: Single<&mut Transform, With<Camera2d>>,
    player_coords: Single<&GridCoords, With<crate::player::Player>>,
    messages: ResMut<Messages<CenterCamera>>,
) {
    let mut cursor = messages.get_cursor();

    cursor.read(&messages).into_iter().for_each(| _ | {
        camera2d_transform.translation = Vec3::new(player_coords.x as f32 * GRID_SIZE as f32,
                                                   player_coords.y as f32 * GRID_SIZE as f32, 0.0);
    });

}

pub fn init_camera(
    mut commands: Commands,
) {
    commands.spawn( (Camera2d,
                    Transform::from_scale(Vec3::splat(1.0 / SCALE)),
                    ) );
}