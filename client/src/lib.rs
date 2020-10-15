#![forbid(unsafe_code)]

use legion::*;
use log::info;
use once_cell::sync::Lazy;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

mod cleanup;
mod generic;
mod ui;
mod utility;

macro_rules! register_module {
    ($module:ident, $world:expr, $resources:expr, $schedule_builder:expr) => {{
        $module::run_initial();
        $module::add_components(&mut $world);
        $module::add_resources(&mut $resources);
        $module::add_systems(&mut $schedule_builder);
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

fn script_callback() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule_builder = Schedule::builder();

    register_module!(cleanup, world, resources, schedule_builder);
    register_module!(generic, world, resources, schedule_builder);
    register_module!(ui, world, resources, schedule_builder);

    let mut schedule = schedule_builder.build();

    loop {
        schedule.execute(&mut world, &mut resources);

        hook::script_wait(0);
    }
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
