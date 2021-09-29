use bevy::prelude::*;
use bevy_ggrs::{GGRSApp, GGRSPlugin};
use ggrs::{P2PSession, PlayerType};
use std::convert::TryInto;
use std::net::SocketAddr;
use structopt::StructOpt;

mod game;

#[derive(StructOpt)]
struct CommandLineArgs {
    #[structopt(long)]
    port: u16,
    #[structopt(long)]
    players: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = CommandLineArgs::from_args();

    let mut session = P2PSession::new(
        cmd.players.len().try_into()?,
        std::mem::size_of::<u8>(),
        cmd.port,
    )?;
    session.set_fps(60)?;
    session.set_sparse_saving(true)?;

    App::new()
        .insert_resource(cmd)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "frogrs".to_owned(),
            vsync: false,
            ..Default::default()
        })
        .insert_resource(game::FrameCounter { frame: 0 })
        //
        .add_plugin(GGRSPlugin)
        .add_plugins(DefaultPlugins)
        //
        .with_fps(60)
        .with_p2p_session(session)
        .with_input_system(game::input)
        .add_startup_system(start_p2p_session)
        .add_startup_system(game::setup_game_system)
        .add_rollback_system(game::update_game_system)
        .add_rollback_system(game::update_frame_system)
        .register_rollback_type::<Transform>()
        //
        .run();

    Ok(())
}

fn start_p2p_session(mut session: ResMut<P2PSession>, cmd: Res<CommandLineArgs>) {
    for (player_handle, player_address) in cmd.players.iter().enumerate() {
        if player_address == "local" {
            session
                .add_player(PlayerType::Local, player_handle)
                .unwrap();
            session.set_frame_delay(2, player_handle).unwrap();
        } else {
            let remote_player_address: SocketAddr = player_address
                .parse()
                .expect("Invalid remote player address");
            session
                .add_player(PlayerType::Remote(remote_player_address), player_handle)
                .unwrap();
        }
    }

    session.start_session().unwrap();
}
