use crate::cmd::settings::compile::Settings;
use crate::cmd::settings::Backend;

use std::any::Any;
use std::{process, thread};

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
  /// specify the backend you want to use
  #[clap(short, long, default_value = "cranelift")]
  backend: String,
}

impl Compile {
  pub async fn handle(&self) {
    use crate::common::{EXIT_FAILURE, EXIT_SUCCESS};

    let settings = Settings {
      ast: self.ast,
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

  println!("compiling the program");

  let program = parser::parse(settings.input);
  let _ = analyzer::analyze(&program);
  let codegen = codegen::cranelift::aot::generate(&program);

  match codegen.build(settings.ir) {
    Ok(done) => {
      done();
      // todo: from a config file <project-name> should be dynamic
      println!("\n✨ compile `<program-name>` successfully\n");

      if settings.ast {
        println!("{}", program.to_string());
      }

      // use as a bottom margin
      println!();
    }
    Err(error) => {
      eprint!("{error}");
      eprintln!("💥 i couldn't compile `project-name`\n");
    }
  }
}
