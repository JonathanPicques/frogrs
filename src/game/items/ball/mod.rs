use bevy::prelude::*;
use rapier2d::prelude::*;

use crate::game::core::physics::structs::{ColliderSetRes, RigidBodyHandle2D, RigidBodySetRes};
use crate::game::core::{frame::structs::FrameCount, physics::systems::PLAYER_PHYSICS_GROUP};
use crate::game::core::{
    maths::structs::{Transform2D, Vector2D},
    physics::systems::SOLID_PHYSICS_GROUP,
};

#[derive(Default, Reflect, Component)]
pub struct Ball2D {}

pub fn ball_system(
    frame_count: Res<FrameCount>,
    mut commands: Commands,
    //
    mut query: Query<(Entity, &Ball2D)>,
) {
    if frame_count.frame > 144 * 1000 {
        for (entity, _) in query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn startup_ball_system(
    mut commands: Commands,
    mut collider_set: ResMut<ColliderSetRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
) {
    let transform = Transform2D::from_position(Vector2D::new(5.0, 3.0));

    commands
        .spawn_empty()
        .insert(Ball2D::default())
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        //
        .insert(create_ball_rigid_body(
            &transform,
            &mut collider_set,
            &mut rigid_body_set,
        ))
        .insert(transform);
}

pub fn create_ball_rigid_body(
    transform: &Transform2D,
    collider_set: &mut ColliderSetRes,
    rigid_body_set: &mut RigidBodySetRes,
) -> RigidBodyHandle2D {
    let rigid_body = RigidBodyBuilder::dynamic()
        .rotation(transform.rotation)
        .translation(vector![transform.position.x, transform.position.y])
        .build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);
    let rigid_body_collider = ColliderBuilder::ball(1.0)
        .restitution(0.7)
        .collision_groups(InteractionGroups::new(
            PLAYER_PHYSICS_GROUP,
            SOLID_PHYSICS_GROUP | PLAYER_PHYSICS_GROUP,
        ))
        .build();

    collider_set.insert_with_parent(rigid_body_collider, rigid_body_handle, rigid_body_set);

    info!("create_ball_rigid_body: handle: {:?}", rigid_body_handle);

    RigidBodyHandle2D(rigid_body_handle)
}
