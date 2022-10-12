use super::builtins::{io_builtins, sys_builtins};
use super::scope::ScopeMap;

use crate::front::parser::tree::ast::Program;
use crate::front::parser::tree::ty::Ty;
use crate::front::parser::tree::PBox;

#[derive(Clone, Debug)]
pub struct Context<'a> {
  pub program: &'a Program,
  pub scope_map: ScopeMap,
  pub return_ty: PBox<Ty>,
  pub loops: i32,
}

impl<'a> Context<'a> {
  pub fn new(program: &'a Program) -> Self {
    let mut scope_map = ScopeMap::default();

    for builtin in io_builtins() {
      let _ = scope_map
        .set_fun(builtin.name, (builtin.proto.0.clone(), builtin.proto.1));
    }

    for builtin in sys_builtins() {
      let _ = scope_map
        .set_fun(builtin.name, (builtin.proto.0.clone(), builtin.proto.1));
    }

    Self {
      program,
      scope_map,
      return_ty: Ty::VOID.into(),
      loops: 0,
    }
  }
}
