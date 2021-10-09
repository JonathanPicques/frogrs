use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::{GameInput, P2PSession, P2PSpectatorSession, PlayerHandle, SyncTestSession};

use super::{
    core::{
        anim::{structs::SpriteSheetAnimation, utilities::speed_as_secs},
        input::structs::{INPUT_LEFT, INPUT_RIGHT},
        maths::structs::Transform2D,
        physics::structs::PhysicsBody2D,
    },
    GAME_FPS,
};

#[derive(Default, Component)]
pub struct Player {
    handle: PlayerHandle,
}

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    player: Player,
    transform: Transform2D,
    physics_body: PhysicsBody2D,
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    sprite_sheet_animation: SpriteSheetAnimation,
}

pub fn player_system(
    mut query: Query<(&Player, &mut PhysicsBody2D), With<Rollback>>,
    inputs: Res<Vec<GameInput>>,
) {
    for (player, mut physics_body) in query.iter_mut() {
        let input = inputs[player.handle].buffer[0];

        if input & INPUT_LEFT != 0 {
            physics_body.velocity.set_x(-10.0);
        } else if input & INPUT_RIGHT != 0 {
            physics_body.velocity.set_x(10.0);
        } else {
            physics_body.velocity.set_x(0.0);
        }
    }
}

pub fn setup_player_system(
    asset_server: Res<AssetServer>,
    //
    mut commands: Commands,
    mut textures: ResMut<Assets<TextureAtlas>>,
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
        .expect("No ggrs session found");
    let texture_handle = asset_server.load("frog/Stand.png");

    for handle in 0..num_players {
        let mut transform = Transform2D::default();
        transform.position.x = 0.0;
        transform.position.y = handle as f32 * 100.0;

        commands.spawn_bundle(OrthographicCameraBundle::new_2d());

        commands
            .spawn_bundle(PlayerBundle {
                player: Player {
                    handle: handle as usize,
                },
                transform,
                sprite_sheet: SpriteSheetBundle {
                    texture_atlas: textures.add(TextureAtlas::from_grid(
                        texture_handle.clone(),
                        Vec2::new(38.0, 32.0),
                        20,
                        1,
                    )),
                    //
                    ..Default::default()
                },
                sprite_sheet_animation: SpriteSheetAnimation {
                    speed: speed_as_secs(GAME_FPS, 0.06),
                    //
                    ..Default::default()
                },
                //
                ..Default::default()
            })
            .insert(Rollback::new(rollback_id_provider.next_id()));
    }
}
