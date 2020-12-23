// TODO: Move imgui related code to separate crate and disallow unsafe code
#![deny(unsafe_code)]

use log::info;
use once_cell::sync::Lazy;
use shared::bevy::{app::ScheduleRunnerPlugin, prelude::*};
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

mod cleanup;
mod entities;
#[allow(unsafe_code)]
mod imgui;
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

    hook::register_present_callback(imgui::d3d11_present);
    hook::register_window_proc_callback(imgui::wndproc);

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
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_system(update_keyboard.thread_local_system())
        .add_plugin(cleanup::CleanupPlugin)
        .add_plugin(ui::UiPlugin)
        .add_system(imgui::handle_cursor.thread_local_system())
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
