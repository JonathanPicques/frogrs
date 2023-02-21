use bytemuck::{Pod, Zeroable};

pub const INPUT_UP: u8 = 1 << 1;
pub const INPUT_DOWN: u8 = 1 << 2;
pub const INPUT_LEFT: u8 = 1 << 3;
pub const INPUT_RIGHT: u8 = 1 << 4;
pub const INPUT_JUMP: u8 = 1 << 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
pub struct BoxInput {
    pub inp: u8,
}
