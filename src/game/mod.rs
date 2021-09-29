use bevy::prelude::*;
use bevy_ggrs::{GGRSApp, GGRSPlugin};

pub mod frame;
pub mod input;
pub mod player;

use frame::frame_system;
use frame::FrameCount;
use input::input_system;
use player::player_system;
use player::setup_player_system;

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
            .with_fps(60)
            .with_input_system(input_system)
            .add_startup_system(setup_player_system)
            .add_rollback_system(frame_system)
            .add_rollback_system(player_system);

        self
    }
}
