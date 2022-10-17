use super::ast::{Expr, ExprKind};
use super::pbox::PBox;

use crate::util::span::Span;

pub trait AsTy: Sized {
  fn as_ty(&self) -> PBox<Ty>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ty {
  pub kind: TyKind,
  pub span: Span,
}

impl Ty {
  pub const VOID: Self = Self::new(TyKind::Void, Span::ZERO);
  pub const BOOL: Self = Self::new(TyKind::Bool, Span::ZERO);
  pub const INT: Self = Self::new(TyKind::Int, Span::ZERO);
  pub const REAL: Self = Self::new(TyKind::Real, Span::ZERO);
  pub const STR: Self = Self::new(TyKind::Str, Span::ZERO);
  pub const INFER: Self = Self::new(TyKind::Infer, Span::ZERO);

  pub const fn new(kind: TyKind, span: Span) -> Self {
    Self { kind, span }
  }

  pub const fn with_void(span: Span) -> Self {
    Self::new(TyKind::Void, span)
  }

  pub const fn with_bool(span: Span) -> Self {
    Self::new(TyKind::Bool, span)
  }

  pub const fn with_int(span: Span) -> Self {
    Self::new(TyKind::Int, span)
  }

  pub const fn with_real(span: Span) -> Self {
    Self::new(TyKind::Real, span)
  }

  pub const fn with_str(span: Span) -> Self {
    Self::new(TyKind::Str, span)
  }

  pub const fn with_lambda(
    args: Vec<PBox<Ty>>,
    return_ty: PBox<Ty>,
    span: Span,
  ) -> Self {
    Self::new(TyKind::Fn(args, return_ty), span)
  }

  pub const fn with_array(ty: PBox<Ty>, size: Option<i64>, span: Span) -> Self {
    Self::new(TyKind::Array(ty, size), span)
  }

  pub const fn with_tuple(elements: Vec<PBox<Ty>>, span: Span) -> Self {
    Self::new(TyKind::Tuple(elements), span)
  }

  pub fn is_numeric(&self) -> bool {
    self.kind.is_numeric()
  }

  pub fn is_boolean(&self) -> bool {
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

#[derive(Clone, Debug)]
pub enum TyKind {
  Void,
  Bool,
  Int,
  Real,
  Str,
  Infer,
  Fn(Vec<PBox<Ty>>, PBox<Ty>),
  Array(PBox<Ty>, Option<i64>),
  Tuple(Vec<PBox<Ty>>),
}

impl TyKind {
  fn is_boolean(&self) -> bool {
    matches!(self, Self::Bool)
  }

  fn is_numeric(&self) -> bool {
    matches!(self, Self::Int | Self::Real)
  }

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
