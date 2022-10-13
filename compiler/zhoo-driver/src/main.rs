use clap::Parser;
use zhoo_driver::Cmd;

fn main() {
  Cmd::parse().run();
}
