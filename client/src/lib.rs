#![forbid(unsafe_code)]

use legion::*;
use log::info;
use once_cell::sync::Lazy;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

mod cleanup;
mod ui;
mod utility;

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
    cleanup::run_initial_cleanup();

    let mut world = World::default();
    ui::add_ui_components(&mut world);
    cleanup::add_cleanup_components(&mut world);

    let mut schedule_builder = Schedule::builder();
    ui::add_ui_systems(&mut schedule_builder);
    cleanup::add_cleanup_systems(&mut schedule_builder);

    let mut schedule = schedule_builder.build();
    let mut resources = Resources::default();

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
