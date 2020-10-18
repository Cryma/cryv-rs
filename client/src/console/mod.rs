use crate::{modules::Module, wrapped_natives::*};
use hook::natives::*;
use hook::KeyboardCallbackState;
use legion::*;
use log::{error, info};
use once_cell::sync::Lazy;
use std::{collections::HashMap, collections::VecDeque, sync::Mutex, time::SystemTime};

mod commands;

type CommandCallback = fn(&mut World, &mut ConsoleModule, &mut Vec<String>);

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

pub struct ConsoleModule {
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

impl Default for ConsoleModule {
    fn default() -> Self {
        ConsoleModule {
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

impl Module for ConsoleModule {
    fn run_initial(&mut self) {
        hook::register_keyboard_callback(on_keyboard_callback);
    }

    fn run_on_tick(&mut self, world: &mut World, _resources: &mut Resources) {
        self.process_key_events();

        while self.command_queue.is_empty() == false {
            let (mut arguments, callback) = self.command_queue.pop_front().unwrap();

            callback(world, self, &mut arguments);
        }

        if hook::is_key_released(hook::keycodes::KEY_F1, true) {
            self.is_visible = !self.is_visible;
        }

        if self.is_visible == false {
            return;
        }

        pad::disable_all_control_actions(0);

        let mut width: i32 = 0;
        let mut height = 0;
        graphics::get_screen_resolution(&mut width, &mut height);

        let output_height = BACKGROUND_LINE_HEIGHT * self.output_lines as f32 / height as f32;
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

        for line in &self.output {
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
            &self.input,
            0.001,
            output_height,
            0.3,
            (255, 255, 255, 255),
            false,
            1.0,
        );

        let now = SystemTime::now();
        if now
            .duration_since(self.last_blink_update)
            .unwrap()
            .as_millis()
            > 500
        {
            self.blink_state = !self.blink_state;

            self.last_blink_update = now;
        }

        if self.blink_state {
            let input = &self.input[..self.cursor_index];
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
}

impl ConsoleModule {
    fn process_key_events(&mut self) {
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
            if self.is_visible == false || state.is_pressed == false {
                return;
            }

            if state.key == hook::keycodes::KEY_RETURN {
                self.handle_command();

                self.input = String::default();
                self.current_history_index = -1;
                self.cursor_index = 0;

                return;
            }

            if state.key == hook::keycodes::KEY_BACK_SPACE {
                if self.input.len() == 0 || self.cursor_index == 0 {
                    return;
                }

                let cursor_index = self.cursor_index;
                if cursor_index >= self.input.len() {
                    self.input.pop();
                } else {
                    self.input.remove(cursor_index);
                }

                self.cursor_index -= 1;

                return;
            }

            if state.key == hook::keycodes::KEY_DELETE {
                if self.input.len() == 0 || self.cursor_index == self.input.len() {
                    return;
                }

                let cursor_index = self.cursor_index as usize;
                self.input.remove(cursor_index);

                return;
            }

            if state.key == hook::keycodes::KEY_END {
                self.cursor_index = self.input.len();

                return;
            }

            if state.key == hook::keycodes::KEY_HOME {
                self.cursor_index = 0;

                return;
            }

            if state.key == hook::keycodes::KEY_DOWN {
                if self.current_history_index <= 0 {
                    return;
                }

                self.current_history_index -= 1;
                self.input = self.input_history[self.current_history_index as usize].clone();
                self.cursor_index = self.input.len();

                return;
            }

            if state.key == hook::keycodes::KEY_UP {
                if self.current_history_index + 1 >= self.input_history.len() as isize {
                    return;
                }

                self.current_history_index += 1;
                self.input = self.input_history[self.current_history_index as usize].clone();
                self.cursor_index = self.input.len();

                return;
            }

            if state.key == hook::keycodes::KEY_LEFT {
                if self.cursor_index <= 0 {
                    return;
                }

                self.cursor_index -= 1;

                return;
            }

            if state.key == hook::keycodes::KEY_RIGHT {
                if self.cursor_index >= self.input.len() {
                    return;
                }

                self.cursor_index += 1;

                return;
            }

            if state.key < 32 || state.key > 111 {
                return;
            }

            let cursor_index = self.cursor_index;
            self.input.insert(cursor_index, state.character);
            self.cursor_index += 1;
        }
    }

    fn handle_command(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let input = self.input.clone();

        let command_array = input
            .split(' ')
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        let command_name = command_array.first().unwrap();
        let mut command_array = (&command_array[1..]).to_vec();
        command_array.reverse();

        self.input_history.insert(0, self.input.clone());

        if self.input_history.len() > 20 {
            self.input_history.pop();
        }

        match COMMANDS.get(command_name.as_str()) {
            Some(command) => self.command_queue.push_back((command_array, *command)),
            None => self.print_line(format!("~o~Unknown command: ~s~{}", command_name).as_str()),
        };
    }

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
