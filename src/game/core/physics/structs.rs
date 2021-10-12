use bevy::prelude::*;
use bevy::reflect::serde::Serializable;
use bevy::reflect::{FromType, GetTypeRegistration, ReflectMut, ReflectRef, TypeRegistration};
use rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Reflect, Component)]
pub struct PhysicsBody2D {
    pub handle: (u32, u32),
}

#[derive(Serialize, Deserialize, Component, Clone)]
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
    pub fn get_rigid_body(&mut self, rigid_body: &PhysicsBody2D) -> &mut RigidBody {
        &mut self.rigid_body_set
            [RigidBodyHandle::from_raw_parts(rigid_body.handle.0, rigid_body.handle.1)]
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

unsafe impl Reflect for PhysicsState {
    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn any(&self) -> &dyn std::any::Any {
        self
    }

    fn any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn apply(&mut self, value: &dyn Reflect) {
        let value = value.any();
        if let Some(value) = value.downcast_ref::<Self>() {
            *self = value.clone();
        } else {
            panic!("Value is not a {}.", std::any::type_name::<Self>());
        }
    }

    fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>> {
        *self = value.take()?;
        Ok(())
    }

    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
        ReflectRef::Value(self)
    }

    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
        ReflectMut::Value(self)
    }

    fn clone_value(&self) -> Box<dyn Reflect> {
        Box::new(self.clone())
    }

    fn reflect_hash(&self) -> Option<u64> {
        None
    }

    fn reflect_partial_eq(&self, _value: &dyn Reflect) -> Option<bool> {
        None
    }

    fn serializable(&self) -> Option<bevy::reflect::serde::Serializable> {
        Some(Serializable::Borrowed(self))
    }
}

impl GetTypeRegistration for PhysicsState {
    fn get_type_registration() -> bevy::reflect::TypeRegistration {
        let mut registration = TypeRegistration::of::<PhysicsState>();
        registration.insert::<ReflectDeserialize>(FromType::<PhysicsState>::from_type());
        registration
    }
}
