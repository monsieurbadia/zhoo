use crate::cmd::settings::Backend;

#[derive(Debug)]
pub struct Settings {
  pub ast: bool,
  pub input: String,
  pub ir: bool,
  pub backend: Backend,
}
