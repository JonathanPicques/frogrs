pub mod core;
pub mod player;

use bevy::prelude::*;
use bevy_ggrs::{GGRSApp, GGRSPlugin};

use crate::game::core::anim::systems::animate_sprite_system;
use crate::game::core::frame::structs::FrameCount;
use crate::game::core::frame::systems::frame_system;
use crate::game::core::input::systems::input_system;
use crate::game::core::maths::structs::Transform2D;
use crate::game::core::maths::systems::sync_transform_system;
use crate::game::core::physics::structs::{PhysicsState, RigidBodyHandle2D};
use crate::game::core::physics::systems::physics_system;
use crate::game::player::{player_system, setup_player_system};

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
            .add_plugin(GGRSPlugin)
            .add_plugins(DefaultPlugins)
            //
            .insert_rollback_resource(FrameCount::default())
            .insert_rollback_resource(PhysicsState::default())
            //
            .register_rollback_type::<Transform2D>()
            .register_rollback_type::<RigidBodyHandle2D>()
            //
            .with_input_system(input_system)
            .with_update_frequency(GAME_FPS)
            //
            .add_startup_system(setup_player_system)
            //
            .with_rollback_schedule(
                Schedule::default()
                    .with_stage(
                        RollbackStages::Game,
                        SystemStage::single_threaded()
                            .with_system(frame_system)
                            .with_system(player_system)
                            .with_system(animate_sprite_system),
                    )
                    .with_stage_after(
                        RollbackStages::Game,
                        RollbackStages::Physics,
                        SystemStage::single_threaded().with_system(physics_system),
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
