use crate::{thread_jumper::ThreadJumperData, wrapped_natives::*};
use bevy::prelude::*;
use hook::natives::*;
use hook::KeyboardCallbackState;
use log::{error, info};
use once_cell::sync::Lazy;
use std::{collections::VecDeque, sync::Mutex, time::SystemTime};

mod commands;

static KEY_EVENT_QUEUE: Lazy<Mutex<VecDeque<KeyboardCallbackState>>> =
    Lazy::new(|| Mutex::new(VecDeque::new()));

const BACKGROUND_INPUT_HEIGHT: f32 = 18.0;
const BACKGROUND_LINE_HEIGHT: f32 = 16.0;

pub struct ConsolePlugin;
impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_event::<CommandEvent>()
            .init_resource::<ConsoleData>()
            .init_resource::<EventListenerState>()
            .add_startup_system(run_startup_system.thread_local_system())
            .add_system(run_on_tick.system())
            .add_system(command_event_listener.thread_local_system());
    }
}

#[derive(Clone)]
pub struct ConsoleData {
    pub is_visible: bool,
    input: String,
    output: Vec<String>,
    output_lines: i32,
    blink_state: bool,
    last_blink_update: SystemTime,
    cursor_index: usize,
    input_history: Vec<String>,
    current_history_index: isize,
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
        }
    }
}

pub fn run_startup_system(_world: &mut World, _resources: &mut Resources) {
    hook::register_keyboard_callback(on_keyboard_callback);
}

#[derive(Debug, Clone)]
pub enum Commands {
    Veh(Vec<String>),
    Rmveh(Vec<String>),
    Connect(Vec<String>),
    NotFound(String),
}

#[derive(Debug)]
pub struct CommandEvent {
    pub command_type: Commands,
}

#[derive(Default)]
pub struct EventListenerState {
    command_event_reader: EventReader<CommandEvent>,
}

fn command_event_listener(world: &mut World, resources: &mut Resources) {
    let mut commands: Vec<Commands> = vec![];

    {
        let events = resources.get::<Events<CommandEvent>>().unwrap();
        let mut state = resources.get_mut::<EventListenerState>().unwrap();

        for event in state.command_event_reader.iter(&events) {
            commands.push(event.command_type.clone());
        }
    }

    for command in &commands {
        match command {
            Commands::Veh(arguments) => commands::command_veh(world, resources, arguments.clone()),
            Commands::Rmveh(arguments) => {
                commands::command_rmveh(world, resources, arguments.clone())
            }
            Commands::Connect(arguments) => {
                commands::command_connect(world, resources, arguments.clone())
            }
            Commands::NotFound(command_name) => {
                let mut console_data = resources.get_mut::<ConsoleData>().unwrap();
                console_data
                    .print_line(format!("~o~Unknown command: ~s~{}", command_name).as_str());
            }
        }
    }
}

pub fn run_on_tick(
    mut thread_jumper: ResMut<ThreadJumperData>,
    mut data: ResMut<ConsoleData>,
    events: ResMut<Events<CommandEvent>>,
) {
    process_key_events(&mut data, events);

    if hook::is_key_released(hook::keycodes::KEY_F1, true) {
        data.is_visible = !data.is_visible;
    }

    if data.is_visible == false {
        return;
    }

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

    let readonly_data = data.clone();

    thread_jumper.invoke(move || {
        pad::disable_all_control_actions(0);

        let mut width: i32 = 0;
        let mut height = 0;
        graphics::get_screen_resolution(&mut width, &mut height);

        let output_height =
            BACKGROUND_LINE_HEIGHT * readonly_data.output_lines as f32 / height as f32;
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

        for line in &readonly_data.output {
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
            &readonly_data.input,
            0.001,
            output_height,
            0.3,
            (255, 255, 255, 255),
            false,
            1.0,
        );

        if readonly_data.blink_state {
            let input = &readonly_data.input[..readonly_data.cursor_index];
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
    });
}

fn process_key_events(data: &mut ConsoleData, events: ResMut<Events<CommandEvent>>) {
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
            handle_command(data, events);

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

        match state.key {
            0x20 | 0x30..=0x5A | 0x60..=0x6F | 0xA0..=0xB0 | 0xBC..=0xDE => (),
            _ => return,
        }

        let cursor_index = data.cursor_index;
        data.input.insert(cursor_index, state.character);
        data.cursor_index += 1;
    }
}

fn handle_command(data: &mut ConsoleData, mut events: ResMut<Events<CommandEvent>>) {
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

    events.send(CommandEvent {
        command_type: {
            match command_name.as_str() {
                "veh" => Commands::Veh(command_array),
                "rmveh" => Commands::Rmveh(command_array),
                "connect" => Commands::Connect(command_array),
                _ => Commands::NotFound(command_name.to_owned()),
            }
        },
    })
}

impl ConsoleData {
    fn print_line(&mut self, text: &str) {
        self.output.push(text.to_owned());

        while self.output.len() > self.output_lines as usize {
            self.output.remove(0);
        }

        info!("GameConsole: {}", text);
    }
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
