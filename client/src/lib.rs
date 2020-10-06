#![recursion_limit = "1024"]
extern crate winapi;

mod memory;
mod utility;

use log::{debug, info};
use memory::{address_fill, get_pattern, get_pattern_rip};

make_entrypoint!(entrypoint);

fn entrypoint() {
    create_logger().expect("Something went wrong while creating the logger!");

    let base = unsafe { winapi::um::libloaderapi::GetModuleHandleA(std::ptr::null()) };

    info!("--------------------------");
    info!("Starting CryV");

    debug!("Base: {:p}", base);

    let logos = get_pattern("70 6C 61 74 66 6F 72 6D 3A".to_string(), 0);
    let game_legal_skip = get_pattern("72 1F E8 ? ? ? ? 8B 0D".to_string(), 0);
    let frame_count = get_pattern_rip("8B 15 ? ? ? ? 41 FF CF".to_string(), 2);
    let story_mode_skip = get_pattern("48 83 3D ? ? ? ? ? 88 05 ? ? ? ? 75 0B".to_string(), 8);

    address_fill(game_legal_skip, 2, 0x90);
    address_fill(logos, 1, 0xC3);
    address_fill(story_mode_skip, 6, 0x90);

    debug!("Logos: {:p}", logos);
    debug!("GameLegalSkip: {:p}", game_legal_skip);
    debug!("FrameCount: {:p}", frame_count);
    debug!("StoryModeSkip: {:p}", story_mode_skip);

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
