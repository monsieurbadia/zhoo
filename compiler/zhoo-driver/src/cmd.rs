mod handle;
mod settings;

use clap::{Parser, Subcommand};
use pollster::block_on;

#[derive(Parser)]
#[clap(version)]
pub struct Cmd {
  #[clap(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
pub enum Command {
  Compile(handle::Compile),
  Run(handle::Run),
}

impl Cmd {
  pub fn run(&self) {
    block_on(self.cmd());
  }

  async fn cmd(&self) {
    match self.command {
      Command::Compile(ref command) => command.handle().await,
      Command::Run(ref command) => command.handle().await,
    }
  }
}
