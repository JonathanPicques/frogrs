use bevy::prelude::*;
use rapier2d::prelude::*;

use crate::game::core::maths::structs::Transform2D;
use crate::game::core::physics::structs::{PhysicsState, RigidBodyHandle2D};

const SCALE: f32 = 20.0;

pub fn physics_system(
    mut physics_state: ResMut<PhysicsState>,
    mut query: Query<(&mut Transform2D, &RigidBodyHandle2D)>,
) {
    step_physics(&mut physics_state);

    for (mut transform, rigid_body_handle) in query.iter_mut() {
        let rigid_body = &physics_state.get_rigid_body(rigid_body_handle);
        let rigid_body_rotation = rigid_body.rotation();
        let rigid_body_translation = rigid_body.translation();

        transform.rotation = rigid_body_rotation.angle();
        transform.position.set(
            rigid_body_translation.x * SCALE,
            rigid_body_translation.y * SCALE,
        );
    }
}

fn step_physics(physics_state: &mut PhysicsState) {
    let hooks = ();
    let events = ();
    let mut physics_pipeline = PhysicsPipeline::new();

    physics_pipeline.step(
        &physics_state.gravity,
        &physics_state.integration_parameters,
        &mut physics_state.island_manager,
        &mut physics_state.broad_phase,
        &mut physics_state.narrow_phase,
        &mut physics_state.rigid_body_set,
        &mut physics_state.collider_set,
        &mut physics_state.joint_set,
        &mut physics_state.ccd_solver,
        &hooks,
        &events,
    );

    physics_state.query_pipeline.update(
        &physics_state.island_manager,
        &physics_state.rigid_body_set,
        &physics_state.collider_set,
    );
}
