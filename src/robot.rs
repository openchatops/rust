use std::collections::HashMap;

use router::Router;

use callback::{ChatRoute, EventRoute};
use chat::{ChatAdapter, ChatCallback, OutgoingMessage};
use config::Configuration;
use error::Error;
use event::EventCallback;
use room::Room;
use storage::StorageAdapter;

/// The interface to the robot provided to callbacks.
pub struct Robot;

/// The primary chat robot runner.
pub struct RobotBrain<C, S> where C: ChatAdapter, S: StorageAdapter {
    chat_adapter: C,
    chat_routes: Vec<ChatRoute>,
    config: Configuration,
    event_routes: HashMap<String, Vec<EventRoute>>,
    http_routes: Router,
    storage_adapter: S,
}

impl<C, S> RobotBrain<C, S> where C: ChatAdapter, S: StorageAdapter {
    /// Sends a message.
    pub fn send_message(&self, message: &OutgoingMessage) -> Result<(), Error> {
        self.chat_adapter.send_message(message)
    }

    /// Sets the topic of a room.
    pub fn set_topic(&self, room: &Room, topic: &str) -> Result<(), Error> {
        self.chat_adapter.set_topic(room, topic)
    }

    /// Makes the robot join a room.
    pub fn join(&self, room: &Room) -> Result<(), Error> {
        self.chat_adapter.join(room)
    }

    /// Makes the robot part from a room.
    pub fn part(&self, room: &Room) -> Result<(), Error> {
        self.chat_adapter.part(room)
    }
}
