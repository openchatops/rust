use std::collections::HashMap;

use futures::stream::Stream;

use error::Error;
use robot::Robot;
use room::Room;
use user::User;


/// An adapter provides the implementation for `RobotBrain`'s core methods for a specific chat
/// service.
pub trait ChatAdapter {
    /// Provides an infinite stream of incoming messages to be dispatched to the robot.
    fn run(self) -> Result<Box<Stream<Item = IncomingMessage, Error = Error>>, Error>;

    /// Sends a message.
    fn send_message(&self, message: &OutgoingMessage) -> Result<(), Error>;

    /// Sets the topic of a room.
    ///
    /// Returns `Error::Unimplemented` if not implemented by the adapter.
    fn set_topic(&self, room: &Room, topic: &str) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    /// Makes the robot join a room.
    ///
    /// Returns `Error::Unimplemented` if not implemented by the adapter.
    fn join(&self, room: &Room) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    /// Makes the robot part from a room.
    ///
    /// Returns `Error::Unimplemented` if not implemented by the adapter.
    fn part(&self, room: &Room) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
}

/// A chat callback is invoked with incoming messages that match certain patterns.
pub trait ChatCallback {
    /// Process an incoming message that matched a route the callback was registered for.
    fn call(&self, robot: Robot, incoming_message: IncomingMessage)
    -> Result<Box<Stream<Item = OutgoingMessage, Error = Error>>, Error>;
}

/// An incoming textual chat message.
pub struct IncomingMessage {
    /// The textual body of the message.
    body: String,
    /// The room the message was sent from, if applicable.
    room: Option<Room>,
    /// The user who sent the message.
    user: User,
}

/// An outgoing textual chat message.
pub struct OutgoingMessage {
    /// The textual body of the message.
    body: String,
    /// The room the message will be sent to.
    room: Option<Room>,
    /// The user the message will be sent to.
    user: Option<User>,
}

impl IncomingMessage {
    /// Constructs a new incoming message.
    pub fn new<S>(body: S, user: User, room: Option<Room>) -> Self where S: Into<String> {
        IncomingMessage {
            body: body.into(),
            room: room,
            user: user,
        }
    }

    /// The textual body of the message.
    pub fn body(&self) -> &str {
        &self.body
    }

    /// The room the message was sent from, if applicable.
    pub fn room(&self) -> &Option<Room> {
        &self.room
    }

    /// The user who sent the message.
    pub fn user(&self) -> &User {
        &self.user
    }
}

impl OutgoingMessage {
    /// Constructs a new message to a room.
    pub fn to_room<S>(body: S, room: Room) -> Self where S: Into<String> {
        OutgoingMessage {
            body: body.into(),
            room: Some(room),
            user: None,
        }
    }
    /// Constructs a new direct message to a user.
    pub fn to_user<S>(body: S, user: User) -> Self where S: Into<String> {
        OutgoingMessage {
            body: body.into(),
            room: None,
            user: Some(user),
        }
    }

    /// Constructs a new message to a user in a room.
    pub fn to_user_in_room<S>(body: S, room: Room, user: User) -> Self where S: Into<String> {
        OutgoingMessage {
            body: body.into(),
            room: Some(room),
            user: Some(user),
        }
    }

    /// The textual body of the message.
    pub fn body(&self) -> &str {
        &self.body
    }

    /// The room the message will be sent to, if applicable.
    pub fn room(&self) -> &Option<Room> {
        &self.room
    }

    /// The user the message will be sent to or addressed to, if applicable.
    pub fn user(&self) -> &Option<User> {
        &self.user
    }
}
