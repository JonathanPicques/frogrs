pub mod core;
pub mod player;

use bevy::prelude::*;
use bevy_ggrs::{GGRSApp, GGRSPlugin};

use self::core::anim::systems::animate_sprite_system;
use self::core::frame::structs::FrameCount;
use self::core::frame::systems::frame_system;
use self::core::input::systems::input_system;
use self::core::maths::structs::Transform2D;
use self::core::maths::systems::sync_transform_system;
use self::core::physics::structs::PhysicsBody2D;
use self::core::physics::systems::physics_system;
use self::player::player_system;
use self::player::setup_player_system;

pub const GAME_FPS: u32 = 60;

const GAME_STAGE: &str = "game_stage";
const PHYSICS_STAGE: &str = "physics_stage";
const TRANSFORM_STAGE: &str = "transform_stage";

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
            .insert_resource(FrameCount::default())
            //
            .add_plugin(GGRSPlugin)
            .add_plugins(DefaultPlugins)
            //
            .with_input_system(input_system)
            .with_update_frequency(GAME_FPS)
            //
            .add_startup_system(setup_player_system)
            //
            .register_rollback_type::<Transform2D>()
            .register_rollback_type::<PhysicsBody2D>()
            //
            .with_rollback_schedule(
                Schedule::default()
                    .with_stage(
                        GAME_STAGE,
                        SystemStage::parallel()
                            .with_system(frame_system)
                            .with_system(player_system)
                            .with_system(animate_sprite_system),
                    )
                    .with_stage_after(
                        GAME_STAGE,
                        PHYSICS_STAGE,
                        SystemStage::parallel().with_system(physics_system),
                    )
                    .with_stage_after(
                        PHYSICS_STAGE,
                        TRANSFORM_STAGE,
                        SystemStage::parallel().with_system(sync_transform_system),
                    ),
            );

        self
    }
}
