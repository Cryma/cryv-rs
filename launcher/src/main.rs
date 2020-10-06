extern crate winapi;

use std::ffi::CString;
use sysinfo::{ProcessExt, SystemExt};
use winapi::shared::minwindef::DWORD;
use winapi::shared::winerror::ERROR_INVALID_HANDLE;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};
use winapi::um::winuser::{FindWindowA, GetWindowThreadProcessId, IsWindowVisible};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

fn main() {
    let path = match get_gta_path() {
        Some(path) => path,
        None => {
            println!("Could not find GTA5 path!");

            return;
        }
    };

    println!("Grand Theft Auto V Path: {}", path);

    let dll_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("client.dll")
        .to_str()
        .unwrap()
        .to_string();

    let dll_directory = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    println!("Current dll path: {}", &dll_path);

    write_registry_value(
        RegKey::predef(HKEY_LOCAL_MACHINE),
        r"Software\CryV".to_owned(),
        "InstallDirectory".to_owned(),
        dll_directory,
    )
    .unwrap();

    if let Err(error) = start_process(path, "".to_owned()) {
        println!("Error while trying to start GTA5 process: {}", error);

        return;
    }

    let mut process_id = 0;
    loop {
        let system = sysinfo::System::new_all();
        for (pid, process) in system.get_processes() {
            if process.name() == "GTA5.exe" {
                process_id = pid.clone();

                break;
            }
        }

        if process_id != 0 {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    loop {
        let window_name = CString::new("Grand Theft Auto V").unwrap();
        let window_name_pointer = window_name.as_ptr();

        let handle = unsafe { FindWindowA(std::ptr::null_mut(), window_name_pointer) };

        let mut foreground_window_id: DWORD = 0;
        unsafe {
            GetWindowThreadProcessId(handle, &mut foreground_window_id as *mut DWORD);
        };

        let is_window_visible =
            unsafe { IsWindowVisible(handle) == winapi::shared::minwindef::TRUE };

        if foreground_window_id as usize == process_id && is_window_visible {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!("Injecting client");

    let process = unsafe {
        OpenProcess(
            winapi::um::winnt::PROCESS_ALL_ACCESS,
            winapi::shared::minwindef::FALSE,
            process_id as u32,
        )
    };

    inject_dll(process, dll_path);

    println!("Done");
}

fn inject_dll(process: *mut winapi::ctypes::c_void, dll_path: String) {
    let dll_path_string = CString::new(dll_path.clone()).unwrap();
    let dll_path_size = dll_path.len() + 1;

    let memory_allocation = unsafe {
        VirtualAllocEx(
            process,
            std::ptr::null_mut(),
            dll_path_size,
            winapi::um::winnt::MEM_COMMIT,
            winapi::um::winnt::PAGE_EXECUTE_READWRITE,
        )
    };

    let mut bytes_written: usize = 0;
    unsafe {
        if WriteProcessMemory(
            process,
            memory_allocation,
            dll_path_string.as_ptr() as *mut winapi::ctypes::c_void,
            dll_path_size,
            &mut bytes_written as *mut usize,
        ) == ERROR_INVALID_HANDLE as i32
        {
            println!("Failed to write memory into target process!");

            return;
        }
    };

    let kernel_string = CString::new("kernel32").unwrap();
    let load_library_string = CString::new("LoadLibraryA").unwrap();

    let mut return_value: DWORD = 0;
    let load_library_address = unsafe {
        GetProcAddress(
            LoadLibraryA(kernel_string.as_ptr()),
            load_library_string.as_ptr(),
        )
    };

    unsafe {
        CreateRemoteThread(
            process,
            std::ptr::null_mut(),
            0,
            Some(std::mem::transmute(load_library_address)),
            memory_allocation,
            0,
            &mut return_value,
        )
    };
}

fn start_process(path: String, arguments: String) -> std::io::Result<()> {
    std::process::Command::new(path).arg(arguments).spawn()?;

    Ok(())
}

fn get_gta_path() -> Option<String> {
    // Steam Launcher
    if let Ok(value) = read_registry_value(
        RegKey::predef(HKEY_LOCAL_MACHINE),
        r"SOFTWARE\WOW6432Node\Rockstar Games\GTAV".to_owned(),
        "InstallFolderSteam".to_owned(),
    ) {
        return Some(value);
    }

    // Former Social Club Launcher
    if let Ok(value) = read_registry_value(
        RegKey::predef(HKEY_LOCAL_MACHINE),
        r"SOFTWARE\WOW6432Node\Rockstar Games\Grand Theft Auto V".to_owned(),
        "InstallFolder".to_owned(),
    ) {
        return Some(value);
    }

    // Hacky workaround for Rockstar Games Launcher
    if let Ok(value) = read_registry_value(
        RegKey::predef(HKEY_LOCAL_MACHINE),
        r"SOFTWARE\WOW6432Node\Rockstar Games\Launcher".to_owned(),
        "InstallFolder".to_owned(),
    ) {
        let path = std::path::Path::new(&value);

        return Some(
            path.parent()
                .unwrap()
                .join(r"Grand Theft Auto V\GTAVLauncher.exe")
                .into_os_string()
                .into_string()
                .unwrap(),
        );
    }

    // TODO: Support Epic Games Launcher
    // TODO: If Steam, start directly through steam instead of launching GTAVLauncher.exe

    None
}

fn read_registry_value(registry_key: RegKey, key: String, name: String) -> std::io::Result<String> {
    let sub_key = registry_key.open_subkey(key)?;
    let value: String = sub_key.get_value(name)?;

    Ok(value)
}

fn write_registry_value(
    registry_key: RegKey,
    key: String,
    name: String,
    value: String,
) -> std::io::Result<()> {
    let (sub_key, _) = registry_key.create_subkey(key)?;

    sub_key.set_value(name, &value)?;

    Ok(())
}
