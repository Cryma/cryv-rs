use crate::memory::{address_fill, get_pattern, get_pattern_rip};
use cpp::cpp;
use log::debug;
use std::ffi::c_void;

struct Pointers {
    logos: *mut c_void,
    game_legal_skip: *mut c_void,
    frame_count: *mut c_void,
    game_state: *mut c_void,
    swapchain: *mut c_void,
    address_to_entity: *mut c_void,
    registration_table: *mut c_void,
    get_label_text: *mut c_void,
    replay_interface: *mut c_void,
    set_vector_results: *mut c_void,
    story_mode_skip: *mut c_void,
    model_check_skip: *mut c_void,
    model_spawn_fix: *mut c_void,
}

impl Pointers {
    const INIT: Self = Pointers {
        logos: std::ptr::null_mut(),
        game_legal_skip: std::ptr::null_mut(),
        frame_count: std::ptr::null_mut(),
        game_state: std::ptr::null_mut(),
        swapchain: std::ptr::null_mut(),
        address_to_entity: std::ptr::null_mut(),
        registration_table: std::ptr::null_mut(),
        get_label_text: std::ptr::null_mut(),
        replay_interface: std::ptr::null_mut(),
        set_vector_results: std::ptr::null_mut(),
        story_mode_skip: std::ptr::null_mut(),
        model_check_skip: std::ptr::null_mut(),
        model_spawn_fix: std::ptr::null_mut(),
    };
}

static mut POINTERS: Pointers = Pointers::INIT;

pub fn initialize() {
    unsafe {
        POINTERS.logos = get_pattern("70 6C 61 74 66 6F 72 6D 3A".to_owned(), 0);
        debug!("Logos: {:p}", POINTERS.logos);
        address_fill(POINTERS.logos, 1, 0xC3);

        POINTERS.game_legal_skip = get_pattern("72 1F E8 ? ? ? ? 8B 0D".to_owned(), 0);
        debug!("GameLegalSkip: {:p}", POINTERS.game_legal_skip);
        address_fill(POINTERS.game_legal_skip, 2, 0x90);

        POINTERS.frame_count = get_pattern("8B 15 ? ? ? ? 41 FF CF".to_owned(), 2);
        debug!("FrameCount: {:p}", POINTERS.frame_count);

        POINTERS.game_state = get_pattern_rip("83 3D ? ? ? ? ? 8A D9 74 0A".to_owned(), 2);
        debug!("GameState: {:p}", POINTERS.game_state);

        POINTERS.swapchain = get_pattern_rip("48 8B 0D ? ? ? ? 48 8D 55 A0 48 8B 01".to_owned(), 3);
        debug!("Swapchain: {:p}", POINTERS.swapchain);

        POINTERS.address_to_entity = get_pattern(
            "48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC 20 8B 15 ? ? ? ? 48 8B F9 48 83 C1 10 33 DB"
                .to_owned(),
            0,
        );
        debug!("AddressToEntity: {:p}", POINTERS.address_to_entity);

        POINTERS.registration_table = get_pattern_rip("76 32 48 8B 53 40 48 8D 0D".to_owned(), 9);
        debug!("RegistrationTable: {:p}", POINTERS.registration_table);

        POINTERS.get_label_text = get_pattern("75 ? E8 ? ? ? ? 8B 0D ? ? ? ? 65 48 8B 04 25 ? ? ? ? BA ? ? ? ? 48 8B 04 C8 8B 0C 02 D1 E9".to_owned(), -19);
        debug!("GetLabelText: {:p}", POINTERS.get_label_text);

        let replay_interface = get_pattern("48 8B 05 ? ? ? ? 41 8B 1E".to_owned(), 0).add(0xEE);
        POINTERS.replay_interface = cpp!([replay_interface as "char*"] -> *mut c_void as "char*" {
            return (replay_interface + *(DWORD*) (replay_interface + 0x3) + 0x7); // TODO: Implement in rust
        });
        debug!("ReplayInterface: {:p}", POINTERS.replay_interface);

        POINTERS.set_vector_results =
            get_pattern("83 79 18 00 48 8B D1 74 4A FF 4A 18".to_owned(), 0);
        debug!("SetVectorResults: {:p}", POINTERS.set_vector_results);

        POINTERS.story_mode_skip =
            get_pattern("48 83 3D ? ? ? ? ? 88 05 ? ? ? ? 75 0B".to_owned(), 8);
        debug!("StoryModeSkip: {:p}", POINTERS.story_mode_skip);
        address_fill(POINTERS.story_mode_skip, 6, 0x90);

        POINTERS.model_check_skip = get_pattern("48 85 C0 0F 84 ? ? ? ? 8B 48 50".to_owned(), 0);
        debug!("ModelCheckSkip: {:p}", POINTERS.model_check_skip);
        address_fill(POINTERS.model_check_skip, 24, 0x90);

        POINTERS.model_spawn_fix = get_pattern("48 8B C8 FF 52 30 84 C0 74 05 48".to_owned(), 8);
        debug!("ModelSpawnFix: {:p}", POINTERS.model_spawn_fix);
        address_fill(POINTERS.model_spawn_fix, 2, 0x90);
    };
}
