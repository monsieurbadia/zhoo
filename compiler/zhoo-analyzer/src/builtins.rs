mod c;
mod io;
mod sys;

pub use c::c_builtins;
pub use io::io_builtins;
pub use sys::sys_builtins;

use zhoo_ast::ast::Ty;
use zhoo_ast::ptr::Fsp;

pub struct Builtin {
  pub name: String,
  pub proto: Proto,
}

impl Builtin {
  pub const fn new(name: String, proto: Proto) -> Self {
    Self { name, proto }
  }
}

pub struct Proto(pub Vec<Fsp<Ty>>, pub Fsp<Ty>);
