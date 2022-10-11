use crate::common::{EXIT_FAILURE, EXIT_SUCCESS};

use std::any::Any;
use std::{process, thread};

#[derive(clap::Parser)]
pub struct Run;

impl Run {
  pub async fn handle(&self) {
    match run().await {
      Ok(_) => process::exit(EXIT_SUCCESS),
      Err(_) => process::exit(EXIT_FAILURE),
    }
  }
}

async fn run() -> Result<(), Box<(dyn Any + Send + 'static)>> {
  thread::spawn(running).join()
}

fn running() {
  use std::process::Command;

  println!("running the program");

  let output = Command::new("./program/main").output().unwrap();
  let output = std::str::from_utf8(&output.stdout).unwrap();

  if !output.is_empty() {
    println!("\n{}", output);
  }
}
