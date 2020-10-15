use hook::natives::*;
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
    draw_text(
        &text_entry.text,
        text_entry.pos_x,
        text_entry.pos_y,
        text_entry.scale,
        text_entry.color,
    );
}

fn draw_text(text: &str, pos_x: f32, pos_y: f32, scale: f32, color: (i32, i32, i32, i32)) {
    hud::set_text_font(0);
    hud::set_text_scale(scale, scale);
    hud::set_text_colour(color.0, color.1, color.2, color.3);
    hud::set_text_wrap(0.0, 1.0);
    hud::set_text_centre(true);
    hud::set_text_dropshadow(0, 0, 0, 0, 0);
    hud::set_text_edge(1, 0, 0, 0, 205);

    let text_type = std::ffi::CString::new("STRING").unwrap();
    let text = std::ffi::CString::new(text).unwrap();

    hud::begin_text_command_display_text(&text_type);
    hud::add_text_component_substring_player_name(&text);
    hud::end_text_command_display_text(pos_x, pos_y, 1);
}
