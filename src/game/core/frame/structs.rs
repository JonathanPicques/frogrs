use bevy::prelude::*;

#[derive(Hash, Default, Reflect, Resource)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}
