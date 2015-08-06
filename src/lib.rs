#![cfg_attr(feature = "timeouts", feature(duration))]

extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub mod client;
pub mod error;
pub mod emoticon;
pub mod room;
pub mod util;

pub use client::Client;
