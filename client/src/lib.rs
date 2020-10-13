#![forbid(unsafe_code)]

use log::info;
use once_cell::sync::Lazy;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

mod utility;

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

    hook::initialize(tick);

    info!("Successfully started CryV");
}

fn tick() {
    //
    draw_text("CryV", 0.975, 0.01, 0.42, (200, 200, 200, 255));
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

fn draw_text(text: &str, pos_x: f32, pos_y: f32, scale: f32, color: (i32, i32, i32, i32)) {
    hud::set_text_font(0);
    hud::set_text_scale(scale, scale);
    hud::set_text_colour(color.0, color.1, color.2, color.3);
    hud::set_text_wrap(0.0, 1.0);
    hud::set_text_centre(true);
    hud::set_text_dropshadow(0, 0, 0, 0, 0);
    hud::set_text_edge(1, 0, 0, 0, 205);

    let text_type = std::ffi::CString::new("STRING").unwrap();
    let text = std::ffi::CString::new(text).unwrap();

    hud::begin_text_command_display_text(&text_type);
    hud::add_text_component_substring_player_name(&text);
    hud::end_text_command_display_text(pos_x, pos_y, 1);
}
