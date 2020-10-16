use hook::natives::*;
use std::ffi::CString;

pub fn draw_text(
    text: &str,
    pos_x: f32,
    pos_y: f32,
    scale: f32,
    color: (i32, i32, i32, i32),
    center: bool,
    text_wrap: f32,
) {
    hud::set_text_font(0);
    hud::set_text_scale(scale, scale);
    hud::set_text_colour(color.0, color.1, color.2, color.3);
    hud::set_text_wrap(0.0, text_wrap);
    hud::set_text_centre(center);
    hud::set_text_dropshadow(0, 0, 0, 0, 0);
    hud::set_text_edge(1, 0, 0, 0, 205);

    let text_type = CString::new("STRING").unwrap();

    let text = match CString::new(text) {
        Ok(val) => val,
        Err(error) => {
            log::error!(
                "Error while unwrapping text CString in draw_text: {}",
                error
            );

            return;
        }
    };

    hud::begin_text_command_display_text(&text_type);
    hud::add_text_component_substring_player_name(&text);
    hud::end_text_command_display_text(pos_x, pos_y, 1);
}

pub fn get_text_width(text: &str, scale: f32) -> f32 {
    hud::set_text_font(0);
    hud::set_text_scale(scale, scale);

    let text_type = CString::new("STRING").unwrap();
    let text = CString::new(text).unwrap();

    hud::_begin_text_command_get_width(&text_type);
    hud::add_text_component_substring_player_name(&text);

    hud::_end_text_command_get_width(true)
}
