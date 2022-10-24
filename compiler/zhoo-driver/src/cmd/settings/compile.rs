use crate::cmd::settings::Backend;

/// an instance of a settings for the compile command
#[derive(Debug)]
pub(crate) struct Settings {
  pub ast: bool,
  pub input: String,
  pub _no_motion: bool,
  pub ir: bool,
  pub _backend: Backend,
}
