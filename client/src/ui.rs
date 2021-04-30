use crate::wrapped_natives::*;
use shared::bevy::prelude::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(ui_startup_system.exclusive_system())
            .add_system(draw_text_entries.exclusive_system());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextEntry {
    text: String,
    pos_x: f32,
    pos_y: f32,
    scale: f32,
    color: (i32, i32, i32, i32),
}

pub fn ui_startup_system(world: &mut World) {
    world.spawn().insert(TextEntry {
        text: "CryV".to_owned(),
        pos_x: 0.975,
        pos_y: 0.01,
        scale: 0.42,
        color: (200, 200, 200, 255),
    });
}

pub fn draw_text_entries(world: &mut World) {
    let mut text_entries = world.query::<&TextEntry>();

    for entry in text_entries.iter(world) {
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
