#![recursion_limit = "1024"]
extern crate winapi;

mod hook;
mod memory;
mod utility;

use log::{debug, info};

make_entrypoint!(entrypoint);

fn entrypoint() {
    create_logger().expect("Something went wrong while creating the logger!");

    info!("--------------------------");
    info!("Starting CryV");

    let base = unsafe { winapi::um::libloaderapi::GetModuleHandleA(std::ptr::null()) };
    debug!("Base: {:p}", base);

    hook::initialize();

    info!("Successfully started CryV");
}

fn create_logger() -> Result<(), fern::InitError> {
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
        .chain(fern::log_file(r"E:\Prototypes\CryV\cryv.log")?)
        .apply()?;

    Ok(())
}
