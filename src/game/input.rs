use bevy::ecs::system::In;
use bevy::ecs::system::Res;
use bevy::input::keyboard::KeyCode;
use bevy::input::Input;
use ggrs::PlayerHandle;

pub type InputType = u8;
pub const INPUT_SIZE: usize = std::mem::size_of::<InputType>();

pub const INPUT_UP: u8 = 1 << 1;
pub const INPUT_DOWN: u8 = 1 << 2;
pub const INPUT_LEFT: u8 = 1 << 3;
pub const INPUT_RIGHT: u8 = 1 << 4;
pub const INPUT_JUMP: u8 = 1 << 5;

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
