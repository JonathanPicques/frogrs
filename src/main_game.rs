mod game;

use bevy::app::App;
use bevy_ggrs::Session;
use ggrs::{PlayerType, SessionBuilder, UdpNonBlockingSocket};
use std::error::Error;
use std::net::SocketAddr;
use structopt::StructOpt;

use crate::game::{GameApp, GameConfig};

#[derive(StructOpt)]
struct CommandLineArgs {
    #[structopt(long)]
    port: u16,
    #[structopt(long)]
    players: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = CommandLineArgs::from_args();
    let num_players = cmd.players.len();
    assert!(num_players > 0);

    // create a GGRS session
    let mut session_builder = SessionBuilder::<GameConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(2)
        .with_max_prediction_window(12);

    // add players
    for (i, player_addr) in cmd.players.iter().enumerate() {
        if player_addr == "local" {
            // local player
            session_builder = session_builder.add_player(PlayerType::Local, i)?;
        } else {
            // remote player
            let remote_addr: SocketAddr = player_addr.parse()?;
            session_builder = session_builder.add_player(PlayerType::Remote(remote_addr), i)?;
        }
    }

    // start the GGRS session
    let socket = UdpNonBlockingSocket::bind_to_port(cmd.port)?;
    let session = session_builder.start_p2p_session(socket)?;

    App::new()
        .insert_game("frogrs")
        .insert_resource(Session::P2PSession(session))
        .run();

    Ok(())
}
