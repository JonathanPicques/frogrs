use bevy::prelude::*;

#[derive(Hash, Default, Reflect)]
#[reflect(Hash)]
pub struct SpriteSheetAnimation {
    pub dt: u8,
    pub speed: u8,
}
