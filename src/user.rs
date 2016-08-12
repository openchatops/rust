use error::Error;

/// Users are people capable of sending and receiving messages.
#[derive(Clone)]
pub struct User {
    id: String,
    name: String,
    mention_name: Option<String>,
}

impl User {
    /// Creates a new user.
    pub fn new(id: String, name: String, mention_name: Option<String>) -> Self {
        User {
            id: id,
            name: name,
            mention_name: mention_name,
        }
    }

    /// The user's unique identifier.
    ///
    /// Not intended to display to humans.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The user's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// A special form of the user's name used for "mentioning" them in chat.
    ///
    /// Defaults to the value of `self.name()` if not specified.
    pub fn mention_name(&self) -> &str {
        match self.mention_name {
            Some(ref mention_name) => mention_name,
            None => self.name(),
        }
    }
}
