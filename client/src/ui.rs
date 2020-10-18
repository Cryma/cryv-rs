use crate::wrapped_natives::*;
use bevy::ecs::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TextEntry {
    text: String,
    pos_x: f32,
    pos_y: f32,
    scale: f32,
    color: (i32, i32, i32, i32),
}

pub fn ui_startup_system(world: &mut World, _resources: &mut Resources) {
    world.spawn((TextEntry {
        text: "CryV".to_owned(),
        pos_x: 0.975,
        pos_y: 0.01,
        scale: 0.42,
        color: (200, 200, 200, 255),
    },));
}

pub fn draw_text_entries(world: &mut World, _resources: &mut Resources) {
    let mut text_entries = world.query::<&TextEntry>();

    for entry in text_entries.iter() {
        ui::draw_text(
            &entry.text,
            entry.pos_x,
            entry.pos_y,
            entry.scale,
            entry.color,
            true,
            1.0,
        );
    }
}
