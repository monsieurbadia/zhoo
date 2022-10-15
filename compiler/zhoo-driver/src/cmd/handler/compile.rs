use crate::cmd::settings::compile::Settings;
use crate::cmd::settings::Backend;

use lazy_static::lazy_static;
use qute::prelude::*;

use std::any::Any;
use std::{process, thread};

lazy_static! {
  pub static ref COMPILATION_IN_PROCESS: String =
    qute!("building").bold().set_color(171).to_string();
  pub static ref COMPILATION_START: String = qute!("compiling")
    .bold()
    .underline()
    .light_green()
    .to_string();
  pub static ref COMPILATION_DONE: String =
    qute!("done").bold().underline().light_green().to_string();
  pub static ref COMPILATION_IN_TIME: String =
    qute!("in").bold().underline().light_green().to_string();
}

#[derive(clap::Parser)]
pub struct Compile {
  /// print the AST of the program
  #[clap(short, long)]
  ast: bool,
  /// specify the path name of the program
  #[clap(short, long)]
  input: String,
  /// print the ir of the program
  #[clap(long)]
  ir: bool,
  /// disable output animations (unimplemented)
  #[clap(long)]
  no_motion: bool,
  /// specify the backend you want to use
  #[clap(short, long, default_value = "cranelift")]
  backend: String,
}

impl Compile {
  pub async fn handle(&self) {
    use crate::common::{EXIT_FAILURE, EXIT_SUCCESS};

    let settings = Settings {
      ast: self.ast,
      no_motion: self.no_motion,
      input: self.input.clone(),
      ir: self.ir,
      backend: Backend::from(self.backend.clone()),
    };

    match compile(settings).await {
      Ok(_) => process::exit(EXIT_SUCCESS),
      Err(_) => process::exit(EXIT_FAILURE),
    }
  }
}

async fn compile(
  settings: Settings,
) -> Result<(), Box<(dyn Any + Send + 'static)>> {
  thread::spawn(move || compiling(settings)).join()
}

fn compiling(settings: Settings) {
  use zhoo::back::codegen;
  use zhoo::front::{analyzer, parser};

  use loaders::spin;

  use std::time::Duration;

  // -- todo #1 --
  //
  // all these sleeps calls are temporary. i use them to design the
  // compiler output.
  //
  // nb: don't forget to delete them later
  //
  // -- todo #2 --
  //
  // values between backticks should be dynamic

  const INTERVAL: u64 = 500;

  let spinner = spin::loading(spin::Spinner::Arc);

  spinner.with_text(&*COMPILATION_IN_PROCESS);

  println!();
  spinner
    .with_info(format!("{} `project-name` `version`", &*COMPILATION_START)); // todo #2

  thread::sleep(Duration::from_millis(INTERVAL)); // todo #1

  let program = parser::parse(settings.input);
  let _ = analyzer::analyze(&program);
  let codegen = codegen::cranelift::aot::generate(&program);

  match codegen.build(settings.ir) {
    Ok(done) => {
      spinner
        .with_info(format!("     {} `mode` | `backend`", &*COMPILATION_DONE)); // todo #2

      thread::sleep(Duration::from_millis(INTERVAL)); // todo #1

      spinner
        .with_time(format!("      {} `time` seconds", &*COMPILATION_IN_TIME)); // todo #2

      thread::sleep(Duration::from_millis(INTERVAL)); // todo #1
      spinner.stop();
      done();
      println!("\n✨ compile `program-name` successfully\n"); // todo #2

      if settings.ast {
        println!("{}", program);
      }

      // use as a bottom margin
      println!();
    }
    Err(error) => {
      spinner.stop();
      eprint!("{error}");
      eprintln!("💥 i couldn't compile `project-name`\n");
    }
  }
}
