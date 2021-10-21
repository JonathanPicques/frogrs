use bevy::prelude::*;
use ggrs::PlayerHandle;

use crate::game::core::{
    anim::structs::SpriteSheetAnimation, maths::structs::Transform2D, physics::structs::*,
};

#[derive(Default, Reflect, Component)]
pub struct Player2D {
    pub handle: PlayerHandle,
}

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    pub player: Player2D,
    pub transform: Transform2D,
    pub rigid_body_handle: RigidBodyHandle2D,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
    pub sprite_sheet_animation: SpriteSheetAnimation,
}
