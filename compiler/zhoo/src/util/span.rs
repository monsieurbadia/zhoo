use std::fmt::{Display, Formatter, Result};
use std::ops::Range;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Span {
  pub lo: u32,
  pub hi: u32,
}

impl Span {
  pub const ZERO: Self = Self::new(0, 0);

  pub const fn new(lo: usize, hi: usize) -> Self {
    Self {
      lo: lo as u32,
      hi: hi as u32,
    }
  }

  pub fn merge(a: &Span, b: &Span) -> Span {
    use std::cmp::{max, min};

    let lo = min(a.lo, b.lo);
    let hi = max(a.hi, b.hi);

    Self::new(lo as usize, hi as usize)
  }
}

impl From<Span> for Range<usize> {
  fn from(span: Span) -> Self {
    span.lo as usize..span.hi as usize
  }
}

#[derive(Copy, Clone, Debug, Eq)]
pub struct Spanned<T> {
  pub span: Span,
  pub node: T,
}

impl<T: Display> Display for Spanned<T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    self.node.fmt(f)
  }
}

impl<T> From<(T, Span, Span)> for Spanned<T> {
  fn from(other: (T, Span, Span)) -> Self {
    Self {
      node: other.0,
      span: Span::new(other.1.lo as usize, other.2.hi as usize),
    }
  }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
  fn eq(&self, rhs: &Spanned<T>) -> bool {
    self.node.eq(&rhs.node)
  }
}

impl<T> Spanned<T> {
  pub const fn new(node: T, span: Span) -> Self {
    Self { node, span }
  }
}
