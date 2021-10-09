use bevy::prelude::Query;

use crate::game::core::maths::structs::Transform2D;

use super::structs::PhysicsBody2D;

pub fn physics_system(mut query: Query<(&mut Transform2D, &PhysicsBody2D)>) {
    for (mut transform, physics_body) in query.iter_mut() {
        transform.position.add_x(physics_body.velocity.x);
        transform.position.add_y(physics_body.velocity.y);
    }
}
