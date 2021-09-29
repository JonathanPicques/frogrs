use bevy::prelude::*;

#[derive(Hash, Default, Reflect)]
#[reflect(Hash)]
pub struct FrameCount {
    frame: u32,
}

pub fn frame_system(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}
