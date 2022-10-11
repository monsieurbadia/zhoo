use crate::front::parser::tree::ast::Program;

#[derive(Clone, Debug)]
pub struct Context<'a> {
  pub program: &'a Program,
}

impl<'a> Context<'a> {
  pub fn new(program: &'a Program) -> Self {
    Self { program }
  }
}
