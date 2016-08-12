use std::collections::HashMap;

use error::Error;

/// Provides an implementation of persistent data storage for the robot.
pub trait StorageAdapter {
    /// Gets a value from the data store by key.
    fn get(&self, key: &str) -> Result<&str, Error>;

    /// Sets a value in the data store under a key.
    fn set<K, V>(&mut self, key: K, value: V) -> Result<(), Error>
    where K: Into<String>, V: Into<String>;
}

/// A storage adapter that uses the process's memory.
pub struct MemoryStorage {
    data: HashMap<String, String>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            data: HashMap::new(),
        }
    }
}

impl StorageAdapter for MemoryStorage {
    fn get(&self, key: &str) -> Result<&str, Error> {
        match self.data.get(key) {
            Some(value) => Ok(value),
            None => Err(Error::MissingData),
        }
    }

    fn set<K, V>(&mut self, key: K, value: V) -> Result<(), Error>
    where K: Into<String>, V: Into<String> {
        self.data.insert(key.into(), value.into());

        Ok(())
    }
}
