#![forbid(unsafe_code)]

use legion::*;
use log::info;
use once_cell::sync::Lazy;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

mod cleanup;
mod console;
mod generic;
mod modules;
mod ui;
mod utility;
mod wrapped_natives;

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

    let mut modules: Vec<Box<dyn modules::Module>> = vec![];

    modules.push(Box::new(cleanup::CleanupModule {}));
    modules.push(Box::new(console::ConsoleModule {}));
    modules.push(Box::new(generic::GenericModule {}));
    modules.push(Box::new(ui::UiModule {}));

    for module in &modules {
        module.run_initial();
        module.add_components(&mut world);
        module.add_resources(&mut resources);
        module.add_systems(&mut schedule_builder);
    }

    let mut schedule = schedule_builder.build();

    loop {
        hook::update_keyboard();

        schedule.execute(&mut world, &mut resources);

        for module in &modules {
            module.run_on_tick(&mut resources);
        }

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
