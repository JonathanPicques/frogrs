use bevy::prelude::*;

use crate::game::core::maths::structs::Transform2D;

pub fn sync_transform_system(mut query: Query<(&mut Transform, &Transform2D)>) {
    for (mut bevy_transform, transform) in query.iter_mut() {
        bevy_transform.rotation = Quat::from_rotation_z(transform.rotation);
        bevy_transform.translation.x = transform.position.x;
        bevy_transform.translation.y = transform.position.y;
    }
}
