mod game;

use bevy::app::App;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy_ggrs::GGRSApp;
use ggrs::SyncTestSession;
use log::LevelFilter;
use simplelog::*;
use std::error::Error;
use structopt::StructOpt;

use crate::game::core::input::structs::INPUT_SIZE;
use crate::game::GameApp;

#[derive(StructOpt)]
struct CommandLineArgs {
    #[structopt(long)]
    players: u32,
    #[structopt(long)]
    check_distance: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = CommandLineArgs::from_args();
    let synctest_session = SyncTestSession::new(cmd.players, INPUT_SIZE, cmd.check_distance)?;

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    App::new()
        .insert_game("frogrs_synctest")
        .insert_resource(cmd)
        .with_synctest_session(synctest_session)
        .add_startup_system(start_synctest_session)
        .run();

    Ok(())
}

fn start_synctest_session(
    mut synctest_session: ResMut<SyncTestSession>,
    cmd: Res<CommandLineArgs>,
) {
    for player_handle in 0..cmd.players {
        synctest_session
            .set_frame_delay(2, player_handle as usize)
            .unwrap();
    }
}
