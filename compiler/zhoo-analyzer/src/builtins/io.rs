use super::{Builtin, Proto};

use zhoo_ast::ast::Ty;

pub fn io_builtins() -> Vec<Builtin> {
  vec![
    Builtin::new(
      String::from("print"),
      Proto(vec![Ty::STR.into()], Ty::VOID.into()),
    ),
    Builtin::new(
      String::from("println"),
      Proto(vec![Ty::STR.into()], Ty::VOID.into()),
    ),
    Builtin::new(
      String::from("printi"),
      Proto(vec![Ty::INT.into()], Ty::VOID.into()),
    ),
    Builtin::new(
      String::from("printiln"),
      Proto(vec![Ty::INT.into()], Ty::VOID.into()),
    ),
    Builtin::new(
      String::from("printr"),
      Proto(vec![Ty::REAL.into()], Ty::VOID.into()),
    ),
    Builtin::new(
      String::from("printrln"),
      Proto(vec![Ty::REAL.into()], Ty::VOID.into()),
    ),
  ]
}
