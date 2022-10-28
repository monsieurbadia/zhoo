mod checker;
mod context;
mod scope;

pub mod builtins;

pub mod analyzer {
  pub use crate::checker::analyze;
}
