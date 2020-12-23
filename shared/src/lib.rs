use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub use bevy;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct EntityTransform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub velocity: Vec3,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct EntityModel {
    pub model: u32,
}

impl std::fmt::Display for EntityModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#X}", self.model)
    }
}
