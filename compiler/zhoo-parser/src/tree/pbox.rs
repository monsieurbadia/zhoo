use std::fmt::{Debug, Display, Formatter, Pointer, Result};
use std::ops::{Deref, DerefMut};

/// an instance of a pbox smart pointer
pub struct PBox<T: ?Sized>(pub Box<T>);

/// create an instance of a pbox smart pointer
#[inline]
pub fn pbox<T: 'static>(value: T) -> PBox<T> {
  PBox(Box::new(value))
}

impl<T: 'static + Clone> Clone for PBox<T> {
  fn clone(&self) -> PBox<T> {
    PBox(Box::new((**self).clone()))
  }
}

impl<T> Deref for PBox<T> {
  type Target = T;

  fn deref<'a>(&'_ self) -> &'_ T {
    &self.0
  }
}

impl<T> DerefMut for PBox<T> {
  fn deref_mut<'a>(&'_ mut self) -> &'_ mut T {
    &mut self.0
  }
}

impl<T: ?Sized + Debug> Debug for PBox<T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    Debug::fmt(&self.0, f)
  }
}

impl<T: Display> Display for PBox<T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    Display::fmt(&**self, f)
  }
}

impl<T: PartialEq> PartialEq for PBox<T> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<T> Pointer for PBox<T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    Pointer::fmt(&self.0, f)
  }
}
