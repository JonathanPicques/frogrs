use bevy::prelude::*;

#[derive(Hash, Default, Reflect, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}
