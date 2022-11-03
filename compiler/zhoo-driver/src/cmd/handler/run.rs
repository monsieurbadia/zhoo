use std::any::Any;
use std::thread;

#[derive(clap::Parser)]
pub struct Run;

impl Run {
  pub async fn handle(&self) {
    use crate::common::{EXIT_FAILURE, EXIT_SUCCESS};

    use std::process;

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
  use zhoo_helper::constant::{ENTRY_POINT, PATH_OUTPUT_DIRECTORY};

  use std::process::Command;
  use std::str;

  println!("🤖 running the program");

  let program = format!("./{PATH_OUTPUT_DIRECTORY}/{ENTRY_POINT}");
  let output = Command::new(program).output().unwrap();
  let output = str::from_utf8(&output.stdout).unwrap();

  if !output.is_empty() {
    println!();
    println!("{}", output);
  }
}
