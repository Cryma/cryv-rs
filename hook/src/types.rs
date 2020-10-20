#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NativeVector3 {
    pub x: f32,
    padding_x: u32,
    pub y: f32,
    padding_y: u32,
    pub z: f32,
    padding_z: u32,
}

impl Default for NativeVector3 {
    fn default() -> Self {
        NativeVector3 {
            x: 0.0,
            padding_x: 0,
            y: 0.0,
            padding_y: 0,
            z: 0.0,
            padding_z: 0,
        }
    }
}

impl std::fmt::Display for NativeVector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}
