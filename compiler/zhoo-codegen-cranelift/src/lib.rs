mod codegen;
mod interface;
mod translator;

pub mod cranelift {
  pub use super::codegen::generate;
}
