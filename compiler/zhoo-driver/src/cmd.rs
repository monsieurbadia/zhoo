mod handler;
mod settings;

use clap::{Parser, Subcommand};
use pollster::block_on;

/// an instance of a cmd
#[derive(Parser)]
#[clap(version)]
pub struct Cmd {
  #[clap(subcommand)]
  command: Command,
}

/// a command enumeration
#[derive(Subcommand)]
pub enum Command {
  Compile(handler::Compile),
  Run(handler::Run),
}

impl Cmd {
  /// run the commands
  pub fn run(&self) {
    block_on(self.cmd());
  }

  /// handle a command
  async fn cmd(&self) {
    match self.command {
      Command::Compile(ref command) => command.handle().await,
      Command::Run(ref command) => command.handle().await,
    }
  }
}
