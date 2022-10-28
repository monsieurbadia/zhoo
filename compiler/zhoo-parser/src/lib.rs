//! this module is used for the `syntax analysis` of the `zhoo` compiler

#[macro_use]
extern crate lalrpop_util;

mod grammar;

pub mod parser;
pub mod tree;
