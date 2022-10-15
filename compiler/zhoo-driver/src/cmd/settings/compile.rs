use crate::cmd::settings::Backend;

#[derive(Debug)]
pub struct Settings {
  pub ast: bool,
  pub input: String,
  pub no_motion: bool,
  pub ir: bool,
  pub backend: Backend,
}
