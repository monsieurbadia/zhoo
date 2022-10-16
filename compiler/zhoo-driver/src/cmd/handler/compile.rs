use crate::cmd::handler::{
  COMPILATION_DONE, COMPILATION_IN_PROCESS, COMPILATION_IN_TIME,
  COMPILATION_START,
};

use crate::cmd::settings::compile::Settings;

use pollster::block_on;

use std::any::Any;
use std::thread;

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
    use crate::cmd::settings::Backend;
    use crate::common::{EXIT_FAILURE, EXIT_SUCCESS};

    use std::process;

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
  thread::spawn(move || block_on(compiling(settings))).join()
}

async fn compiling(settings: Settings) {
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
  //
  // `program-name`: the name of the program from a configuration file
  // `version`: the version of the program from a configuration file
  // `mode`: [dev|release]
  // `backend`: [cranelift|llvm]
  // `time`: the compilation time in seconds

  const INTERVAL: u64 = 500;

  let spinner = spin::loading(spin::Spinner::Arc);

  spinner.with_text(&*COMPILATION_IN_PROCESS);

  // used as a margin top
  println!();

  spinner
    .with_info(format!("{} `project-name` `version`", &*COMPILATION_START)); // todo #2

  thread::sleep(Duration::from_millis(INTERVAL)); // todo #1

  // -- front --

  let program = parser::parse(settings.input);
  let _ = analyzer::analyze(&program);

  // -- back --

  let codegen = codegen::cranelift::aot::generate(&program);

  match codegen.build(settings.ir).await {
    Ok(done) => {
      spinner
        .with_info(format!("     {} `mode` | `backend`", &*COMPILATION_DONE)); // todo #2

      thread::sleep(Duration::from_millis(INTERVAL)); // todo #1

      spinner
        .with_time(format!("      {} `time` seconds", &*COMPILATION_IN_TIME)); // todo #2

      thread::sleep(Duration::from_millis(INTERVAL)); // todo #1
      spinner.stop();
      done();
      println!("🤖 compile `program-name` successfully\n"); // todo #2

      if settings.ast {
        println!("{}", program);
      }

      // use as a margin bottom
      println!();
    }
    Err(error) => {
      spinner.stop();
      eprint!("{error}\n");
      eprintln!("🤖 i couldn't compile `project-name`\n"); // todo #2
    }
  }
}
