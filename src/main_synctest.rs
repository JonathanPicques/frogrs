mod game;

use bevy::app::App;
use bevy_ggrs::Session;
use ggrs::{PlayerType, SessionBuilder};
use std::error::Error;
use structopt::StructOpt;

use crate::game::{GameApp, GameConfig};

#[derive(StructOpt)]
struct CommandLineArgs {
    #[structopt(long)]
    players: usize,
    #[structopt(long)]
    check_distance: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = CommandLineArgs::from_args();
    let num_players = cmd.players;
    assert!(num_players > 0);

    // create a GGRS session
    let mut session_builder = SessionBuilder::<GameConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(2)
        .with_max_prediction_window(cmd.check_distance);

    // add players
    for i in 0..cmd.players {
        session_builder = session_builder.add_player(PlayerType::Local, i)?;
    }

    // start the GGRS session
    let session = session_builder.start_synctest_session()?;

    App::new()
        .insert_game("frogrs_synctest")
        .insert_resource(Session::SyncTestSession(session))
        .run();

    Ok(())
}
