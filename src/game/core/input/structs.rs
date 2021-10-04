pub type InputType = u8;
pub const INPUT_SIZE: usize = std::mem::size_of::<InputType>();

pub const INPUT_UP: u8 = 1 << 1;
pub const INPUT_DOWN: u8 = 1 << 2;
pub const INPUT_LEFT: u8 = 1 << 3;
pub const INPUT_RIGHT: u8 = 1 << 4;
pub const INPUT_JUMP: u8 = 1 << 5;
