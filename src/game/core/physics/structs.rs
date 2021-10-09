use bevy::ecs::component::Component;
use bevy::reflect::Reflect;

use crate::game::core::maths::structs::Vector2D;

#[derive(Default, Reflect, Component)]
pub struct PhysicsBody2D {
    pub velocity: Vector2D,
}
