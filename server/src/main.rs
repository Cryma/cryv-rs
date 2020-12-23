use shared::bevy::{app::ScheduleRunnerPlugin, core::CorePlugin, prelude::*};

fn main() {
    create_logger().expect("Something went wrong while creating the logger!");

    App::build()
        .add_plugin(CorePlugin)
        .add_plugin(ScheduleRunnerPlugin::default())
        .run();
}

fn create_logger() -> Result<(), fern::InitError> {
    let log_file_path = format!(
        "{}/cryv_server.log",
        std::env::current_dir().unwrap().to_str().unwrap()
    );

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
