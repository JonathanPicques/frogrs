use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::{GameInput, P2PSession, P2PSpectatorSession, PlayerHandle, SyncTestSession};
use std::convert::TryInto;

use super::input::*;

pub struct Player {
    handle: PlayerHandle,
}

pub fn player_system(
    mut query: Query<(&Player, &mut Transform), With<Rollback>>,
    inputs: Res<Vec<GameInput>>,
) {
    for (player, mut transform) in query.iter_mut() {
        let input = inputs[player.handle as usize].buffer[0];

        if input & INPUT_LEFT != 0 {
            transform.translation.x -= 10.0;
        }
        if input & INPUT_RIGHT != 0 {
            transform.translation.x += 10.0;
        }
    }
}

pub fn setup_player_system(
    asset_server: Res<AssetServer>,
    //
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rollback_id_provider: ResMut<RollbackIdProvider>,
    //
    p2p_session: Option<Res<P2PSession>>,
    synctest_session: Option<Res<SyncTestSession>>,
    spectator_session: Option<Res<P2PSpectatorSession>>,
) {
    let num_players = p2p_session
        .map(|s| s.num_players())
        .or_else(|| synctest_session.map(|s| s.num_players()))
        .or_else(|| spectator_session.map(|s| s.num_players()))
        .expect("No GGRS session found");
    let texture_handle = asset_server.load("C:/Projects/frogrs/assets/frog.png");

    for handle in 0..num_players {
        let mut transform = Transform::default();
        transform.translation.x = handle as f32 * 100.0;

        commands.spawn_bundle(OrthographicCameraBundle::new_2d());

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(texture_handle.clone().into()),
                transform,
                ..Default::default()
            })
            .insert(Player {
                handle: handle.try_into().unwrap(),
            })
            .insert(Rollback::new(rollback_id_provider.next_id()));
    }
}
