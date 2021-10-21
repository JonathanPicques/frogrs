pub mod core;
pub mod items;
pub mod player;

use bevy::prelude::*;
use bevy_ggrs::{GGRSApp, GGRSPlugin};
use bevy_prototype_lyon::plugin::ShapePlugin;

use crate::game::core::anim::systems::animate_sprite_system;
use crate::game::core::debug::debug_system;
use crate::game::core::frame::structs::FrameCount;
use crate::game::core::frame::systems::frame_system;
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

pub const GAME_FPS: u32 = 60;

#[derive(Eq, Hash, Clone, Debug, PartialEq, StageLabel)]
enum RollbackStages {
    Game,
    Physics,
    TransformSynchronization,
}

pub trait GameApp {
    fn insert_game(&mut self, window_title: &str) -> &mut Self;
}

impl GameApp for App {
    fn insert_game(&mut self, window_title: &str) -> &mut Self {
        self
            //
            .insert_resource(Msaa { samples: 4 })
            .insert_resource(WindowDescriptor {
                title: window_title.to_owned(),
                vsync: false,
                width: 1280.0,
                height: 720.0,
                ..Default::default()
            })
            //
            .add_plugins(DefaultPlugins)
            .add_plugin(ShapePlugin)
            .add_plugin(GGRSPlugin)
            //
            .insert_rollback_resource(FrameCount::default())
            //
            .insert_rollback_resource(GravityRes::default())
            .insert_rollback_resource(JointSetRes::default())
            .insert_rollback_resource(CCDSolverRes::default())
            .insert_rollback_resource(BroadPhaseRes::default())
            .insert_rollback_resource(ColliderSetRes::default())
            .insert_rollback_resource(NarrowPhaseRes::default())
            .insert_rollback_resource(RigidBodySetRes::default())
            .insert_rollback_resource(IslandManagerRes::default())
            .insert_rollback_resource(QueryPipelineRes::default())
            .insert_rollback_resource(IntegrationParametersRes::default())
            .insert_rollback_resource(RigidBodyRemovedEntitiesRes::default())
            //
            .register_rollback_type::<Ball2D>()
            .register_rollback_type::<Player2D>()
            .register_rollback_type::<Transform2D>()
            .register_rollback_type::<RigidBodyHandle2D>()
            //
            .with_input_system(input_system)
            .with_update_frequency(GAME_FPS)
            //
            .add_system(debug_system.exclusive_system())
            .add_startup_system(startup_ball_system)
            .add_startup_system(startup_player_system)
            //
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
                        RollbackStages::TransformSynchronization,
                        SystemStage::parallel().with_system(sync_transform_system),
                    ),
            );

        self
    }
}
