// TODO: Move imgui related code to separate crate and disallow unsafe code
// #![forbid(unsafe_code)]

use hook::winapi::{
    shared::minwindef::LPARAM, shared::minwindef::UINT, shared::minwindef::WPARAM,
    shared::windef::HWND, um::winuser::FindWindowA,
};
use log::info;
use once_cell::sync::Lazy;
use shared::bevy::{app::ScheduleRunnerPlugin, prelude::*};
use shared::bevy_prototype_networking_laminar::{Connection, NetworkDelivery, NetworkResource};
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

mod cleanup;
mod console;
mod entities;
mod thread_jumper;
mod ui;
mod utility;
mod wrapped_natives;

#[macro_export]
macro_rules! clone_query {
    ($query:expr) => {{
        let mut items = Vec::new();

        for item in &mut $query.iter() {
            items.push(item.clone());
        }

        items
    }};
}

static INSTALL_DIRECTORY: Lazy<String> = Lazy::new(|| {
    let key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(r"Software\CryV")
        .unwrap();

    let value: String = key.get_value("InstallDirectory").unwrap();

    value
});

#[derive(Default)]
struct NetworkInfo {
    connection: Option<Connection>,
    is_connected: bool,
}

make_entrypoint!(entrypoint);

fn entrypoint() {
    create_logger().expect("Something went wrong while creating the logger!");

    info!("--------------------------");
    info!("Starting CryV");

    hook::initialize(script_callback);

    hook::register_present_callback(d3d11_present);
    hook::register_window_proc_callback(wndproc);

    info!("Successfully started CryV");
}

static mut HAS_INIT: bool = false;
fn d3d11_present(
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

            HAS_INIT = true;
        }

        imgui_sys::ImGui_ImplDX11_NewFrame();
        imgui_sys::ImGui_ImplWin32_NewFrame();
        imgui_sys::igNewFrame();

        let window_title = std::ffi::CString::new("funkcheck").unwrap();
        if imgui_sys::igBegin(window_title.as_ptr(), std::ptr::null_mut(), 0) {
            if imgui_sys::igButton(window_title.as_ptr(), imgui_sys::ImVec2::new(60.0, 20.0)) {}
        }
        imgui_sys::igEnd();

        imgui_sys::igRender();
        imgui_sys::ImGui_ImplDX11_RenderDrawData(
            imgui_sys::igGetDrawData() as *mut std::ffi::c_void
        );
    };
}

fn wndproc(hwnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) {
    unsafe {
        imgui_sys::ImGui_ImplWin32_WndProcHandler(
            hwnd as *mut std::ffi::c_void,
            message,
            w_param as *mut u32,
            l_param as i64,
        );
    };
}

fn update_keyboard(_world: &mut World, _resources: &mut Resources) {
    hook::update_keyboard();
}

fn script_wait(_world: &mut World, _resources: &mut Resources) {
    hook::script_wait(0);
}

fn connection_established_handler(
    mut console_data: ResMut<console::ConsoleData>,
    mut network_info: ResMut<NetworkInfo>,
    mut state: Local<shared::NetworkMessageEventReader>,
    events: Res<Events<shared::NetworkMessageEvent>>,
) {
    for event in state.network_messages.iter(&events) {
        log::debug!(
            "Received network message from {}: {:?}",
            event.connection,
            event.message
        );

        if let shared::NetworkMessage::ConnectionEstablished = &event.message {
            log::info!(
                "Successfully established connection to: {}",
                event.connection
            );
            console_data.print_line(&format!(
                "Successfully established connection to: {}",
                event.connection
            ));

            network_info.connection = Some(event.connection);
            network_info.is_connected = true;
        }
    }
}

fn update_local_player(_world: &mut World, resources: &mut Resources) {
    let network_info = resources.get::<NetworkInfo>().unwrap();

    if network_info.is_connected == false || network_info.connection.is_none() {
        return;
    }

    let network_resource = resources.get::<NetworkResource>().unwrap();

    let transform = entities::get_entity_transform(hook::natives::player::player_ped_id());

    if let Err(error) = network_resource.send(
        network_info.connection.unwrap().addr,
        &shared::NetworkMessage::UpdateEntityTransform(transform).encode()[..],
        NetworkDelivery::ReliableOrdered(Some(1)),
    ) {
        log::error!("Error while trying to update entity transform: {}", error);
    }
}

fn script_callback() {
    App::build()
        .init_resource::<NetworkInfo>()
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_system(update_keyboard.thread_local_system())
        .add_plugin(shared::bevy_prototype_networking_laminar::NetworkingPlugin)
        .add_plugin(shared::NetworkingPlugin)
        .add_plugin(cleanup::CleanupPlugin)
        .add_plugin(console::ConsolePlugin)
        .add_plugin(ui::UiPlugin)
        .add_system(script_wait.thread_local_system())
        .add_system(connection_established_handler.system())
        .add_system(update_local_player.thread_local_system())
        .add_plugin(thread_jumper::ThreadJumperPlugin)
        .run();
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
