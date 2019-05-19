#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(rust_2018_idioms)]

//! Unified REST APIs for world-wide astronomy data.

pub mod client;

/// Activities are the records the observing activities around the world.
/// They intend to gather in a single object an observing activity in a given
/// observing site, with a given telescope, a given instrument by a given
/// observer, or collaboration or organisation.
pub mod endpoint;
