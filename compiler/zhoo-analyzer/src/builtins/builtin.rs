pub mod io;
pub mod sys;

use zhoo_parser::tree::ty::Ty;
use zhoo_parser::tree::PBox;

pub struct Builtin {
  pub name: String,
  pub proto: Proto,
}

impl Builtin {
  pub const fn new(name: String, proto: Proto) -> Self {
    Self { name, proto }
  }
}

pub struct Proto(pub Vec<PBox<Ty>>, pub PBox<Ty>);
