//! this module contains modules for the `zhoo-driver`

mod cmd;
mod common;

pub use cmd::Cmd;

use clap::Parser;

pub fn main() {
  Cmd::parse().run();
}
