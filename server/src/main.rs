use bevy::{
    app::ScheduleRunnerPlugin, core::CorePlugin, prelude::*, type_registry::TypeRegistryPlugin,
};
use bevy_prototype_networking_laminar::{
    Connection, NetworkDelivery, NetworkResource, NetworkingPlugin,
};
use shared::EntityTransform;
use std::time::Duration;

struct NetworkIdentifier {
    pub connection: Connection,
}

fn main() {
    create_logger().expect("Something went wrong while creating the logger!");

    App::build()
        .add_plugin(TypeRegistryPlugin::default())
        .add_plugin(CorePlugin)
        .add_plugin(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugin(NetworkingPlugin)
        .add_plugin(shared::NetworkingPlugin)
        .add_startup_system(start_server.system())
        .add_system(entity_transform_update_handler.system())
        .add_system(connection_established_handler.system())
        .run();
}

fn connection_established_handler(
    mut commands: Commands,
    net: Res<NetworkResource>,
    mut state: ResMut<shared::NetworkMessageEventReader>,
    events: Res<Events<shared::NetworkMessageEvent>>,
) {
    for event in state.network_messages.iter(&events) {
        if let shared::NetworkMessage::EstablishConnection(model, transform) = &event.message {
            let connection_established_message = shared::NetworkMessage::ConnectionEstablished;

            if let Err(error) = net.send(
                event.connection.addr,
                &connection_established_message.encode()[..],
                NetworkDelivery::ReliableOrdered(Some(1)),
            ) {
                log::error!(
                    "Something went wrong while trying to send connection established message: {}",
                    error
                );
            }

            commands.spawn((
                NetworkIdentifier {
                    connection: event.connection,
                },
                *model,
                *transform,
            ));

            log::info!(
                "Received introduction payload from {} with content model {} and transform {:?}",
                event.connection,
                model,
                transform
            );
        }
    }
}

fn entity_transform_update_handler(
    mut state: ResMut<shared::NetworkMessageEventReader>,
    events: Res<Events<shared::NetworkMessageEvent>>,
    mut query: Query<(&NetworkIdentifier, &mut EntityTransform)>,
) {
    for event in state.network_messages.iter(&events) {
        if let shared::NetworkMessage::UpdateEntityTransform(transform) = &event.message {
            for (network_identifier, mut entity_transform) in &mut query.iter() {
                if network_identifier.connection.addr != event.connection.addr {
                    continue;
                }

                entity_transform.position = transform.position;
                entity_transform.rotation = transform.rotation;
                entity_transform.velocity = transform.velocity;

                log::info!(
                    "Received entity transform update from {} with transform {:?}",
                    event.connection,
                    transform
                );
            }
        }
    }
}

fn start_server(mut network_resource: ResMut<NetworkResource>) {
    network_resource.bind("127.0.0.1:1337").unwrap();
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
