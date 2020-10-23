#![forbid(unsafe_code)]

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_prototype_networking_laminar::NetworkingPlugin;
use log::info;
use once_cell::sync::Lazy;
use std::time::Duration;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

mod cleanup;
mod console;
mod entities;
mod thread_jumper;
mod ui;
mod utility;
mod wrapped_natives;

#[macro_export]
macro_rules! clone_query {
    ($query:expr) => {{
        let mut items = Vec::new();

        for item in &mut $query.iter() {
            items.push(item.clone());
        }

        items
    }};
}

static INSTALL_DIRECTORY: Lazy<String> = Lazy::new(|| {
    let key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(r"Software\CryV")
        .unwrap();

    let value: String = key.get_value("InstallDirectory").unwrap();

    value
});

make_entrypoint!(entrypoint);

fn entrypoint() {
    create_logger().expect("Something went wrong while creating the logger!");

    info!("--------------------------");
    info!("Starting CryV");

    hook::initialize(script_callback);

    info!("Successfully started CryV");
}

fn update_keyboard(_world: &mut World, _resources: &mut Resources) {
    hook::update_keyboard();
}

fn script_wait(_world: &mut World, _resources: &mut Resources) {
    hook::script_wait(0);
}

fn script_callback() {
    App::build()
        .add_plugin(ScheduleRunnerPlugin::run_loop(Duration::from_millis(0)))
        .add_system(update_keyboard.thread_local_system())
        .add_plugin(NetworkingPlugin)
        .add_plugin(cleanup::CleanupPlugin)
        .add_plugin(console::ConsolePlugin)
        .add_plugin(ui::UiPlugin)
        .add_system(script_wait.thread_local_system())
        .add_plugin(thread_jumper::ThreadJumperPlugin)
        .run();
}

fn create_logger() -> Result<(), fern::InitError> {
    let log_file_path = format!("{}/cryv.log", INSTALL_DIRECTORY.to_owned());

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file_path)?)
        .apply()?;

    Ok(())
}
