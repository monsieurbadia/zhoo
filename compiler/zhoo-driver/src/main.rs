use clap::Parser;
use zhoo_driver::Cmd;

// the entry point of the `zhoo` compiler
fn main() {
  Cmd::parse().run();
}
