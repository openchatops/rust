//! Crate **oco** is an [OpenChatOps](https://github.com/openchatops/spec) library for Rust.

extern crate futures;
extern crate protobuf;
extern crate regex;
extern crate router;

/// Callbacks are programs run in response to incoming messages, events, or HTTP requests.
pub mod callback;
/// Types related to textual chat.
pub mod chat;
/// User-supplied configuration.
pub mod config;
/// Error types.
pub mod error;
/// Pub-sub events.
pub mod event;
/// The primary chat robot.
pub mod robot;
/// Chat rooms.
pub mod room;
/// Persistent data storage.
pub mod storage;
/// Users capable of sending and receiving messages.
pub mod user;
