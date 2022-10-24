mod compile;
mod run;

pub use compile::Compile;
pub use run::Run;

use lazy_static::lazy_static;
use qute::prelude::*;

lazy_static! {
  /// the compilation name when it's processing
  pub(crate) static ref COMPILATION_IN_PROCESS: String =
    qute!("building").bold().set_color(171).to_string();

  /// the compilation name when it's starting
  pub(crate) static ref COMPILATION_START: String = qute!("compiling")
    .bold()
    .underline()
    .light_green()
    .to_string();

  /// the compilation name when it's done
  pub(crate) static ref COMPILATION_DONE: String =
    qute!("done").bold().underline().light_green().to_string();

  /// the compilation name when it's displaying the compilation duration
  pub(crate) static ref COMPILATION_IN_TIME: String =
    qute!("in").bold().underline().light_green().to_string();
}
