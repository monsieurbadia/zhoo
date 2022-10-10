pub mod io;
pub mod sys;

use crate::front::parser::tree::ty::Ty;

pub struct Builtin {
  pub name: String,
  pub proto: Proto,
}

impl Builtin {
  pub const fn new(name: String, proto: Proto) -> Self {
    Self { name, proto }
  }
}

pub struct Proto(pub Vec<Ty>, pub Ty);
