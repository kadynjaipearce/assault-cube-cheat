use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ViewMatrix {
    pub m: [[f32; 4]; 4],
}
