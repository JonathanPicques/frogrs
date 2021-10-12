use bevy::prelude::*;

use crate::game::core::frame::structs::FrameCount;

pub fn frame_system(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}
