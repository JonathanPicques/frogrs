use bevy::prelude::*;
use bevy_ggrs::{GGRSApp, GGRSPlugin};
use ggrs::SyncTestSession;
use structopt::StructOpt;

mod game;

#[derive(StructOpt)]
struct CommandLineArgs {
    #[structopt(long)]
    players: u32,
    #[structopt(long)]
    check_distance: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = CommandLineArgs::from_args();
    let session = SyncTestSession::new(cmd.players, std::mem::size_of::<u8>(), cmd.check_distance)?;

    App::new()
        .insert_resource(cmd)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "frogrs_synctest".to_owned(),
            vsync: false,
            ..Default::default()
        })
        .insert_resource(game::FrameCounter { frame: 0 })
        //
        .add_plugin(GGRSPlugin)
        .add_plugins(DefaultPlugins)
        //
        .with_fps(60)
        .with_input_system(game::input_random)
        .with_synctest_session(session)
        .add_startup_system(start_synctest_session)
        .add_startup_system(game::setup_game_system)
        .add_rollback_system(game::update_game_system)
        .add_rollback_system(game::update_frame_system)
        .register_rollback_type::<Transform>()
        //
        .run();

    Ok(())
}

fn start_synctest_session(mut session: ResMut<SyncTestSession>, cmd: Res<CommandLineArgs>) {
    for player_handle in 0..cmd.players {
        session.set_frame_delay(2, player_handle as usize).unwrap();
    }
}
