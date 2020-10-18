use crate::wrapped_natives::*;
use bevy::ecs::prelude::*;
use hook::natives::*;
use hook::KeyboardCallbackState;
use log::{error, info};
use once_cell::sync::Lazy;
use std::{collections::HashMap, collections::VecDeque, sync::Mutex, time::SystemTime};

mod commands;

type CommandCallback = fn(&mut World, &mut ConsoleData, &mut Vec<String>);

static KEY_EVENT_QUEUE: Lazy<Mutex<VecDeque<KeyboardCallbackState>>> =
    Lazy::new(|| Mutex::new(VecDeque::new()));
static COMMANDS: Lazy<HashMap<&str, CommandCallback>> = Lazy::new(|| {
    let mut commands: HashMap<&str, CommandCallback> = HashMap::new();

    commands.insert("veh", commands::command_veh);
    commands.insert("rmveh", commands::command_rmveh);

    commands
});

const BACKGROUND_INPUT_HEIGHT: f32 = 18.0;
const BACKGROUND_LINE_HEIGHT: f32 = 16.0;

pub struct ConsoleData {
    is_visible: bool,
    input: String,
    output: Vec<String>,
    output_lines: i32,
    blink_state: bool,
    last_blink_update: SystemTime,
    cursor_index: usize,
    input_history: Vec<String>,
    current_history_index: isize,
    command_queue: VecDeque<(Vec<String>, CommandCallback)>,
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
            command_queue: VecDeque::<(Vec<String>, CommandCallback)>::new(),
        }
    }
}

pub fn run_startup_system(_world: &mut World, _resources: &mut Resources) {
    hook::register_keyboard_callback(on_keyboard_callback);
}

pub fn run_on_tick(world: &mut World, resources: &mut Resources) {
    let mut data = match resources.get_mut::<ConsoleData>() {
        Some(value) => value,
        None => {
            error!("Can not find ConsoleData resource!");

            return;
        }
    };

    process_key_events(&mut data);

    while data.command_queue.is_empty() == false {
        let (mut arguments, callback) = data.command_queue.pop_front().unwrap();

        callback(world, &mut data, &mut arguments);
    }

    if hook::is_key_released(hook::keycodes::KEY_F1, true) {
        data.is_visible = !data.is_visible;
    }

    if data.is_visible == false {
        return;
    }

    pad::disable_all_control_actions(0);

    let mut width: i32 = 0;
    let mut height = 0;
    graphics::get_screen_resolution(&mut width, &mut height);

    let output_height = BACKGROUND_LINE_HEIGHT * data.output_lines as f32 / height as f32;
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

    for line in &data.output {
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
        &data.input,
        0.001,
        output_height,
        0.3,
        (255, 255, 255, 255),
        false,
        1.0,
    );

    let now = SystemTime::now();
    if now
        .duration_since(data.last_blink_update)
        .unwrap()
        .as_millis()
        > 500
    {
        data.blink_state = !data.blink_state;

        data.last_blink_update = now;
    }

    if data.blink_state {
        let input = &data.input[..data.cursor_index];
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

fn process_key_events(data: &mut ConsoleData) {
    let mut key_event_queue = match KEY_EVENT_QUEUE.try_lock() {
        Ok(val) => val,
        Err(error) => {
            error!(
                "Error while trying to lock key event queue in on_keyboard_callback: {}",
                error
            );

            return;
        }
    };

    let key_events = key_event_queue.clone();

    key_event_queue.clear();

    for state in key_events.iter() {
        if data.is_visible == false || state.is_pressed == false {
            return;
        }

        if state.key == hook::keycodes::KEY_RETURN {
            handle_command(data);

            data.input = String::default();
            data.current_history_index = -1;
            data.cursor_index = 0;

            return;
        }

        if state.key == hook::keycodes::KEY_BACK_SPACE {
            if data.input.len() == 0 || data.cursor_index == 0 {
                return;
            }

            let cursor_index = data.cursor_index;
            if cursor_index >= data.input.len() {
                data.input.pop();
            } else {
                data.input.remove(cursor_index);
            }

            data.cursor_index -= 1;

            return;
        }

        if state.key == hook::keycodes::KEY_DELETE {
            if data.input.len() == 0 || data.cursor_index == data.input.len() {
                return;
            }

            let cursor_index = data.cursor_index as usize;
            data.input.remove(cursor_index);

            return;
        }

        if state.key == hook::keycodes::KEY_END {
            data.cursor_index = data.input.len();

            return;
        }

        if state.key == hook::keycodes::KEY_HOME {
            data.cursor_index = 0;

            return;
        }

        if state.key == hook::keycodes::KEY_DOWN {
            if data.current_history_index <= 0 {
                return;
            }

            data.current_history_index -= 1;
            data.input = data.input_history[data.current_history_index as usize].clone();
            data.cursor_index = data.input.len();

            return;
        }

        if state.key == hook::keycodes::KEY_UP {
            if data.current_history_index + 1 >= data.input_history.len() as isize {
                return;
            }

            data.current_history_index += 1;
            data.input = data.input_history[data.current_history_index as usize].clone();
            data.cursor_index = data.input.len();

            return;
        }

        if state.key == hook::keycodes::KEY_LEFT {
            if data.cursor_index <= 0 {
                return;
            }

            data.cursor_index -= 1;

            return;
        }

        if state.key == hook::keycodes::KEY_RIGHT {
            if data.cursor_index >= data.input.len() {
                return;
            }

            data.cursor_index += 1;

            return;
        }

        if state.key < 32 || state.key > 111 {
            return;
        }

        let cursor_index = data.cursor_index;
        data.input.insert(cursor_index, state.character);
        data.cursor_index += 1;
    }
}

fn handle_command(data: &mut ConsoleData) {
    if data.input.is_empty() {
        return;
    }

    let input = data.input.clone();

    let command_array = input
        .split(' ')
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();

    let command_name = command_array.first().unwrap();
    let mut command_array = (&command_array[1..]).to_vec();
    command_array.reverse();

    data.input_history.insert(0, data.input.clone());

    if data.input_history.len() > 20 {
        data.input_history.pop();
    }

    match COMMANDS.get(command_name.as_str()) {
        Some(command) => data.command_queue.push_back((command_array, *command)),
        None => print_line(
            data,
            format!("~o~Unknown command: ~s~{}", command_name).as_str(),
        ),
    };
}

fn print_line(data: &mut ConsoleData, text: &str) {
    data.output.push(text.to_owned());

    while data.output.len() > data.output_lines as usize {
        data.output.remove(0);
    }

    info!("GameConsole: {}", text);
}

fn on_keyboard_callback(state: KeyboardCallbackState) {
    let mut key_event_queue = match KEY_EVENT_QUEUE.try_lock() {
        Ok(val) => val,
        Err(error) => {
            error!(
                "Error while trying to lock key event queue in on_keyboard_callback: {}",
                error
            );

            return;
        }
    };

    key_event_queue.push_back(state);
}
