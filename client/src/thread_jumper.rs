use bevy::prelude::*;
use std::collections::VecDeque;

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
            .add_system(run_native_callbacks.thread_local_system());
    }
}

fn run_native_callbacks(_world: &mut World, resources: &mut Resources) {
    let thread_jumper_data = resources.get_mut::<ThreadJumperData>();

    match thread_jumper_data {
        Some(mut thread_jumper_data) => thread_jumper_data.work(),
        None => log::error!("Could not fetch thread jumper data in run_native_callbacks system."),
    }
}
