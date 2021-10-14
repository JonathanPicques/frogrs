use bevy::prelude::*;

use crate::game::core::maths::structs::Transform2D;

pub fn sync_transform_system(
    mut query: Query<(&mut Transform, &Transform2D), Changed<Transform2D>>,
) {
    for (mut bevy_transform, game_transform) in query.iter_mut() {
        bevy_transform.rotation = Quat::from_rotation_z(game_transform.rotation);
        bevy_transform.translation.x = game_transform.position.x;
        bevy_transform.translation.y = game_transform.position.y;
    }
}
