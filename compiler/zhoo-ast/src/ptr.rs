//! frozen owned smart pointer
//! @see https://github.com/rust-lang/rust/blob/master/compiler/rustc_ast/src/ptr.rs

use std::fmt::{Debug, Display, Formatter, Pointer, Result};
use std::ops::{Deref, DerefMut};

pub struct Fsp<T: ?Sized>(pub Box<T>);

pub fn fsp<T: 'static>(value: T) -> Fsp<T> {
  Fsp(Box::new(value))
}

impl<T: 'static + Clone> Clone for Fsp<T> {
  fn clone(&self) -> Fsp<T> {
    Fsp(Box::new((**self).clone()))
  }
}

impl<T> Deref for Fsp<T> {
  type Target = T;

  fn deref<'a>(&'_ self) -> &'_ T {
    &self.0
  }
}

impl<T> DerefMut for Fsp<T> {
  fn deref_mut<'a>(&'_ mut self) -> &'_ mut T {
    &mut self.0
  }
}

impl<T: ?Sized + Debug> Debug for Fsp<T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    Debug::fmt(&self.0, f)
  }
}

impl<T: Display> Display for Fsp<T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    Display::fmt(&**self, f)
  }
}

impl<T: PartialEq> PartialEq for Fsp<T> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<T> Pointer for Fsp<T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    Pointer::fmt(&self.0, f)
  }
}
