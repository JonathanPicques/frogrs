use bevy::prelude::*;

#[derive(Hash, Default, Reflect, Component)]
#[reflect(Hash)]
pub struct SpriteSheetAnimation {
    pub dt: u8,
    pub speed: u8,
}
