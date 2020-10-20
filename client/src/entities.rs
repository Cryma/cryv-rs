use hook::types::NativeVector3;

// The name is prefixed with 'CryV' as it would inevitably conflict with 'Entity' from bevy_ecs
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CryVEntity {
    pub handle: i32,
    pub model: u32,
    pub position: NativeVector3,
    pub rotation: NativeVector3, // TODO: Should probably use a quaternion
}

impl Default for CryVEntity {
    fn default() -> Self {
        CryVEntity {
            handle: 0,
            model: 0,
            position: NativeVector3::default(),
            rotation: NativeVector3::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vehicle {
    pub color_primary: i32,
    pub color_secondary: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ped;
