use std::io::Error as IoError;

/// An error returned when operations fail.
#[derive(Debug)]
pub enum Error {
    /// An error not covered by other variants.
    Generic(String),
    /// An IO error occurred.
    Io(IoError),
    /// No data was found in the robot's storage.
    MissingData,
    /// An error for operations not implemented by the particular chat service.
    Unimplemented,
}
