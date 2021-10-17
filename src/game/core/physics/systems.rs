use bevy::prelude::*;
use rapier2d::prelude::*;

use crate::game::core::maths::structs::Transform2D;
use crate::game::core::physics::structs::*;

const SCALE: f32 = 20.0;

pub enum PhysicsGroups {
    Solid = 1 << 0,
    Player = 1 << 1,
}

pub fn physics_system_add(
    rigid_body_set: ResMut<RigidBodySetRes>,
    mut rigid_body_entities: ResMut<RigidBodyRemovedEntitiesRes>,
    //
    mut query: Query<(Entity, &RigidBodyHandle2D), Added<RigidBodyHandle2D>>,
) {
    for (entity, rigid_body_handle) in query.iter_mut() {
        if rigid_body_set.contains(rigid_body_handle.0) {
            rigid_body_entities.insert(entity, rigid_body_handle.0);
        }
    }
}

pub fn physics_system_step(
    gravity: Res<GravityRes>,
    integration_parameters: Res<IntegrationParametersRes>,
    //
    mut joint_set: ResMut<JointSetRes>,
    mut ccd_solver: ResMut<CCDSolverRes>,
    mut broad_phase: ResMut<BroadPhaseRes>,
    mut collider_set: ResMut<ColliderSetRes>,
    mut narrow_phase: ResMut<NarrowPhaseRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    mut island_manager: ResMut<IslandManagerRes>,
    mut query_pipeline: ResMut<QueryPipelineRes>,
    //
    mut query: Query<(&mut Transform2D, &RigidBodyHandle2D)>,
) {
    let hooks = ();
    let events = ();
    let mut physics_pipeline = PhysicsPipeline::new();

    physics_pipeline.step(
        &gravity,
        &integration_parameters,
        &mut island_manager,
        &mut broad_phase,
        &mut narrow_phase,
        &mut rigid_body_set,
        &mut collider_set,
        &mut joint_set,
        &mut ccd_solver,
        &hooks,
        &events,
    );

    query_pipeline.update(&island_manager, &rigid_body_set, &collider_set);

    for (mut transform, rigid_body_handle) in query.iter_mut() {
        let rigid_body = &rigid_body_set[rigid_body_handle.0];
        let rigid_body_rotation = rigid_body.rotation();
        let rigid_body_translation = rigid_body.translation();

        transform.rotation = rigid_body_rotation.angle();
        transform.position.set(
            rigid_body_translation.x * SCALE,
            rigid_body_translation.y * SCALE,
        );
    }
}

pub fn physics_system_remove(
    removed_entities: RemovedComponents<RigidBodyHandle2D>,
    //
    mut joint_set: ResMut<JointSetRes>,
    mut collider_set: ResMut<ColliderSetRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    mut island_manager: ResMut<IslandManagerRes>,
    mut rigid_body_entities: ResMut<RigidBodyRemovedEntitiesRes>,
) {
    for removed_entity in removed_entities.iter() {
        if let Some(&rigid_body_handle) = rigid_body_entities.get(&removed_entity) {
            rigid_body_set.remove(
                rigid_body_handle,
                &mut island_manager,
                &mut collider_set,
                &mut joint_set,
            );

            rigid_body_entities.remove(&removed_entity);
        }
    }
}
