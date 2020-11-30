use std::{collections::HashMap, collections::VecDeque, sync::Mutex};

use once_cell::sync::{Lazy, OnceCell};
use winapi::{
    shared::minwindef::DWORD, shared::minwindef::LPARAM, shared::minwindef::LRESULT,
    shared::minwindef::UINT, shared::minwindef::WPARAM, shared::windef::HWND,
    um::sysinfoapi::GetTickCount, um::winuser::CallWindowProcA, um::winuser::FindWindowA,
    um::winuser::SetWindowLongPtrA, um::winuser::ToAscii, um::winuser::GWLP_WNDPROC,
    um::winuser::VK_CONTROL, um::winuser::VK_MENU, um::winuser::VK_SHIFT, um::winuser::WM_KEYDOWN,
    um::winuser::WM_KEYUP, um::winuser::WM_SYSKEYDOWN, um::winuser::WM_SYSKEYUP,
    um::winuser::WNDPROC,
};
#[derive(Copy, Clone, Debug)]
pub struct KeyboardCallbackState {
    pub key: u8,
    pub character: char,
    pub is_pressed: bool,
}

pub type KeyboardCallback = fn(KeyboardCallbackState);
pub type WindowProcCallback = fn(HWND, UINT, WPARAM, LPARAM);

#[derive(Copy, Clone, Debug)]
struct KeyState {
    time: u32,
    is_with_alt: bool,
    was_down_before: bool,
    is_up_now: bool,
}

impl Default for KeyState {
    fn default() -> Self {
        KeyState {
            time: 0,
            is_with_alt: false,
            was_down_before: false,
            is_up_now: false,
        }
    }
}

const KEYS_SIZE: u8 = 255;
const NOW_PERIOD: DWORD = 100;
const MAX_DOWN: DWORD = 5000;

static KEY_STATES: Lazy<Mutex<HashMap<u8, KeyState>>> = Lazy::new(|| {
    let mut key_states: HashMap<u8, KeyState> = HashMap::new();

    for i in 0..KEYS_SIZE {
        key_states.insert(i, KeyState::default());
    }

    Mutex::new(key_states)
});

static KEYBOAD_CALLBACKS: Lazy<Mutex<Vec<KeyboardCallback>>> = Lazy::new(|| Mutex::new(vec![]));
static WINDOW_PROC_CALLBACKS: Lazy<Mutex<Vec<WindowProcCallback>>> =
    Lazy::new(|| Mutex::new(vec![]));
static QUEUED_STATES: Lazy<Mutex<VecDeque<KeyboardCallbackState>>> =
    Lazy::new(|| Mutex::new(VecDeque::new()));
static WNDPROC: OnceCell<Mutex<WNDPROC>> = OnceCell::new();

pub fn initialize_keyboard_hook() {
    let window_name = std::ffi::CString::new("grcWindow").unwrap();
    let window = unsafe { FindWindowA(window_name.as_ptr(), std::ptr::null()) };

    WNDPROC.get_or_init(|| {
        Mutex::new(unsafe {
            std::mem::transmute(SetWindowLongPtrA(
                window,
                GWLP_WNDPROC,
                proc_window as isize,
            ))
        })
    });
}

pub fn is_key_pressed(key: u8) -> bool {
    if key > KEYS_SIZE {
        return false;
    }

    let key_states = KEY_STATES.lock().unwrap();
    let key_state = key_states.get(&key).unwrap();

    return (unsafe { GetTickCount() } < key_state.time + MAX_DOWN) && key_state.is_up_now == false;
}

pub fn is_key_released(key: u8, exclusive: bool) -> bool {
    if key > KEYS_SIZE {
        return false;
    }

    let mut key_states = KEY_STATES.lock().unwrap();
    let key_state = key_states.get(&key).unwrap();

    let is_released =
        unsafe { GetTickCount() } < key_state.time + NOW_PERIOD && key_state.is_up_now;

    if is_released && exclusive {
        reset_key_state(key, &mut key_states);
    }

    is_released
}

pub fn register_keyboard_callback(callback: KeyboardCallback) {
    let mut keyboard_callbacks = KEYBOAD_CALLBACKS.lock().unwrap();

    keyboard_callbacks.push(callback);
}

pub fn register_window_proc_callback(callback: WindowProcCallback) {
    let mut window_proc_callbacks = WINDOW_PROC_CALLBACKS.lock().unwrap();

    window_proc_callbacks.push(callback);
}

pub fn update_keyboard() {
    let mut queued_states = QUEUED_STATES.lock().unwrap();

    while queued_states.is_empty() == false {
        let state = queued_states.pop_front().unwrap();

        let keyboard_callbacks = KEYBOAD_CALLBACKS.lock().unwrap().clone();

        for callback in keyboard_callbacks {
            callback(state);
        }
    }
}

fn reset_key_state(key: u8, key_states: &mut HashMap<u8, KeyState>) {
    key_states.insert(key, KeyState::default());
}

fn char_for_key_state(key: u32, scan_code: u8, alt_pressed: bool) -> u16 {
    let mut states: [u8; 256] = [0; 256];

    if is_key_pressed(keycodes::KEY_SHIFT) {
        states[VK_SHIFT as usize] = 0xFF;
    }

    if alt_pressed {
        states[VK_CONTROL as usize] = 0xFF;
        states[VK_MENU as usize] = 0xFF;
    }

    let mut char_value: u16 = 0;

    if unsafe { ToAscii(key, scan_code as u32, states.as_ptr(), &mut char_value, 0) } <= 0 {
        return 0;
    }

    return char_value;
}

unsafe extern "system" fn proc_window(
    hwnd: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    wndproc(hwnd, message, w_param, l_param);

    if WNDPROC.get().is_none() {
        return 0;
    }

    let window_proc_callbacks = WINDOW_PROC_CALLBACKS.lock().unwrap().clone();

    for callback in window_proc_callbacks {
        callback(hwnd, message, w_param, l_param);
    }

    let wndproc = *WNDPROC.get().unwrap().lock().unwrap();
    CallWindowProcA(wndproc, hwnd, message, w_param, l_param)
}

fn wndproc(_: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) {
    if message == WM_KEYDOWN
        || message == WM_KEYUP
        || message == WM_SYSKEYDOWN
        || message == WM_SYSKEYUP
    {
        let w_param = w_param as i8;

        on_keyboard_message(
            w_param as u32,
            (l_param & 0xFFFF) as u16,
            ((l_param >> 16) & 0xFF) as u8,
            (l_param >> 24) & 1 > 0,
            message == WM_SYSKEYDOWN || message == WM_SYSKEYUP,
            (l_param >> 30) & 1 > 0,
            message == WM_SYSKEYUP || message == WM_KEYUP,
        );
    }
}

fn on_keyboard_message(
    key: u32,
    repeats: u16,
    scan_code: u8,
    _: bool,
    is_with_alt: bool,
    was_down_before: bool,
    is_up_now: bool,
) {
    if (key as u8) < KEYS_SIZE {
        let mut key_states = KEY_STATES.lock().unwrap();

        key_states.insert(
            key as u8,
            KeyState {
                time: unsafe { GetTickCount() },
                is_with_alt,
                was_down_before,
                is_up_now,
            },
        );
    }

    let mut queued_states = QUEUED_STATES.lock().unwrap();
    for _ in 0..repeats {
        queued_states.push_back(KeyboardCallbackState {
            key: key as u8,
            is_pressed: is_up_now == false,
            character: std::char::from_u32(char_for_key_state(key, scan_code, is_with_alt) as u32)
                .unwrap(),
        });
    }
}
