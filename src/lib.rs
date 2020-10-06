#![recursion_limit = "1024"]
extern crate winapi;

mod memory;
mod utility;

use memory::{address_fill, get_pattern, get_pattern_rip};
use std::io::prelude::*;

fn entrypoint() {
    let mut file = std::fs::File::create("E:\\Prototypes\\CryV\\tmp.txt").unwrap();

    let base = unsafe { winapi::um::libloaderapi::GetModuleHandleA(std::ptr::null()) };

    file.write(format!("Base: {:p}\n", base).as_bytes())
        .unwrap();

    let logos = get_pattern("70 6C 61 74 66 6F 72 6D 3A".to_string(), 0);
    let game_legal_skip = get_pattern("72 1F E8 ? ? ? ? 8B 0D".to_string(), 0);
    let frame_count = get_pattern_rip("8B 15 ? ? ? ? 41 FF CF".to_string(), 2);
    let story_mode_skip = get_pattern("48 83 3D ? ? ? ? ? 88 05 ? ? ? ? 75 0B".to_string(), 8);

    address_fill(game_legal_skip, 2, 0x90);
    address_fill(logos, 1, 0xC3);
    address_fill(story_mode_skip, 6, 0x90);

    file.write(format!("Logos: {:p}\n", logos).as_bytes())
        .unwrap();
    file.write(format!("GameLegalSkip: {:p}\n", game_legal_skip).as_bytes())
        .unwrap();
    file.write(format!("FrameCount: {:p}\n", frame_count).as_bytes())
        .unwrap();
    file.write(format!("StoryModeSkip: {:p}\n", story_mode_skip).as_bytes())
        .unwrap();

    file.write(format!("Done!\n").as_bytes()).unwrap();
}

make_entrypoint!(entrypoint);
