// todo
//
// - [ ] unit tests
// - [ ] update `type checker`
// - [ ] update `name checker`
// - [ ] update `codegen`
// - [ ] implement `type inference` (w algorithm - hindley milner)
// - [ ] implement module system
// - [ ] update syntax error messages
// - [ ] `codegen` for `lambda`

#[macro_use]
extern crate lalrpop_util;

pub mod back;
pub mod front;
pub mod util;
