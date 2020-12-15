use std::sync::{atomic::AtomicBool, Mutex};

use once_cell::sync::Lazy;
use shared::bevy::ecs::{Entity, Resources, World};
use winapi::{
    shared::minwindef::LPARAM, shared::minwindef::UINT, shared::minwindef::WPARAM,
    shared::windef::HWND, um::winuser::FindWindowA,
};

use crate::{entities::EntityHandle, wrapped_natives::entities};

static mut SHOW_CURSOR: AtomicBool = AtomicBool::new(false);
static mut HAS_INIT: bool = false;
static LOG_BUFFER: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

static SPAWN_VEHICLE_MODAL: &str = "Spawn Vehicle\0";

pub(super) fn d3d11_present(
    device: *mut hook::winapi::um::d3d11::ID3D11Device,
    context: *mut hook::winapi::um::d3d11::ID3D11DeviceContext,
) {
    unsafe {
        if HAS_INIT == false {
            let window_name = std::ffi::CString::new("grcWindow").unwrap();
            let window = FindWindowA(window_name.as_ptr(), std::ptr::null());

            imgui_sys::igCreateContext(std::ptr::null_mut());
            imgui_sys::ImGui_ImplDX11_Init(
                device as *mut imgui_sys::ID3D11Device,
                context as *mut imgui_sys::ID3D11DeviceContext,
            );
            imgui_sys::ImGui_ImplWin32_Init(window as *mut std::ffi::c_void);

            add_log("Initialized D3D11_PRESENT hook");

            HAS_INIT = true;
        }

        imgui_sys::ImGui_ImplDX11_NewFrame();
        imgui_sys::ImGui_ImplWin32_NewFrame();
        imgui_sys::igNewFrame();

        draw_imgui();

        imgui_sys::igRender();
        imgui_sys::ImGui_ImplDX11_RenderDrawData(
            imgui_sys::igGetDrawData() as *mut std::ffi::c_void
        );
    };
}

pub(super) fn wndproc(hwnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) {
    unsafe {
        if *SHOW_CURSOR.get_mut() == false {
            return;
        }

        imgui_sys::ImGui_ImplWin32_WndProcHandler(
            hwnd as *mut std::ffi::c_void,
            message,
            w_param as *mut u32,
            l_param as i64,
        );
    };
}

pub(super) fn handle_cursor(_world: &mut World, _resources: &mut Resources) {
    if hook::is_key_released(hook::keycodes::KEY_F1, true) {
        unsafe {
            SHOW_CURSOR = AtomicBool::new(*SHOW_CURSOR.get_mut() == false);
        }
    }

    if unsafe { *SHOW_CURSOR.get_mut() } {
        hook::natives::pad::disable_all_control_actions(0);
    }
}

unsafe fn draw_imgui() {
    (*imgui_sys::igGetIO()).MouseDrawCursor = *SHOW_CURSOR.get_mut();

    let window_title = std::ffi::CString::new("CryV - Debug").unwrap();
    imgui_sys::igSetNextWindowSize(
        imgui_sys::ImVec2::new(350.0, 300.0),
        imgui_sys::ImGuiCond_Appearing,
    );

    if imgui_sys::igBegin(window_title.as_ptr(), std::ptr::null_mut(), 0) {
        draw_commands();
        draw_log();
    }

    draw_modals();

    imgui_sys::igEnd();
}

unsafe fn draw_commands() {
    let commands_text = std::ffi::CString::new("Commands").unwrap();
    if imgui_sys::igCollapsingHeaderBoolPtr(commands_text.as_ptr(), std::ptr::null_mut(), 0) {
        let spawn_vehicle_text = std::ffi::CString::new("Spawn Vehicle").unwrap();
        if imgui_sys::igButton(
            spawn_vehicle_text.as_ptr(),
            imgui_sys::ImVec2::new(0.0, 0.0),
        ) {
            imgui_sys::igOpenPopup(SPAWN_VEHICLE_MODAL.as_ptr() as *const i8, 0);
        }

        imgui_sys::igSameLine(0.0, -1.0);

        let delete_vehicle_text = std::ffi::CString::new("Delete Vehicle").unwrap();
        if imgui_sys::igButton(
            delete_vehicle_text.as_ptr(),
            imgui_sys::ImVec2::new(0.0, 0.0),
        ) {
            crate::thread_jumper::run(|world, _| {
                let player_ped_id = hook::natives::player::player_ped_id();
                let is_in_vehicle = hook::natives::ped::is_ped_in_any_vehicle(player_ped_id, false);

                if is_in_vehicle == false {
                    add_log("You are not in any vehicle");

                    return;
                }

                let mut vehicle_id =
                    hook::natives::ped::get_vehicle_ped_is_in(player_ped_id, false);
                // Copy the vehicle id, as GTA5 will clear the current one, after deleting the entity
                let vehicle_id_copy = vehicle_id.clone();

                entities::delete_entity(&mut vehicle_id);

                let existing_entities = world.query::<(Entity, &EntityHandle)>();

                let mut found_entity: Option<Entity> = None;

                for (entity, entity_data) in existing_entities {
                    if entity_data.handle != vehicle_id_copy {
                        continue;
                    }

                    found_entity = Some(entity);
                }

                if let Some(x) = found_entity {
                    world.despawn(x).unwrap();

                    add_log("Vehicle has been deleted");
                }
            });
        }
    }
}

unsafe fn draw_log() {
    let log_text = std::ffi::CString::new("Log").unwrap();
    if imgui_sys::igCollapsingHeaderBoolPtr(log_text.as_ptr(), &mut true, 0) {
        let buffer = LOG_BUFFER.lock().unwrap();
        let text = buffer.join("\n");
        let text_cstring = std::ffi::CString::new(text.clone()).unwrap();
        imgui_sys::igTextUnformatted(
            text_cstring.as_ptr(),
            text_cstring
                .as_ptr()
                .add(text.len() * std::mem::size_of::<std::os::raw::c_char>()),
        );
    }
}

unsafe fn draw_modals() {
    let imgui_io = *imgui_sys::igGetIO();
    let center = imgui_sys::ImVec2::new(imgui_io.DisplaySize.x * 0.5, imgui_io.DisplaySize.y * 0.5);
    imgui_sys::igSetNextWindowPos(
        center,
        imgui_sys::ImGuiCond_Appearing,
        imgui_sys::ImVec2::new(0.5, 0.5),
    );

    if imgui_sys::igBeginPopupModal(
        SPAWN_VEHICLE_MODAL.as_ptr() as *const i8,
        std::ptr::null_mut(),
        imgui_sys::ImGuiWindowFlags_AlwaysAutoResize,
    ) {
        static mut VEHICLE_NAME: [u8; 64] = [b'\0'; 64];
        imgui_sys::igInputText(
            SPAWN_VEHICLE_MODAL.as_ptr() as *const i8,
            VEHICLE_NAME.as_mut_ptr() as *mut i8,
            8 * 64,
            0,
            None,
            std::ptr::null_mut(),
        );

        static mut VEHICLE_COLOR_PRIMARY: i32 = 112;
        static mut VEHICLE_COLOR_SECONDARY: i32 = 112;
        imgui_sys::igInputInt(std::ffi::CString::new("Color1").unwrap().as_ptr(),
            &mut VEHICLE_COLOR_PRIMARY, 1, 1, 0);
        imgui_sys::igInputInt(std::ffi::CString::new("Color2").unwrap().as_ptr(),
            &mut VEHICLE_COLOR_SECONDARY, 1, 1, 0);

        imgui_sys::igSeparator();

        let done_text = std::ffi::CString::new("Done").unwrap();
        if imgui_sys::igButton(done_text.as_ptr(), imgui_sys::ImVec2::new(120.0, 0.0)) {
            crate::thread_jumper::run(|world, _| {
                let model_name_raw = std::ffi::CStr::from_bytes_with_nul_unchecked(&VEHICLE_NAME);
                let model_name = model_name_raw
                    .to_str()
                    .unwrap()
                    .trim_matches(char::from(0))
                    .to_owned();

                let player_ped_id = hook::natives::player::player_ped_id();
                let position = hook::natives::entity::get_entity_coords(player_ped_id, true);
                let rotation = hook::natives::entity::get_entity_rotation(player_ped_id, 2);

                let (handle, model, transform, vehicle) =
                    crate::entities::create_vehicle_with_model_name(
                        &model_name,
                        position,
                        rotation,
                        VEHICLE_COLOR_PRIMARY,
                        VEHICLE_COLOR_SECONDARY,
                    );

                hook::natives::ped::set_ped_into_vehicle(player_ped_id, handle.handle, -1);

                world.spawn((handle, model, transform, vehicle));

                add_log(&format!("Spawned vehicle ({})", model_name.to_uppercase()));
                VEHICLE_NAME = [b'\0'; 64];
            });

            imgui_sys::igCloseCurrentPopup();
        }

        imgui_sys::igEndPopup();
    }
}

fn add_log(message: &str) {
    let mut buffer = LOG_BUFFER.lock().unwrap();
    let time = chrono::Local::now();
    buffer.push(
        format!(
            "[{}] {}",
            time.format("%Y-%m-%d %H:%M:%S"),
            message.to_string()
        )
        .to_string(),
    );
}
