pub mod core;
pub mod player;

use bevy::prelude::*;
use bevy_ggrs::{GGRSApp, GGRSPlugin};

use self::core::anim::systems::animate_sprite_system;
use self::core::frame::structs::FrameCount;
use self::core::frame::systems::frame_system;
use self::core::input::systems::input_system;
use self::player::player_system;
use self::player::setup_player_system;

pub const GAME_FPS: u32 = 60;

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
            .with_fps(GAME_FPS)
            .with_input_system(input_system)
            .add_startup_system(setup_player_system)
            .add_rollback_system(frame_system)
            .add_rollback_system(player_system)
            .add_rollback_system(animate_sprite_system);

        self
    }
}
