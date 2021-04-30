use std::sync::{atomic::AtomicBool, Mutex};

use once_cell::sync::{Lazy, OnceCell};
use shared::bevy::prelude::Entity;
use winapi::{
    shared::minwindef::LPARAM, shared::minwindef::UINT, shared::minwindef::WPARAM,
    shared::windef::HWND, um::winuser::FindWindowA,
};

use crate::{entities::EntityHandle, wrapped_natives::entities};

static mut SHOW_CURSOR: AtomicBool = AtomicBool::new(false);
static LOG_BUFFER: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

static mut CTX: OnceCell<Mutex<ContextWrapper>> = OnceCell::new();
static mut IMGUI_DATA: OnceCell<ImguiData> = OnceCell::new();

#[derive(Debug)]
struct ContextWrapper {
    pub ctx: imgui::Context,
}

#[derive(Debug)]
struct ImguiData {
    pub show_commands: bool,
    pub show_logs: bool,
    pub vehicle_model: imgui::ImString,
    pub primary_color: i32,
    pub secondary_color: i32,
}

impl Default for ImguiData {
    fn default() -> Self {
        ImguiData {
            show_commands: false,
            show_logs: false,
            vehicle_model: imgui::ImString::with_capacity(32),
            primary_color: 112,
            secondary_color: 112,
        }
    }
}

unsafe impl Send for ContextWrapper {}

fn get_context(
    device: *mut hook::winapi::um::d3d11::ID3D11Device,
    context: *mut hook::winapi::um::d3d11::ID3D11DeviceContext,
) -> &'static Mutex<ContextWrapper> {
    unsafe {
        CTX.get_or_init(|| {
            let window_name = std::ffi::CString::new("grcWindow").unwrap();
            let window = FindWindowA(window_name.as_ptr(), std::ptr::null());

            let ctx = imgui::Context::create();

            imgui_sys::ImGui_ImplDX11_Init(
                device as *mut imgui_sys::ID3D11Device,
                context as *mut imgui_sys::ID3D11DeviceContext,
            );
            imgui_sys::ImGui_ImplWin32_Init(window as *mut std::ffi::c_void);

            add_log("Initialized D3D11_PRESENT hook");

            Mutex::new(ContextWrapper { ctx })
        })
    }
}

fn get_data<'a>() -> &'a mut ImguiData {
    unsafe {
        if let None = IMGUI_DATA.get() {
            IMGUI_DATA.set(ImguiData::default()).unwrap();
        }

        IMGUI_DATA.get_mut().unwrap()
    }
}

pub(super) fn d3d11_present(
    device: *mut hook::winapi::um::d3d11::ID3D11Device,
    context: *mut hook::winapi::um::d3d11::ID3D11DeviceContext,
) {
    let mut wrapper = get_context(device, context).lock().unwrap();
    let data = get_data();

    unsafe {
        imgui_sys::ImGui_ImplDX11_NewFrame();
        imgui_sys::ImGui_ImplWin32_NewFrame();
    };

    let ui = wrapper.ctx.frame();

    draw(&ui, data);

    unsafe {
        (*imgui_sys::igGetIO()).MouseDrawCursor = *SHOW_CURSOR.get_mut();
        let draw_data = ui.render() as *const imgui::DrawData;
        imgui_sys::ImGui_ImplDX11_RenderDrawData(
            (draw_data as *const std::ffi::c_void) as *mut std::ffi::c_void,
        );
    };
}

fn draw(ui: &imgui::Ui, data: &mut ImguiData) {
    ui.main_menu_bar(|| {
        ui.menu(imgui::im_str!("CryV"), true, || {});

        ui.menu(imgui::im_str!("Windows"), true, || {
            if imgui::MenuItem::new(imgui::im_str!("Commands"))
                .selected(data.show_commands)
                .build(&ui)
            {
                data.show_commands = !data.show_commands;
            }

            if imgui::MenuItem::new(imgui::im_str!("Logs"))
                .selected(data.show_logs)
                .build(&ui)
            {
                data.show_logs = !data.show_logs;
            }
        });
    });

    if data.show_commands {
        imgui::Window::new(imgui::im_str!("Commands"))
            .size([300.0, 100.0], imgui::Condition::FirstUseEver)
            .build(&ui, || {
                if ui.button(imgui::im_str!("Spawn Vehicle"), [0.0, 0.0]) {
                    ui.open_popup(imgui::im_str!("Spawn Vehicle"));
                }

                ui.same_line_with_spacing(0.0, -1.0);

                if ui.button(imgui::im_str!("Delete Vehicle"), [0.0, 0.0]) {
                    crate::thread_jumper::run(Box::new(|world| {
                        let player_ped_id = hook::natives::player::player_ped_id();
                        let is_in_vehicle =
                            hook::natives::ped::is_ped_in_any_vehicle(player_ped_id, false);

                        if is_in_vehicle == false {
                            add_log("You are not in any vehicle");

                            return;
                        }

                        let mut vehicle_id =
                            hook::natives::ped::get_vehicle_ped_is_in(player_ped_id, false);
                        // Copy the vehicle id, as GTA5 will clear the current one, after deleting the entity
                        let vehicle_id_copy = vehicle_id.clone();

                        entities::delete_entity(&mut vehicle_id);

                        let mut existing_entities = world.query::<(Entity, &EntityHandle)>();

                        let mut found_entity: Option<Entity> = None;

                        for (entity, entity_data) in existing_entities.iter(world) {
                            if entity_data.handle != vehicle_id_copy {
                                continue;
                            }

                            found_entity = Some(entity);
                        }

                        if let Some(x) = found_entity {
                            world.despawn(x);

                            add_log("Vehicle has been deleted");
                        }
                    }));
                }

                if ui.button(imgui::im_str!("Load Cayo Perico"), [0.0, 0.0]) {
                    crate::thread_jumper::run(Box::new(|_| {
                        let heist_island_cstring = std::ffi::CString::new("HeistIsland").unwrap();
                        hook::natives::streaming::_set_island_hopper_enabled(
                            &heist_island_cstring,
                            true,
                        );
                    }));
                }

                ui.same_line_with_spacing(0.0, -1.0);

                if ui.button(imgui::im_str!("Unload Cayo Perico"), [0.0, 0.0]) {
                    crate::thread_jumper::run(Box::new(|_| {
                        let x = std::ffi::CString::new("HeistIsland").unwrap();
                        hook::natives::streaming::_set_island_hopper_enabled(&x, false);
                    }));
                }

                if ui.button(imgui::im_str!("Teleport to Cayo Perico"), [0.0, 0.0]) {
                    crate::thread_jumper::run(Box::new(|_| {
                        let player_ped = hook::natives::player::player_ped_id();

                        hook::natives::entity::set_entity_coords_no_offset(
                            player_ped, 4840.571, -5174.425, 2.0, false, false, false,
                        );
                    }));
                }

                ui.popup_modal(imgui::im_str!("Spawn Vehicle"))
                    .always_auto_resize(true)
                    .build(|| {
                        ui.input_text(imgui::im_str!("Vehicle Model"), &mut data.vehicle_model)
                            .build();

                        ui.input_int(imgui::im_str!("Primary Color"), &mut data.primary_color)
                            .build();

                        ui.input_int(imgui::im_str!("Secondary Color"), &mut data.secondary_color)
                            .build();

                        ui.separator();

                        if ui.button(imgui::im_str!("Done"), [120.0, 0.0]) {
                            let model_name = data.vehicle_model.to_string();
                            let primary_color = data.primary_color;
                            let secondary_color = data.secondary_color;

                            crate::thread_jumper::run(Box::new(move |world| {
                                let player_ped_id = hook::natives::player::player_ped_id();
                                let position =
                                    hook::natives::entity::get_entity_coords(player_ped_id, true);
                                let rotation =
                                    hook::natives::entity::get_entity_rotation(player_ped_id, 2);

                                let (handle, model, transform, vehicle) =
                                    crate::entities::create_vehicle_with_model_name(
                                        &model_name,
                                        position,
                                        rotation,
                                        primary_color,
                                        secondary_color,
                                    );

                                hook::natives::ped::set_ped_into_vehicle(
                                    player_ped_id,
                                    handle.handle,
                                    -1,
                                );

                                world
                                    .spawn()
                                    .insert_bundle((handle, model, transform, vehicle));

                                add_log(&format!(
                                    "Spawned vehicle ({})",
                                    model_name.to_uppercase()
                                ));
                            }));

                            ui.close_current_popup();
                        }
                    });
            });
    }

    if data.show_logs {
        imgui::Window::new(imgui::im_str!("Logs"))
            .size([300.0, 100.0], imgui::Condition::FirstUseEver)
            .build(&ui, || {
                let buffer = LOG_BUFFER.lock().unwrap();
                ui.text(imgui::ImString::new(buffer.join("\n")));
            });
    }
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

pub(super) fn handle_cursor() {
    if hook::is_key_released(hook::keycodes::KEY_F1, true) {
        unsafe {
            SHOW_CURSOR = AtomicBool::new(*SHOW_CURSOR.get_mut() == false);
        }
    }

    if unsafe { *SHOW_CURSOR.get_mut() } {
        hook::natives::pad::disable_all_control_actions(0);
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
