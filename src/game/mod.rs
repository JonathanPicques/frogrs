pub mod core;
pub mod items;
pub mod player;

use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_ggrs::GGRSPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use ggrs::Config;

use crate::game::core::anim::systems::animate_sprite_system;
use crate::game::core::debug::debug_system;
use crate::game::core::frame::structs::FrameCount;
use crate::game::core::frame::systems::frame_system;
use crate::game::core::input::structs::BoxInput;
use crate::game::core::input::systems::input_system;
use crate::game::core::maths::structs::Transform2D;
use crate::game::core::maths::systems::sync_transform_system;
use crate::game::core::physics::structs::*;
use crate::game::core::physics::systems::{
    physics_system_add, physics_system_remove, physics_system_step,
};
use crate::game::items::ball::{ball_system, startup_ball_system, Ball2D};
use crate::game::player::structs::Player2D;
use crate::game::player::systems::{player_system, startup_player_system};

pub const GAME_FPS: usize = 60;

#[derive(Debug)]
pub struct GameConfig;
impl Config for GameConfig {
    type Input = BoxInput;
    type State = u8;
    type Address = SocketAddr;
}

#[derive(Eq, Hash, Clone, Debug, PartialEq, StageLabel)]
enum RollbackStages {
    Game,
    Physics,
    Synchronization,
}

pub trait GameApp {
    fn insert_game(&mut self, window_title: &str) -> &mut Self;
}

impl GameApp for App {
    fn insert_game(&mut self, window_title: &str) -> &mut Self {
        GGRSPlugin::<GameConfig>::new()
            // define frequency of rollback game logic update
            .with_update_frequency(GAME_FPS)
            // define system that returns inputs given a player handle, so GGRS can send the inputs around
            .with_input_system(input_system)
            // register types of resources you want to be rolled back
            .register_rollback_resource::<FrameCount>()
            .register_rollback_resource::<GravityRes>()
            .register_rollback_resource::<CCDSolverRes>()
            .register_rollback_resource::<BroadPhaseRes>()
            .register_rollback_resource::<ColliderSetRes>()
            .register_rollback_resource::<NarrowPhaseRes>()
            .register_rollback_resource::<RigidBodySetRes>()
            .register_rollback_resource::<IslandManagerRes>()
            .register_rollback_resource::<QueryPipelineRes>()
            .register_rollback_resource::<ImpulseJointSetRes>()
            .register_rollback_resource::<MultibodyJointSetRes>()
            .register_rollback_resource::<IntegrationParametersRes>()
            .register_rollback_resource::<RigidBodyRemovedEntitiesRes>()
            // register types of components you want to be rolled back
            .register_rollback_component::<Ball2D>()
            .register_rollback_component::<Player2D>()
            .register_rollback_component::<Transform2D>()
            .register_rollback_component::<RigidBodyHandle2D>()
            // these systems will be executed as part of the advance frame update
            .with_rollback_schedule(
                Schedule::default()
                    .with_stage(
                        RollbackStages::Game,
                        SystemStage::single_threaded()
                            .with_system(ball_system)
                            .with_system(frame_system)
                            .with_system(player_system)
                            .with_system(animate_sprite_system),
                    )
                    .with_stage_after(
                        RollbackStages::Game,
                        RollbackStages::Physics,
                        SystemStage::single_threaded()
                            .with_system(physics_system_add)
                            .with_system(physics_system_step)
                            .with_system(physics_system_remove),
                    )
                    .with_stage_after(
                        RollbackStages::Physics,
                        RollbackStages::Synchronization,
                        SystemStage::parallel().with_system(sync_transform_system),
                    ),
            )
            // make it happen in the bevy app
            .build(self);

        self
            //
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    width: 720.,
                    height: 720.,
                    title: window_title.to_owned(),
                    ..default()
                },
                ..default()
            }))
            .add_plugin(ShapePlugin)
            //
            .insert_resource(Msaa { samples: 4 })
            //
            .insert_resource(FrameCount::default())
            .insert_resource(GravityRes::default())
            .insert_resource(CCDSolverRes::default())
            .insert_resource(BroadPhaseRes::default())
            .insert_resource(ColliderSetRes::default())
            .insert_resource(NarrowPhaseRes::default())
            .insert_resource(RigidBodySetRes::default())
            .insert_resource(IslandManagerRes::default())
            .insert_resource(QueryPipelineRes::default())
            .insert_resource(ImpulseJointSetRes::default())
            .insert_resource(MultibodyJointSetRes::default())
            .insert_resource(IntegrationParametersRes::default())
            .insert_resource(RigidBodyRemovedEntitiesRes::default())
            //
            .add_system(debug_system.at_end())
            .add_startup_system(startup_ball_system)
            .add_startup_system(startup_player_system);

        self
    }
}
