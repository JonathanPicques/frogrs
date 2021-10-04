use bevy::prelude::*;

#[derive(Hash, Default, Reflect)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}
