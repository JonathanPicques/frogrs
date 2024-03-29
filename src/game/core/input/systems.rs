use bevy::prelude::*;
use ggrs::PlayerHandle;

use crate::game::core::input::structs::{
    BoxInput, INPUT_DOWN, INPUT_JUMP, INPUT_LEFT, INPUT_RIGHT, INPUT_UP,
};

pub fn input_system(_handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> BoxInput {
    let mut input: u8 = 0;

    if keyboard_input.pressed(KeyCode::Up) {
        input |= INPUT_UP;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        input |= INPUT_DOWN;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        input |= INPUT_LEFT;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        input |= INPUT_RIGHT;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        input |= INPUT_JUMP;
    }

    BoxInput { inp: input }
}
