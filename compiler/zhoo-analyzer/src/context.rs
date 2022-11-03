use super::builtins::{c_builtins, io_builtins, sys_builtins};
use super::scope::ScopeMap;

use zhoo_ast::ast::{Program, Ty};
use zhoo_ast::ptr::Fsp;

#[derive(Clone, Debug)]
pub(crate) struct Context<'a> {
  pub program: &'a Program,
  pub scope_map: ScopeMap,
  pub return_ty: Fsp<Ty>,
  pub loop_depth: i32,
}

impl<'a> Context<'a> {
  pub fn new(program: &'a Program) -> Self {
    let builtins = vec![c_builtins(), io_builtins(), sys_builtins()];
    let mut scope_map = ScopeMap::default();

    for builtin in builtins.into_iter().flatten() {
      scope_map
        .set_fun(builtin.name, (builtin.proto.0, builtin.proto.1))
        .expect("declare builtin");
    }

    Self {
      program,
      scope_map,
      return_ty: Ty::VOID.into(),
      loop_depth: 0,
    }
  }
}
