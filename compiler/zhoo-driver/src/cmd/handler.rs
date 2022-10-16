mod compile;
mod run;

pub use compile::Compile;
pub use run::Run;

use lazy_static::lazy_static;
use qute::prelude::*;

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
