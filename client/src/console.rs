use std::{collections::HashMap, sync::Mutex, time::SystemTime};

use crate::wrapped_natives::*;
use hook::natives::*;
use hook::KeyboardCallbackState;
use legion::systems::Builder;
use legion::*;
use once_cell::sync::Lazy;

static CONSOLE_DATA: Lazy<Mutex<ConsoleData>> = Lazy::new(|| Mutex::new(ConsoleData::default()));

const BACKGROUND_INPUT_HEIGHT: f32 = 18.0;
const BACKGROUND_LINE_HEIGHT: f32 = 16.0;

#[derive(Clone, Debug)]
struct ConsoleData {
    is_visible: bool,
    input: String,
    output: Vec<String>,
    output_lines: i32,
    blink_state: bool,
    last_blink_update: SystemTime,
    cursor_index: usize,
    input_history: Vec<String>,
    current_history_index: isize,
    commands: HashMap<String, fn()>,
}

impl Default for ConsoleData {
    fn default() -> Self {
        ConsoleData {
            is_visible: false,
            input: "".to_owned(),
            output: Vec::new(),
            output_lines: 5,
            blink_state: false,
            last_blink_update: SystemTime::UNIX_EPOCH,
            cursor_index: 0,
            input_history: Vec::new(),
            current_history_index: -1,
            commands: HashMap::new(),
        }
    }
}

pub fn run_initial() {
    hook::register_keyboard_callback(on_keyboard_callback);
}

pub fn run_on_tick(_resources: &mut Resources) {
    let mut console_data = match CONSOLE_DATA.try_lock() {
        Ok(val) => val,
        Err(error) => {
            log::error!("no: {}", error);
            return;
        }
    };

    if hook::is_key_released(hook::keycodes::KEY_F1, true) {
        console_data.is_visible = !console_data.is_visible;
    }

    if console_data.is_visible == false {
        return;
    }

    pad::disable_all_control_actions(0);

    let mut width: i32 = 0;
    let mut height = 0;
    graphics::get_screen_resolution(&mut width, &mut height);

    let output_height = BACKGROUND_LINE_HEIGHT * console_data.output_lines as f32 / height as f32;
    let input_height = BACKGROUND_INPUT_HEIGHT / height as f32;

    graphics::draw_rect(
        0.5,
        output_height / 2.0,
        1.0,
        output_height,
        50,
        50,
        50,
        150,
        false,
    );

    graphics::draw_rect(
        0.5,
        output_height + input_height / 2.0,
        1.0,
        input_height,
        0,
        0,
        0,
        150,
        false,
    );

    let mut count = 0;

    for line in &console_data.output {
        ui::draw_text(
            line,
            0.001,
            BACKGROUND_LINE_HEIGHT * count as f32 / height as f32,
            0.3,
            (255, 255, 255, 255),
            false,
            1.0,
        );

        count += 1;
    }

    ui::draw_text(
        &console_data.input,
        0.001,
        output_height,
        0.3,
        (255, 255, 255, 255),
        false,
        1.0,
    );

    let now = SystemTime::now();
    if now
        .duration_since(console_data.last_blink_update)
        .unwrap()
        .as_millis()
        > 500
    {
        console_data.blink_state = !console_data.blink_state;

        console_data.last_blink_update = now;
    }

    if console_data.blink_state {
        let input = &console_data.input[..console_data.cursor_index];
        let text_width = ui::get_text_width(input, 0.3);

        graphics::draw_rect(
            text_width - 0.0005,
            output_height + input_height / 2.0,
            0.001,
            input_height * 0.8,
            255,
            255,
            255,
            200,
            false,
        );
    }
}

pub fn add_components(_world: &mut World) {}

pub fn add_resources(_resources: &mut Resources) {}

pub fn add_systems(_builder: &mut Builder) {}

fn on_keyboard_callback(state: KeyboardCallbackState) {
    let mut console_data = match CONSOLE_DATA.try_lock() {
        Ok(val) => val,
        Err(error) => {
            log::error!("x_no: {}", error);
            return;
        }
    };

    if console_data.is_visible == false || state.is_pressed == false {
        return;
    }

    if state.key == hook::keycodes::KEY_ENTER {
        // TODO: Handle command

        console_data.input = String::default();
        console_data.current_history_index = -1;
        console_data.cursor_index = 0;

        return;
    }

    if state.key == hook::keycodes::KEY_BACK_SPACE {
        if console_data.input.len() == 0 || console_data.cursor_index == 0 {
            return;
        }

        let cursor_index = console_data.cursor_index;
        if cursor_index >= console_data.input.len() {
            console_data.input.pop();
        } else {
            console_data.input.remove(cursor_index);
        }

        console_data.cursor_index -= 1;

        return;
    }

    if state.key == hook::keycodes::KEY_DELETE {
        if console_data.input.len() == 0 || console_data.cursor_index == console_data.input.len() {
            return;
        }

        let cursor_index = console_data.cursor_index as usize;
        console_data.input.remove(cursor_index);

        return;
    }

    if state.key == hook::keycodes::KEY_END {
        console_data.cursor_index = console_data.input.len();

        return;
    }

    if state.key == hook::keycodes::KEY_HOME {
        console_data.cursor_index = 0;

        return;
    }

    if state.key == hook::keycodes::KEY_DOWN {
        if console_data.current_history_index <= 0 {
            return;
        }

        console_data.current_history_index -= 1;
        console_data.input =
            console_data.input_history[console_data.current_history_index as usize].clone();
        console_data.cursor_index = console_data.input.len();

        return;
    }

    if state.key == hook::keycodes::KEY_UP {
        if console_data.current_history_index + 1 >= console_data.input_history.len() as isize {
            return;
        }

        console_data.current_history_index += 1;
        console_data.input =
            console_data.input_history[console_data.current_history_index as usize].clone();
        console_data.cursor_index = console_data.input.len();

        return;
    }

    if state.key == hook::keycodes::KEY_LEFT {
        if console_data.cursor_index <= 0 {
            return;
        }

        console_data.cursor_index -= 1;

        return;
    }

    if state.key == hook::keycodes::KEY_RIGHT {
        if console_data.cursor_index >= console_data.input.len() {
            return;
        }

        console_data.cursor_index += 1;

        return;
    }

    if state.key < 32 || state.key > 111 {
        return;
    }

    let cursor_index = console_data.cursor_index;
    console_data.input.insert(cursor_index, state.character);
    console_data.cursor_index += 1;
}
