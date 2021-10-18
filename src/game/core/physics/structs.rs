use bevy::prelude::*;
use bevy::reflect::impl_reflect_value;
use derive_more::{Deref, DerefMut};
use rapier2d::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Physics state resources

#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct GravityRes(pub Vector<f32>);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct JointSetRes(pub JointSet);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct CCDSolverRes(pub CCDSolver);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct BroadPhaseRes(pub BroadPhase);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct ColliderSetRes(pub ColliderSet);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct NarrowPhaseRes(pub NarrowPhase);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct RigidBodySetRes(pub RigidBodySet);
#[derive(Clone, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct IslandManagerRes(pub IslandManager);
#[derive(Clone, Deref, DerefMut, Default, Component, Serialize, Deserialize)]
pub struct QueryPipelineRes(pub QueryPipeline);
#[derive(Clone, Deref, DerefMut, Default, Component, Serialize, Deserialize)]
pub struct IntegrationParametersRes(pub IntegrationParameters);

impl Default for GravityRes {
    fn default() -> Self {
        Self(vector![0.0, -9.81])
    }
}
impl Default for JointSetRes {
    fn default() -> Self {
        Self(JointSet::new())
    }
}
impl Default for CCDSolverRes {
    fn default() -> Self {
        Self(CCDSolver::new())
    }
}
impl Default for BroadPhaseRes {
    fn default() -> Self {
        Self(BroadPhase::new())
    }
}
impl Default for ColliderSetRes {
    fn default() -> Self {
        Self(ColliderSet::new())
    }
}
impl Default for NarrowPhaseRes {
    fn default() -> Self {
        Self(NarrowPhase::new())
    }
}
impl Default for RigidBodySetRes {
    fn default() -> Self {
        Self(RigidBodySet::new())
    }
}
impl Default for IslandManagerRes {
    fn default() -> Self {
        Self(IslandManager::new())
    }
}
impl_reflect_value!(GravityRes(Serialize, Deserialize));
impl_reflect_value!(JointSetRes(Serialize, Deserialize));
impl_reflect_value!(CCDSolverRes(Serialize, Deserialize));
impl_reflect_value!(BroadPhaseRes(Serialize, Deserialize));
impl_reflect_value!(ColliderSetRes(Serialize, Deserialize));
impl_reflect_value!(NarrowPhaseRes(Serialize, Deserialize));
impl_reflect_value!(RigidBodySetRes(Serialize, Deserialize));
impl_reflect_value!(IslandManagerRes(Serialize, Deserialize));
impl_reflect_value!(QueryPipelineRes(Serialize, Deserialize));
impl_reflect_value!(IntegrationParametersRes(Serialize, Deserialize));

// Physics ECS components

#[derive(Clone, Debug, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct RigidBodyHandle2D(pub RigidBodyHandle);

impl Default for RigidBodyHandle2D {
    fn default() -> Self {
        Self(RigidBodyHandle::invalid())
    }
}
impl_reflect_value!(RigidBodyHandle2D(Serialize, Deserialize));

// Physics ECS components book-keeping

#[derive(Clone, Default, Deref, DerefMut, Component, Serialize, Deserialize)]
pub struct RigidBodyRemovedEntitiesRes(pub HashMap<Entity, RigidBodyHandle>);

impl_reflect_value!(RigidBodyRemovedEntitiesRes(Serialize, Deserialize));
