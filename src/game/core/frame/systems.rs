use super::structs::FrameCount;
use bevy::prelude::ResMut;

pub fn frame_system(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}
