use super::{Builtin, Proto};

use crate::front::parser::tree::ty::Ty;

pub fn sys_builtins() -> Vec<Builtin> {
  vec![
    Builtin::new(
      String::from("exit"),
      Proto(vec![Ty::INT.into()], Ty::VOID.into()),
    ),
    // fixme: `Undefined symbols for architecture x86_64: _create", referenced from: _main in main.o`
    // the declaration is defined here: `library/core/src/sys.rs`
    Builtin::new(
      String::from("create"),
      Proto(vec![Ty::STR.into(), Ty::STR.into()], Ty::VOID.into()),
    ),
    // fixme: `dyld: BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB has segment 1 which is not writable`
    // the declaration is defined here: `library/core/src/sys.rs`
    Builtin::new(
      String::from("open"),
      Proto(vec![Ty::STR.into()], Ty::STR.into()),
    ),
  ]
}
