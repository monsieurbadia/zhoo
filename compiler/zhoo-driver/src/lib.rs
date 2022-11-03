mod cmd;
mod common;

pub use cmd::Cmd;

use clap::Parser;

pub fn main() {
  Cmd::parse().run();
}
