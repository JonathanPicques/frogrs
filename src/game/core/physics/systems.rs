use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rapier2d::prelude::*;

use crate::game::core::maths::structs::Transform2D;
use crate::game::core::physics::range::scale_physics;
use crate::game::core::physics::structs::*;

pub const SOLID_PHYSICS_GROUP: rapier2d::geometry::Group = Group::GROUP_1;
pub const PLAYER_PHYSICS_GROUP: rapier2d::geometry::Group = Group::GROUP_2;

pub fn physics_system_add(
    collider_set: Res<ColliderSetRes>,
    rigid_body_set: Res<RigidBodySetRes>,
    mut commands: Commands,
    mut rigid_body_entities: ResMut<RigidBodyRemovedEntitiesRes>,
    //
    mut query: Query<(Entity, &RigidBodyHandle2D), Added<RigidBodyHandle2D>>,
) {
    for (entity, rigid_body_handle) in query.iter_mut() {
        if rigid_body_set.contains(rigid_body_handle.0) {
            let rigid_body = &rigid_body_set[rigid_body_handle.0];

            for collider_handle in rigid_body.colliders() {
                let rigid_body_collider = &collider_set[*collider_handle];
                let rigid_body_collider_shape = rigid_body_collider.shape();

                match rigid_body_collider_shape.shape_type() {
                    ShapeType::Ball => {
                        let ball = rigid_body_collider_shape.as_ball().unwrap();
                        let circle_shape = shapes::Circle {
                            radius: scale_physics(ball.radius),
                            ..default()
                        };

                        commands
                            .entity(entity)
                            .insert(SpatialBundle::default())
                            .with_children(|child_builder| {
                                child_builder.spawn(GeometryBuilder::build_as(
                                    &circle_shape,
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
                                    Transform::default(), // TODO: collider transform
                                ));
                            });
                    }
                    ShapeType::Cuboid => {
                        let cuboid = rigid_body_collider_shape.as_cuboid().unwrap();
                        let rectangle_shape = shapes::Rectangle {
                            extents: Vec2::new(
                                scale_physics(cuboid.half_extents[0] * 2.0),
                                scale_physics(cuboid.half_extents[1] * 2.0),
                            ),
                            ..default()
                        };

                        commands
                            .entity(entity)
                            .insert(SpatialBundle::default())
                            .with_children(|child_builder| {
                                child_builder.spawn(GeometryBuilder::build_as(
                                    &rectangle_shape,
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
                                    Transform::default(), // TODO: collider transform
                                ));
                            });
                    }
                    _ => (),
                };
            }

            rigid_body_entities.insert(entity, rigid_body_handle.0);
        }
    }
}

pub fn physics_system_step(
    gravity: Res<GravityRes>,
    integration_parameters: Res<IntegrationParametersRes>,
    //
    mut ccd_solver: ResMut<CCDSolverRes>,
    mut broad_phase: ResMut<BroadPhaseRes>,
    mut collider_set: ResMut<ColliderSetRes>,
    mut narrow_phase: ResMut<NarrowPhaseRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    mut island_manager: ResMut<IslandManagerRes>,
    mut query_pipeline: ResMut<QueryPipelineRes>,
    mut impulse_joint_set: ResMut<ImpulseJointSetRes>,
    mut multibody_joint_set: ResMut<MultibodyJointSetRes>,
    //
    mut query: Query<(&mut Transform2D, &RigidBodyHandle2D)>,
) {
    let hooks = ();
    let events = ();
    let mut physics_pipeline = PhysicsPipeline::new();

    physics_pipeline.step(
        &gravity,
        &integration_parameters,
        &mut island_manager,
        &mut broad_phase,
        &mut narrow_phase,
        &mut rigid_body_set,
        &mut collider_set,
        &mut impulse_joint_set,
        &mut multibody_joint_set,
        &mut ccd_solver,
        Some(&mut query_pipeline),
        &hooks,
        &events,
    );

    query_pipeline.update(&rigid_body_set, &collider_set);

    for (mut transform, rigid_body_handle) in query.iter_mut() {
        let rigid_body = &rigid_body_set[rigid_body_handle.0];
        let rigid_body_rotation = rigid_body.rotation();
        let rigid_body_translation = rigid_body.translation();

        transform.rotation = rigid_body_rotation.angle();
        transform.position.set(
            scale_physics(rigid_body_translation.x),
            scale_physics(rigid_body_translation.y),
        );
    }
}

pub fn physics_system_remove(
    removed_entities: RemovedComponents<RigidBodyHandle2D>,
    //
    mut collider_set: ResMut<ColliderSetRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    mut island_manager: ResMut<IslandManagerRes>,
    mut impulse_joint_set: ResMut<ImpulseJointSetRes>,
    mut multibody_joint_set: ResMut<MultibodyJointSetRes>,
    mut rigid_body_entities: ResMut<RigidBodyRemovedEntitiesRes>,
) {
    for removed_entity in removed_entities.iter() {
        if let Some(&rigid_body_handle) = rigid_body_entities.get(&removed_entity) {
            rigid_body_set.remove(
                rigid_body_handle,
                &mut island_manager,
                &mut collider_set,
                &mut impulse_joint_set,
                &mut multibody_joint_set,
                true,
            );

            rigid_body_entities.remove(&removed_entity);
        }
    }
}
