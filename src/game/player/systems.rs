use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;
use bevy_ggrs::{PlayerInputs, Rollback, RollbackIdProvider, Session};
use log::info;
use rapier2d::prelude::*;

use crate::game::{
    core::physics::systems::{PLAYER_PHYSICS_GROUP, SOLID_PHYSICS_GROUP},
    GameConfig,
};
use crate::game::{
    core::{
        anim::{structs::SpriteSheetAnimation, utilities::speed_as_secs},
        input::structs::{INPUT_JUMP, INPUT_LEFT, INPUT_RIGHT},
        maths::structs::{Transform2D, Vector2D},
        physics::structs::*,
    },
    player::structs::{Player2D, PlayerBundle},
    GAME_FPS,
};

pub fn player_system(
    inputs: Res<PlayerInputs<GameConfig>>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    //
    mut query: Query<(&Player2D, &RigidBodyHandle2D, &Children), With<Rollback>>,
    mut query_children_text: Query<&mut Text>,
) {
    for (player, rigid_body_handle, children) in query.iter_mut() {
        let (input, _) = inputs[player.handle];
        let rigid_body = &mut rigid_body_set[rigid_body_handle.0];

        for &child in children.iter() {
            if let Ok(mut text) = query_children_text.get_mut(child) {
                text.sections[0].value = format!("{:?} {:?}", player.handle, input.inp).to_string();
            }
        }

        if input.inp & INPUT_LEFT != 0 {
            rigid_body.apply_impulse(vector!(-1.0, 0.0), true);
        } else if input.inp & INPUT_RIGHT != 0 {
            rigid_body.apply_impulse(vector!(1.0, 0.0), true);
        } else if input.inp & INPUT_JUMP != 0 {
            rigid_body.apply_impulse(vector!(0.0, 1.0), true);
        }
    }
}

pub fn startup_player_system(
    session: Res<Session<GameConfig>>,
    asset_server: Res<AssetServer>,
    //
    mut commands: Commands,
    mut textures: ResMut<Assets<TextureAtlas>>,
    mut collider_set: ResMut<ColliderSetRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    mut rollback_id_provider: ResMut<RollbackIdProvider>,
) {
    let num_players = match &*session {
        Session::P2PSession(s) => s.num_players(),
        Session::SyncTestSession(s) => s.num_players(),
        Session::SpectatorSession(s) => s.num_players(),
    };
    let font_handle: Handle<Font> = asset_server.load("fonts/Pixellari.ttf");
    let texture_handle: Handle<Image> = asset_server.load("textures/frog/Stand.png");

    let transform = Transform2D::from_position(Vector2D::new(0.0, -10.0));
    commands.spawn((
        Transform::default(),
        GlobalTransform::default(),
        create_world_rigid_body(&transform, &mut collider_set, &mut rigid_body_set),
        transform,
    ));

    for player_handle in 0..num_players {
        let transform = Transform2D::from_position(Vector2D::new(player_handle as f32 * 5.0, 10.0));
        let rigid_body_handle = create_player_rigid_body(
            player_handle,
            &transform,
            &mut collider_set,
            &mut rigid_body_set,
        );

        let is_local_player = match &*session {
            Session::P2PSession(s) => s.local_player_handles().contains(&player_handle),
            Session::SyncTestSession(_) => false,
            Session::SpectatorSession(_) => false,
        };

        if is_local_player {
            commands.spawn(Camera2dBundle::default());
        }

        commands
            .spawn((
                PlayerBundle {
                    player: Player2D {
                        handle: player_handle,
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
                            None,
                            None,
                        )),
                        //
                        ..default()
                    },
                    sprite_sheet_animation: SpriteSheetAnimation {
                        speed: speed_as_secs(GAME_FPS as u32, 0.06),
                        //
                        ..default()
                    },
                    //
                    ..default()
                },
                rollback_id_provider.next(),
            ))
            //
            .with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        "".to_string(),
                        TextStyle {
                            font: font_handle.clone(),
                            color: Color::WHITE,
                            font_size: 11.0,
                        },
                    ),
                    transform: Transform {
                        translation: Vec3::new(0.0, 30.0, 0.0),
                        ..default()
                    },
                    //
                    ..default()
                });
            });
    }
}

fn create_world_rigid_body(
    transform: &Transform2D,
    collider_set: &mut ColliderSetRes,
    rigid_body_set: &mut RigidBodySetRes,
) -> RigidBodyHandle2D {
    let rigid_body = RigidBodyBuilder::fixed()
        .rotation(transform.rotation)
        .translation(vector![transform.position.x, transform.position.y])
        .build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);
    let rigid_body_collider = ColliderBuilder::cuboid(20.0, 1.0)
        .restitution(0.0)
        .collision_groups(InteractionGroups::new(
            SOLID_PHYSICS_GROUP,
            SOLID_PHYSICS_GROUP | PLAYER_PHYSICS_GROUP,
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
    let rigid_body = RigidBodyBuilder::dynamic()
        .rotation(transform.rotation)
        .translation(vector![transform.position.x, transform.position.y])
        .lock_rotations()
        .build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);
    let rigid_body_collider = ColliderBuilder::cuboid(0.5, 1.4)
        .restitution(0.0)
        .collision_groups(InteractionGroups::new(
            PLAYER_PHYSICS_GROUP,
            SOLID_PHYSICS_GROUP | PLAYER_PHYSICS_GROUP,
        ))
        .build();

    collider_set.insert_with_parent(rigid_body_collider, rigid_body_handle, rigid_body_set);

    info!(
        "create_player_rigid_body: player_handle: {}, rigid_body_handle: {:?}",
        handle, rigid_body_handle
    );

    RigidBodyHandle2D(rigid_body_handle)
}
