use bevy::prelude::*;

#[derive(Default, Reflect)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vector2D {
    #[inline]
    pub fn set(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    #[inline]
    pub fn set_x(&mut self, value: f32) -> &mut Self {
        self.x = value;
        self
    }
    #[inline]
    pub fn set_y(&mut self, value: f32) -> &mut Self {
        self.y = value;
        self
    }
    pub fn set_all(&mut self, value: f32) -> &mut Self {
        self.x = value;
        self.y = value;
        self
    }

    #[inline]
    pub fn add(&mut self, x: f32, y: f32) -> &mut Self {
        self.x += x;
        self.y += y;
        self
    }
    #[inline]
    pub fn add_x(&mut self, value: f32) -> &mut Self {
        self.x += value;
        self
    }
    #[inline]
    pub fn add_y(&mut self, value: f32) -> &mut Self {
        self.y += value;
        self
    }
    pub fn add_all(&mut self, value: f32) -> &mut Self {
        self.x += value;
        self.y += value;
        self
    }
}

#[derive(Default, Reflect, Component)]
pub struct Transform2D {
    pub scale: Vector2D,
    pub position: Vector2D,
    pub rotation: f32,
}
