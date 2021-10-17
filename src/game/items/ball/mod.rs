use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{
    DrawMode, FillMode, FillOptions, GeometryBuilder, StrokeMode, StrokeOptions,
};
use bevy_prototype_lyon::shapes;
use rapier2d::prelude::*;

use crate::game::core::frame::structs::FrameCount;
use crate::game::core::maths::structs::Transform2D;
use crate::game::core::physics::structs::{ColliderSetRes, RigidBodyHandle2D, RigidBodySetRes};
use crate::game::core::physics::systems::PhysicsGroups;

#[derive(Default, Reflect, Component)]
pub struct Ball2D {}

pub fn ball_system(
    frame_count: Res<FrameCount>,
    mut commands: Commands,
    //
    mut query: Query<(Entity, &Ball2D)>,
) {
    if frame_count.frame > 144 {
        for (entity, _) in query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn startup_ball_system(
    mut commands: Commands,
    mut collider_set: ResMut<ColliderSetRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
) {
    let shape = shapes::Circle {
        radius: 30.0,
        ..shapes::Circle::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode {
                    color: Color::YELLOW,
                    options: FillOptions::default(),
                },
                outline_mode: StrokeMode {
                    color: Color::BLACK,
                    options: StrokeOptions::default(),
                },
            },
            Transform::default(),
        ))
        .insert(Ball2D::default())
        .insert(Transform2D::default())
        .insert(create_ball_rigid_body(
            &mut collider_set,
            &mut rigid_body_set,
        ));
}

pub fn create_ball_rigid_body(
    collider_set: &mut ColliderSetRes,
    rigid_body_set: &mut RigidBodySetRes,
) -> RigidBodyHandle2D {
    let rigid_body = RigidBodyBuilder::new_dynamic().build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);
    let rigid_body_collider = ColliderBuilder::ball(1.0)
        .restitution(0.7)
        .collision_groups(InteractionGroups::new(
            PhysicsGroups::Player as u32,
            PhysicsGroups::Solid as u32 | PhysicsGroups::Player as u32,
        ))
        .build();

    collider_set.insert_with_parent(rigid_body_collider, rigid_body_handle, rigid_body_set);

    info!("create_ball_rigid_body: handle: {:?}", rigid_body_handle);

    RigidBodyHandle2D(rigid_body_handle)
}
