use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::{GameInput, P2PSession, P2PSpectatorSession, PlayerHandle, SyncTestSession};
use log::info;
use rapier2d::prelude::*;

use crate::game::{
    core::{
        anim::{structs::SpriteSheetAnimation, utilities::speed_as_secs},
        input::structs::{INPUT_JUMP, INPUT_LEFT, INPUT_RIGHT},
        maths::structs::{Transform2D, Vector2D},
        physics::{structs::*, systems::PhysicsGroups},
    },
    player::structs::{Player2D, PlayerBundle},
    GAME_FPS,
};

pub fn player_system(
    inputs: Res<Vec<GameInput>>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    //
    mut query: Query<(&Player2D, &RigidBodyHandle2D), With<Rollback>>,
) {
    for (player, rigid_body_handle) in query.iter_mut() {
        let input = inputs[player.handle].buffer[0];
        let rigid_body = &mut rigid_body_set[rigid_body_handle.0];

        if input & INPUT_LEFT != 0 {
            rigid_body.apply_force(vector!(-30.0, 0.0), true);
        } else if input & INPUT_RIGHT != 0 {
            rigid_body.apply_force(vector!(30.0, 0.0), true);
        } else if input & INPUT_JUMP != 0 {
            rigid_body.apply_force(vector!(0.0, 100.0), true);
        }
    }
}

pub fn startup_player_system(
    asset_server: Res<AssetServer>,
    //
    p2p_session: Option<Res<P2PSession>>,
    synctest_session: Option<Res<SyncTestSession>>,
    spectator_session: Option<Res<P2PSpectatorSession>>,
    //
    mut commands: Commands,
    mut textures: ResMut<Assets<TextureAtlas>>,
    mut collider_set: ResMut<ColliderSetRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    mut rollback_id_provider: ResMut<RollbackIdProvider>,
) {
    let num_players = p2p_session
        .map(|s| s.num_players())
        .or_else(|| synctest_session.map(|s| s.num_players()))
        .or_else(|| spectator_session.map(|s| s.num_players()))
        .expect("No ggrs session found");
    let font_handle = asset_server.load("fonts/Pixellari.ttf");
    let texture_handle = asset_server.load("textures/frog/Stand.png");

    let transform = Transform2D::from_position(Vector2D::new(0.0, -10.0));
    commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        //
        .insert(create_world_rigid_body(
            &transform,
            &mut collider_set,
            &mut rigid_body_set,
        ))
        .insert(transform);

    for handle in 0..num_players {
        let transform = Transform2D::from_position(Vector2D::new(handle as f32 * 10.0, 10.0));
        let rigid_body_handle = create_player_rigid_body(
            handle as usize,
            &transform,
            &mut collider_set,
            &mut rigid_body_set,
        );

        commands.spawn_bundle(OrthographicCameraBundle::new_2d());

        commands
            .spawn_bundle(PlayerBundle {
                player: Player2D {
                    handle: handle as usize,
                },
                transform,
                rigid_body_handle,
                //
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
            .insert(Rollback::new(rollback_id_provider.next_id()))
            //
            .with_children(|parent| {
                parent.spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "RootKernel".to_string(),
                        TextStyle {
                            font: font_handle.clone(),
                            color: Color::WHITE,
                            font_size: 11.0,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    transform: Transform {
                        translation: Vec3::new(0.0, 30.0, 0.0),
                        ..Default::default()
                    },
                    //
                    ..Default::default()
                });
            });
    }
}

fn create_world_rigid_body(
    transform: &Transform2D,
    collider_set: &mut ColliderSetRes,
    rigid_body_set: &mut RigidBodySetRes,
) -> RigidBodyHandle2D {
    let rigid_body = RigidBodyBuilder::new_static()
        .rotation(transform.rotation)
        .translation(vector![transform.position.x, transform.position.y])
        .build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);
    let rigid_body_collider = ColliderBuilder::cuboid(20.0, 1.0)
        .restitution(0.0)
        .collision_groups(InteractionGroups::new(
            PhysicsGroups::Solid as u32,
            PhysicsGroups::Solid as u32 | PhysicsGroups::Player as u32,
        ))
        .build();

    collider_set.insert_with_parent(rigid_body_collider, rigid_body_handle, rigid_body_set);

    info!(
        "create_world_rigid_body: rigid_body_handle: {:?}",
        rigid_body_handle
    );

    RigidBodyHandle2D(rigid_body_handle)
}

fn create_player_rigid_body(
    handle: PlayerHandle,
    transform: &Transform2D,
    collider_set: &mut ColliderSetRes,
    rigid_body_set: &mut RigidBodySetRes,
) -> RigidBodyHandle2D {
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .rotation(transform.rotation)
        .translation(vector![transform.position.x, transform.position.y])
        .lock_rotations()
        .build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);
    let rigid_body_collider = ColliderBuilder::cuboid(0.5, 1.4)
        .restitution(0.0)
        .collision_groups(InteractionGroups::new(
            PhysicsGroups::Player as u32,
            PhysicsGroups::Solid as u32 | PhysicsGroups::Player as u32,
        ))
        .build();

    collider_set.insert_with_parent(rigid_body_collider, rigid_body_handle, rigid_body_set);

    info!(
        "create_player_rigid_body: player_handle: {}, rigid_body_handle: {:?}",
        handle, rigid_body_handle
    );

    RigidBodyHandle2D(rigid_body_handle)
}
