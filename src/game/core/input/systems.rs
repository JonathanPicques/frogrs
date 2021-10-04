use crate::game::core::input::structs::{
    INPUT_DOWN, INPUT_JUMP, INPUT_LEFT, INPUT_RIGHT, INPUT_UP,
};

use bevy::{
    input::Input,
    prelude::{In, KeyCode, Res},
};
use ggrs::PlayerHandle;

pub fn input_system(_handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> Vec<u8> {
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

    vec![input]
}
