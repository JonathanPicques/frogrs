use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::{GameInput, P2PSession, P2PSpectatorSession, PlayerHandle, SyncTestSession};

const INPUT_UP: u8 = 1 << 1;
const INPUT_DOWN: u8 = 1 << 2;
const INPUT_LEFT: u8 = 1 << 3;
const INPUT_RIGHT: u8 = 1 << 4;
const INPUT_JUMP: u8 = 1 << 5;

pub struct Player {
    pub handle: u32,
}

pub struct FrameCounter {
    pub frame: u32,
}

#[allow(dead_code)]
pub fn input(_handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> Vec<u8> {
    let mut input: u8 = 0;

    if keyboard_input.pressed(KeyCode::Up) {
        input |= INPUT_UP;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        input |= INPUT_DOWN;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        input |= INPUT_LEFT;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        input |= INPUT_RIGHT;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        input |= INPUT_JUMP;
    }

    vec![input]
}

#[allow(dead_code)]
pub fn input_random(_handle: In<PlayerHandle>, _keyboard_input: Res<Input<KeyCode>>) -> Vec<u8> {
    vec![rand::random()]
}

pub fn setup_game_system(
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
            .insert(Player { handle })
            .insert(Rollback::new(rollback_id_provider.next_id()));
    }
}

pub fn update_game_system(
    mut query: Query<(&Player, &mut Transform), With<Rollback>>,
    inputs: Res<Vec<GameInput>>,
) {
    for (player, mut _transform) in query.iter_mut() {
        let _input = inputs[player.handle as usize].buffer[0];
    }
}

pub fn update_frame_system(mut frame_counter: ResMut<FrameCounter>) {
    frame_counter.frame += 1;
}
