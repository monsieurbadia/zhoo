use super::ast::{Expr, ExprKind};
use super::pbox::PBox;

use crate::util::span::Span;

/// a trait for a free conversion into a ty
pub(crate) trait AsTy: Sized {
  fn as_ty(&self) -> PBox<Ty>;
}

/// an instance of a ty
#[derive(Clone, Debug, PartialEq)]
pub struct Ty {
  pub kind: TyKind,
  pub span: Span,
}

impl Ty {
  /// a `void` type
  pub(crate) const VOID: Self = Self::new(TyKind::Void, Span::ZERO);

  /// a `bool` type
  pub(crate) const BOOL: Self = Self::new(TyKind::Bool, Span::ZERO);

  /// a `int` type
  pub(crate) const INT: Self = Self::new(TyKind::Int, Span::ZERO);

  /// a `real` type
  pub(crate) const REAL: Self = Self::new(TyKind::Real, Span::ZERO);

  /// a `str` type
  pub(crate) const STR: Self = Self::new(TyKind::Str, Span::ZERO);

  /// a `infer` type
  pub(crate) const INFER: Self = Self::new(TyKind::Infer, Span::ZERO);

  /// create an instance of ty
  #[inline]
  pub(crate) const fn new(kind: TyKind, span: Span) -> Self {
    Self { kind, span }
  }

  /// create an instance of a `void` type
  #[inline]
  pub(crate) const fn with_void(span: Span) -> Self {
    Self::new(TyKind::Void, span)
  }

  /// create an instance of a `bool` type
  #[inline]
  pub(crate) const fn with_bool(span: Span) -> Self {
    Self::new(TyKind::Bool, span)
  }

  /// create an instance of a `int` type
  #[inline]
  pub(crate) const fn with_int(span: Span) -> Self {
    Self::new(TyKind::Int, span)
  }

  /// create an instance of a `real` type
  #[inline]
  pub(crate) const fn with_real(span: Span) -> Self {
    Self::new(TyKind::Real, span)
  }

  /// create an instance of a `str` type
  #[inline]
  pub(crate) const fn with_str(span: Span) -> Self {
    Self::new(TyKind::Str, span)
  }

  /// create an instance of a `lambda` type
  #[inline]
  pub(crate) const fn _with_lambda(
    args: Vec<PBox<Ty>>,
    return_ty: PBox<Ty>,
    span: Span,
  ) -> Self {
    Self::new(TyKind::Fn(args, return_ty), span)
  }

  /// create an instance of a `array` type
  #[inline]
  pub(crate) const fn with_array(
    ty: PBox<Ty>,
    size: Option<i64>,
    span: Span,
  ) -> Self {
    Self::new(TyKind::Array(ty, size), span)
  }

  /// create an instance of a `tuple` type
  #[inline]
  pub(crate) const fn with_tuple(elements: Vec<PBox<Ty>>, span: Span) -> Self {
    Self::new(TyKind::Tuple(elements), span)
  }

  /// check if a variant of a ty is a numeric ty
  pub(crate) fn is_numeric(&self) -> bool {
    self.kind.is_numeric()
  }

  /// check if a variant of a ty is a boolean ty
  pub(crate) fn is_boolean(&self) -> bool {
    self.kind.is_boolean()
  }
}

impl From<PBox<Expr>> for Ty {
  fn from(expr: PBox<Expr>) -> Self {
    let kind = if let ExprKind::Identifier(identifier) = &expr.kind {
      match identifier.as_str() {
        "void" => TyKind::Void,
        "bool" => TyKind::Bool,
        "int" => TyKind::Int,
        "real" => TyKind::Real,
        "str" => TyKind::Str,
        _ => panic!("from ty error"),
      }
    } else {
      TyKind::Void
    };

    Ty::new(kind, expr.span)
  }
}

impl From<Ty> for PBox<Ty> {
  fn from(ty: Ty) -> Self {
    PBox(Box::new(ty))
  }
}

impl From<&Ty> for PBox<Ty> {
  fn from(ty: &Ty) -> Self {
    PBox(Box::new(ty.clone()))
  }
}

/// a ty kind enumeration
#[derive(Clone, Debug)]
pub enum TyKind {
  /// a variant for `void` ty
  Void,

  /// a variant for `bool` ty
  Bool,

  /// a variant for `int` ty
  Int,

  /// a variant for `real` ty
  Real,

  /// a variant for `str` ty
  Str,

  /// a variant for `infer` ty
  Infer,

  /// a variant for `lambda` ty i.e `fn(x:int): int`
  Fn(Vec<PBox<Ty>>, PBox<Ty>),

  /// a variant for `array` ty i.e `int[]`
  Array(PBox<Ty>, Option<i64>),

  /// a variant for `tuple` ty i.e `(int, int)`
  Tuple(Vec<PBox<Ty>>),
}

impl TyKind {
  /// check if a variant of a ty is a numeric ty
  fn is_numeric(&self) -> bool {
    matches!(self, Self::Int | Self::Real)
  }

  /// check if a variant of a ty is a boolean ty
  fn is_boolean(&self) -> bool {
    matches!(self, Self::Bool)
  }

  /// check if a variant of a ty is a integer ty
  pub fn is_int(&self) -> bool {
    matches!(self, Self::Int)
  }
}

impl PartialEq for TyKind {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Void, Self::Void)
      | (Self::Bool, Self::Bool)
      | (Self::Int, Self::Int)
      | (Self::Real, Self::Real)
      | (Self::Str, Self::Str)
      | (Self::Infer, Self::Infer) => true,
      (Self::Fn(_, lhs), Self::Fn(_, rhs)) => lhs.kind == rhs.kind,
      (Self::Array(lhs, _), Self::Array(rhs, _)) => lhs.kind == rhs.kind,
      (Self::Tuple(lhs), Self::Tuple(rhs)) => lhs == rhs,
      _ => false,
    }
  }
}
