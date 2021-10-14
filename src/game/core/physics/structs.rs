use bevy::prelude::*;
use bevy::reflect::impl_reflect_value;
use rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Serialize, Deserialize)]
pub struct PhysicsState {
    pub joint_set: JointSet,
    pub collider_set: ColliderSet,
    pub rigid_body_set: RigidBodySet,
    //
    pub gravity: Vector<f32>,
    pub ccd_solver: CCDSolver,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub island_manager: IslandManager,
    pub query_pipeline: QueryPipeline,
    pub integration_parameters: IntegrationParameters,
}

impl PhysicsState {
    pub fn get_rigid_body(&mut self, rigid_body_handle: &RigidBodyHandle2D) -> &mut RigidBody {
        &mut self.rigid_body_set[rigid_body_handle.0]
    }
}

impl Default for PhysicsState {
    fn default() -> Self {
        Self {
            joint_set: JointSet::new(),
            collider_set: ColliderSet::new(),
            rigid_body_set: RigidBodySet::new(),
            //
            gravity: vector![0.0, -9.81],
            ccd_solver: CCDSolver::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            island_manager: IslandManager::new(),
            query_pipeline: QueryPipeline::default(),
            integration_parameters: Default::default(),
        }
    }
}

impl_reflect_value!(PhysicsState(Serialize, Deserialize));

#[derive(Clone, Component, Serialize, Deserialize)]
pub struct RigidBodyHandle2D(pub RigidBodyHandle);

impl Default for RigidBodyHandle2D {
    fn default() -> Self {
        Self(RigidBodyHandle::invalid())
    }
}

impl_reflect_value!(RigidBodyHandle2D(Serialize, Deserialize));
