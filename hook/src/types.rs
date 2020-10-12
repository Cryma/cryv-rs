#[derive(Copy, Clone)]
pub struct NativeVector3 {
    pub x: f32,
    pub padding_x: u32,
    pub y: f32,
    pub padding_y: u32,
    pub z: f32,
    pub padding_z: u32,
}

impl std::fmt::Display for NativeVector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}
