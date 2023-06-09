// vector3.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn uniform(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}
