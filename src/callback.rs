/// Details on how to reach a callback over the network.
pub struct CallbackAddress;

/// A route from an incoming message to a chat callback.
pub struct ChatRoute {
    address: CallbackAddress,
    pattern: String,
}

/// A route from an event name to an event callback.
pub struct EventRoute {
    address: CallbackAddress,
    event: String,
}
