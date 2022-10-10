use crate::back::codegen::builtins::builtin::{Builtin, Proto};
use crate::front::parser::tree::ty::Ty;

pub fn io_builtins() -> Vec<Builtin> {
  vec![
    Builtin::new(String::from("print"), Proto(vec![Ty::STR], Ty::VOID)),
    Builtin::new(String::from("println"), Proto(vec![Ty::STR], Ty::VOID)),
    Builtin::new(String::from("print_int"), Proto(vec![Ty::INT], Ty::VOID)),
    Builtin::new(String::from("println_int"), Proto(vec![Ty::INT], Ty::VOID)),
  ]
}
