mod game;

use bevy::app::App;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy_ggrs::GGRSApp;
use ggrs::P2PSession;
use ggrs::PlayerType;
use log::LevelFilter;
use simplelog::*;
use std::error::Error;
use std::net::SocketAddr;
use structopt::StructOpt;

use crate::game::core::input::structs::INPUT_SIZE;
use crate::game::GameApp;
use crate::game::GAME_FPS;

#[derive(StructOpt)]
struct CommandLineArgs {
    #[structopt(long)]
    port: u16,
    #[structopt(long)]
    players: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = CommandLineArgs::from_args();
    let mut p2p_session = P2PSession::new(cmd.players.len() as u32, INPUT_SIZE, cmd.port)?;
    p2p_session.set_fps(GAME_FPS)?;
    p2p_session.set_sparse_saving(true)?;

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    App::new()
        .insert_game("frogrs")
        .insert_resource(cmd)
        .with_p2p_session(p2p_session)
        .add_startup_system(start_p2p_session)
        .run();

    Ok(())
}

fn start_p2p_session(mut p2p_session: ResMut<P2PSession>, cmd: Res<CommandLineArgs>) {
    for (player_handle, player_address) in cmd.players.iter().enumerate() {
        if player_address == "local" {
            p2p_session
                .add_player(PlayerType::Local, player_handle)
                .unwrap();
            p2p_session.set_frame_delay(2, player_handle).unwrap();
        } else {
            let remote_player_address: SocketAddr = player_address
                .parse()
                .expect("Invalid remote player address");
            p2p_session
                .add_player(PlayerType::Remote(remote_player_address), player_handle)
                .unwrap();
        }
    }

    p2p_session.start_session().unwrap();
}
