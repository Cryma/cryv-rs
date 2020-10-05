#![recursion_limit = "1024"]
extern crate winapi;

mod memory;
mod utility;

use cpp::cpp;
use std::ffi::{c_void, CString};
use std::io::prelude::*;

fn get_pattern(pattern: String, add: i32) -> *mut c_void {
    let pattern_raw = CString::new(pattern).unwrap();
    let pattern_raw_pointer = pattern_raw.as_ptr();

    let data = unsafe {
        cpp!([pattern_raw_pointer as "const char *", add as "uint32_t"] -> *mut c_void as "char*" {
            auto data = Signature(pattern_raw_pointer).scan().add(add).as<char*>();

            return data;
        })
    };

    data
}

fn get_pattern_rip(pattern: String, add: i32) -> *mut c_void {
    let pattern_raw = CString::new(pattern).unwrap();
    let pattern_raw_pointer = pattern_raw.as_ptr();

    let data = unsafe {
        cpp!([pattern_raw_pointer as "const char *", add as "uint32_t"] -> *mut c_void as "char*" {
            auto data = Signature(pattern_raw_pointer).scan().add(add).rip().as<char*>();

            return data;
        })
    };

    data
}

fn entrypoint() {
    let mut file = std::fs::File::create("E:\\Prototypes\\CryV\\tmp.txt").unwrap();

    let base = unsafe { winapi::um::libloaderapi::GetModuleHandleA(std::ptr::null()) };

    file.write(format!("Base: {:p}\n", base).as_bytes())
        .unwrap();

    let logos = get_pattern("70 6C 61 74 66 6F 72 6D 3A".to_string(), 0);
    let game_legal_skip = get_pattern("72 1F E8 ? ? ? ? 8B 0D".to_string(), 0);
    let frame_count = get_pattern_rip("8B 15 ? ? ? ? 41 FF CF".to_string(), 2);

    let story_mode_skip = get_pattern("48 83 3D ? ? ? ? ? 88 05 ? ? ? ? 75 0B".to_string(), 8);
    unsafe {
        cpp!([game_legal_skip as "char*", logos as "char*", story_mode_skip as "char*"] {
            std::fill_n(game_legal_skip, 2, 0x90);
            std::fill_n(logos, 1, 0xC3);
            std::fill_n(story_mode_skip, 6, 0x90);
        });
    };

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
