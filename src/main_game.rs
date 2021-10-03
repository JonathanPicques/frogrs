use bevy::app::App;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy_ggrs::GGRSApp;
use ggrs::P2PSession;
use ggrs::PlayerType;
use std::error::Error;
use std::net::SocketAddr;
use structopt::StructOpt;

mod game;
use game::input::INPUT_SIZE;
use game::GameApp;

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
    p2p_session.set_fps(60)?;
    p2p_session.set_sparse_saving(true)?;

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
