use once_cell::sync::Lazy;
use shared::bevy::prelude::*;
use std::{collections::VecDeque, sync::Mutex};

pub type NativeCallback = Box<dyn Fn(&mut World) + Send + Sync>;

static NATIVE_CALLBACKS: Lazy<Mutex<Vec<NativeCallback>>> = Lazy::new(|| Mutex::new(vec![]));

pub struct ThreadJumperData {
    queue: VecDeque<Box<dyn Fn() + Send + Sync + 'static>>,
}

impl Default for ThreadJumperData {
    fn default() -> Self {
        ThreadJumperData {
            queue: VecDeque::new(),
        }
    }
}

impl<'a> ThreadJumperData {
    pub fn invoke(&mut self, callback: impl Fn() + Send + Sync + 'static) {
        self.queue.push_back(Box::new(callback));
    }

    fn work(&mut self) {
        while let Some(callback) = self.queue.pop_front() {
            callback();
        }

        self.queue.clear();
    }
}

pub struct ThreadJumperPlugin;

impl Plugin for ThreadJumperPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ThreadJumperData>()
            .add_system(run_native_callbacks.exclusive_system());
    }
}

fn run_native_callbacks(world: &mut World) {
    let mut native_callbacks = NATIVE_CALLBACKS.lock().unwrap();

    for callback in native_callbacks.as_slice() {
        callback(world);
    }

    native_callbacks.clear();

    let thread_jumper_data = world.get_resource_mut::<ThreadJumperData>();

    match thread_jumper_data {
        Some(mut thread_jumper_data) => thread_jumper_data.work(),
        None => log::error!("Could not fetch thread jumper data in run_native_callbacks system."),
    }
}

pub fn run(callback: NativeCallback) {
    let mut callbacks = NATIVE_CALLBACKS.lock().unwrap();

    callbacks.push(callback);
}
