// allow internal warnings from lalrpop
#![allow(clippy::clone_on_copy)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::let_unit_value)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unused_unit)]

lalrpop_mod!(grammar, "/front/grammar/grammar.rs");

pub use super::grammar::grammar::ProgramParser;
