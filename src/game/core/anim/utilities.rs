pub fn speed_as_secs(fps: u32, secs: f32) -> u8 {
    return (secs * fps as f32) as u8;
}
