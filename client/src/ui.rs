use crate::wrapped_natives::*;
use legion::systems::Builder;
use legion::*;

#[derive(Clone, Debug, PartialEq)]
struct TextEntry {
    text: String,
    pos_x: f32,
    pos_y: f32,
    scale: f32,
    color: (i32, i32, i32, i32),
}

pub fn run_initial() {}

pub fn run_on_tick(_resources: &mut Resources) {}

pub fn add_components(world: &mut World) {
    let _entity = world.push((TextEntry {
        text: "CryV".to_owned(),
        pos_x: 0.975,
        pos_y: 0.01,
        scale: 0.42,
        color: (200, 200, 200, 255),
    },));
}

pub fn add_resources(_resources: &mut Resources) {}

pub fn add_systems(builder: &mut Builder) {
    builder.add_thread_local(draw_text_entries_system());
}

#[system(for_each)]
fn draw_text_entries(text_entry: &TextEntry) {
    ui::draw_text(
        &text_entry.text,
        text_entry.pos_x,
        text_entry.pos_y,
        text_entry.scale,
        text_entry.color,
        true,
        1.0,
    );
}
