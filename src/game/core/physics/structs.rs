use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use bevy::reflect::impl_reflect_value;
use rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Serialize, Deserialize)]
pub struct GravityRes(pub Vector<f32>);
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct JointSetRes(pub JointSet);
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct CCDSolverRes(pub CCDSolver);
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct BroadPhaseRes(pub BroadPhase);
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct ColliderSetRes(pub ColliderSet);
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct NarrowPhaseRes(pub NarrowPhase);
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct RigidBodySetRes(pub RigidBodySet);
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct IslandManagerRes(pub IslandManager);
#[derive(Clone, Default, Component, Serialize, Deserialize)]
pub struct QueryPipelineRes(pub QueryPipeline);
#[derive(Clone, Default, Component, Serialize, Deserialize)]
pub struct IntegrationParametersRes(pub IntegrationParameters);

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

impl Deref for GravityRes {
    type Target = Vector<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for JointSetRes {
    type Target = JointSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for CCDSolverRes {
    type Target = CCDSolver;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for BroadPhaseRes {
    type Target = BroadPhase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for ColliderSetRes {
    type Target = ColliderSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for NarrowPhaseRes {
    type Target = NarrowPhase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for RigidBodySetRes {
    type Target = RigidBodySet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for IslandManagerRes {
    type Target = IslandManager;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for QueryPipelineRes {
    type Target = QueryPipeline;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for IntegrationParametersRes {
    type Target = IntegrationParameters;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GravityRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DerefMut for JointSetRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DerefMut for CCDSolverRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DerefMut for BroadPhaseRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DerefMut for ColliderSetRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DerefMut for NarrowPhaseRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DerefMut for RigidBodySetRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DerefMut for IslandManagerRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DerefMut for QueryPipelineRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DerefMut for IntegrationParametersRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

#[derive(Clone, Component, Serialize, Deserialize)]
pub struct RigidBodyHandle2D(pub RigidBodyHandle);

impl Deref for RigidBodyHandle2D {
    type Target = RigidBodyHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RigidBodyHandle2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for RigidBodyHandle2D {
    fn default() -> Self {
        Self(RigidBodyHandle::invalid())
    }
}

impl_reflect_value!(RigidBodyHandle2D(Serialize, Deserialize));
