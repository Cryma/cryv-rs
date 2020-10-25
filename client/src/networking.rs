use bevy::prelude::*;
use bevy_prototype_networking_laminar::{Connection, NetworkEvent};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<NetworkMessageEvent>()
            .init_resource::<NetworkEventReader>()
            .init_resource::<NetworkMessageEventReader>()
            .add_system(receive_messages.system());
    }
}

#[derive(Default)]
pub struct NetworkMessageEventReader {
    pub network_messages: EventReader<NetworkMessageEvent>,
}

#[derive(Default)]
struct NetworkEventReader {
    network_events: EventReader<NetworkEvent>,
}

#[derive(Debug)]
pub struct NetworkMessageEvent {
    pub connection: Connection,
    pub message: shared::NetworkMessage,
}

fn receive_messages(
    mut state: ResMut<NetworkEventReader>,
    network_events: Res<Events<NetworkEvent>>,
    mut network_messages: ResMut<Events<NetworkMessageEvent>>,
) {
    for event in state.network_events.iter(&network_events) {
        match event {
            NetworkEvent::Message(connection, data) => {
                let message = shared::NetworkMessage::decode(data);

                network_messages.send(NetworkMessageEvent {
                    connection: *connection,
                    message,
                });
            }
            NetworkEvent::Connected(connection) => log::info!("Connected: {}", connection),
            NetworkEvent::Disconnected(connection) => log::info!("Disconnected: {}", connection),
            NetworkEvent::SendError(error) => log::error!("Error: {}", error),
        }
    }
}
