use bevy::{
    app::ScheduleRunnerPlugin, core::CorePlugin, prelude::*, type_registry::TypeRegistryPlugin,
};
use bevy_prototype_networking_laminar::{
    NetworkDelivery, NetworkEvent, NetworkResource, NetworkingPlugin,
};
use std::time::Duration;

fn main() {
    create_logger().expect("Something went wrong while creating the logger!");

    App::build()
        .add_plugin(TypeRegistryPlugin::default())
        .add_plugin(CorePlugin)
        .add_plugin(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .init_resource::<NetworkEventReader>()
        .add_plugin(NetworkingPlugin)
        .add_startup_system(start_server.system())
        .add_system(receive_messages.system())
        .run();
}

fn start_server(mut network_resource: ResMut<NetworkResource>) {
    network_resource.bind("127.0.0.1:1337").unwrap();
}

#[derive(Default)]
struct NetworkEventReader {
    network_events: EventReader<NetworkEvent>,
}

fn receive_messages(
    net: Res<NetworkResource>,
    mut state: ResMut<NetworkEventReader>,
    events: Res<Events<NetworkEvent>>,
) {
    for event in state.network_events.iter(&events) {
        match event {
            NetworkEvent::Message(connection, data) => {
                let network_message = shared::NetworkMessage::decode(data);

                match network_message {
                    shared::NetworkMessage::EstablishConnection(payload) => {
                        let connection_established_message =
                            shared::NetworkMessage::ConnectionEstablished;

                        if let Err(error) = net.send(
                            connection.addr,
                            &connection_established_message.encode()[..],
                            NetworkDelivery::ReliableOrdered(Some(1)),
                        ) {
                            log::error!("Something went wrong while trying to send connection established message: {}", error);
                        }

                        log::info!(
                            "Received introduction payload from {} with content: {}",
                            connection,
                            payload
                        );
                    }
                    _ => {}
                }
            }
            NetworkEvent::Connected(connection) => log::info!("Connected: {}", connection),
            NetworkEvent::Disconnected(connection) => log::info!("Disconnected: {}", connection),
            NetworkEvent::SendError(error) => log::error!("Error: {}", error),
        }
    }
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
