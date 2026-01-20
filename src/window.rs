use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::core::*;
pub struct RoguelikeWindowPlugin;
impl Plugin for RoguelikeWindowPlugin {
    fn build(&self, app: &mut App) {
            app.init_resource::<WindowProperties>();
            app.add_systems(Update, handle_window_resize);
}

}

#[derive(Default, Resource)]
pub struct WindowProperties {
    pub size: Vec2,
    pub proportions: Vec2,
}

//impl Default for WindowPrope

fn handle_window_resize(
    mut camera2d_transform: Single<&mut Transform, With<Camera2d>>,
    player_coords: Single<&GridCoords, With<crate::player::Player>>,
) {
    camera2d_transform.translation = Vec3::new(player_coords.x as f32 * GRID_SIZE as f32,
    player_coords.y as f32 * GRID_SIZE as f32, 0.0);
    camera2d_transform.scale = Vec3::splat(1.0 / SCALE);
}