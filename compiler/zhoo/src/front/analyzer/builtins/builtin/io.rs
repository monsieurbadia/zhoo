use super::{Builtin, Proto};

use crate::front::parser::tree::ty::Ty;

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
      String::from("print_int"),
      Proto(vec![Ty::INT.into()], Ty::VOID.into()),
    ),
    Builtin::new(
      String::from("println_int"),
      Proto(vec![Ty::INT.into()], Ty::VOID.into()),
    ),
  ]
}
