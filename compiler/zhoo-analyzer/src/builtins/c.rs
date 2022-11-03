use super::{Builtin, Proto};

use zhoo_ast::ast::Ty;

pub fn c_builtins() -> Vec<Builtin> {
  vec![
    Builtin::new(
      String::from("malloc"),
      Proto(vec![Ty::INT.into()], Ty::INT.into()),
    ),
    Builtin::new(
      String::from("free"),
      Proto(vec![Ty::INT.into()], Ty::VOID.into()),
    ),
  ]
}
