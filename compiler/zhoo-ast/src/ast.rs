use super::ptr::Fsp;

use zhoo_errors::Reporter;
use zhoo_span::span::{Span, Spanned};

pub type TyOrInfer = (Option<Fsp<Ty>>, Fsp<Expr>);

#[derive(Clone, Debug)]
pub enum Public {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub enum Async {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub enum Unsafe {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub enum Wasm {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub enum Mutability {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub struct Pattern {
  pub kind: PatternKind,
  pub span: Span,
}

impl Pattern {
  pub const fn new(kind: PatternKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum PatternKind {
  Underscore,
  Identifier(Fsp<Expr>),
  Lit(Fsp<Expr>),
  MeLower,
}

#[derive(Debug)]
pub struct Program {
  pub stmts: Vec<Fsp<Stmt>>,
  pub reporter: Reporter,
  pub span: Span,
}

impl Program {
  pub const fn new(
    stmts: Vec<Fsp<Stmt>>,
    span: Span,
    reporter: Reporter,
  ) -> Self {
    Self {
      stmts,
      span,
      reporter,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Stmt {
  pub kind: StmtKind,
  pub span: Span,
}

impl Stmt {
  pub const fn new(kind: StmtKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum StmtKind {
  Ext(Fsp<Ext>),
  TyAlias(Fsp<TyAlias>),
  Val(Fsp<Decl>),
  Fun(Fsp<Fun>),
  Unit(Fsp<Unit>),
}

#[derive(Clone, Debug)]
pub struct Ext {
  pub public: Public,
  pub prototype: Prototype,
  pub body: Option<Fsp<Block>>,
  pub span: Span,
}

impl Ext {
  pub const fn new(
    public: Public,
    prototype: Prototype,
    body: Option<Fsp<Block>>,
    span: Span,
  ) -> Self {
    Self {
      public,
      prototype,
      body,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct TyAlias {
  pub public: Public,
  pub name: Fsp<Expr>,
  pub kind: TyAliasKind,
  pub span: Span,
}

impl TyAlias {
  pub const fn new(
    public: Public,
    name: Fsp<Expr>,
    kind: TyAliasKind,
    span: Span,
  ) -> Self {
    Self {
      public,
      name,
      kind,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub enum TyAliasKind {
  Single(Fsp<Ty>),
  Group(Vec<Fsp<TyAliasField>>),
}

#[derive(Clone, Debug)]
pub struct TyAliasField {
  pub name: Fsp<Expr>,
  pub ty: Fsp<Ty>,
  pub span: Span,
}

impl TyAliasField {
  pub const fn new(name: Fsp<Expr>, ty: Fsp<Ty>, span: Span) -> Self {
    Self { name, ty, span }
  }
}

#[derive(Clone, Debug)]
pub struct Decl {
  pub mutability: Mutability,
  pub kind: DeclKind,
  pub pattern: Pattern,
  pub ty: Option<Fsp<Ty>>,
  pub value: Fsp<Expr>,
  pub span: Span,
}

impl Decl {
  pub const fn new(
    mutability: Mutability,
    kind: DeclKind,
    pattern: Pattern,
    ty: Option<Fsp<Ty>>,
    value: Fsp<Expr>,
    span: Span,
  ) -> Self {
    Self {
      mutability,
      kind,
      pattern,
      ty,
      value,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub enum DeclKind {
  Val,
  Imu,
  Mut,
}

#[derive(Clone, Debug)]
pub struct Fun {
  pub public: Public,
  pub asyncness: Async,
  pub unsafeness: Unsafe,
  pub wasm: Wasm,
  pub prototype: Prototype,
  pub body: Fsp<Block>,
  pub span: Span,
}

impl Fun {
  pub const fn new(
    public: Public,
    asyncness: Async,
    unsafeness: Unsafe,
    wasm: Wasm,
    prototype: Prototype,
    body: Fsp<Block>,
    span: Span,
  ) -> Self {
    Self {
      public,
      asyncness,
      unsafeness,
      wasm,
      prototype,
      body,
      span,
    }
  }
}

impl AsTy for Fun {
  fn as_ty(&self) -> Fsp<Ty> {
    self.prototype.as_ty()
  }
}

#[derive(Clone, Debug)]
pub struct Prototype {
  pub name: Fsp<Expr>,
  pub inputs: Vec<Fsp<Arg>>,
  pub output: ReturnTy,
  pub span: Span,
}

impl Prototype {
  pub const fn new(
    name: Fsp<Expr>,
    inputs: Vec<Fsp<Arg>>,
    output: ReturnTy,
    span: Span,
  ) -> Self {
    Self {
      name,
      inputs,
      output,
      span,
    }
  }

  pub fn as_inputs_tys(&self) -> Vec<Fsp<Ty>> {
    self
      .inputs
      .iter()
      .map(|input| input.ty.to_owned())
      .collect::<Vec<_>>()
  }
}

impl AsTy for Prototype {
  fn as_ty(&self) -> Fsp<Ty> {
    self.output.as_ty()
  }
}

#[derive(Clone, Debug)]
pub struct Arg {
  pub pattern: Pattern,
  pub ty: Fsp<Ty>,
  pub span: Span,
}

impl Arg {
  pub const fn new(pattern: Pattern, ty: Fsp<Ty>, span: Span) -> Self {
    Self { pattern, ty, span }
  }
}

#[derive(Clone, Debug)]
pub enum ReturnTy {
  Default(Span),
  Ty(Fsp<Ty>),
}

impl AsTy for ReturnTy {
  fn as_ty(&self) -> Fsp<Ty> {
    match self {
      Self::Ty(ty) => ty.clone(),
      Self::Default(span) => Ty::new(TyKind::Void, *span).into(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Block {
  pub exprs: Vec<Fsp<Expr>>,
  pub span: Span,
}

impl Block {
  pub const fn new(exprs: Vec<Fsp<Expr>>, span: Span) -> Self {
    Self { exprs, span }
  }
}

#[derive(Clone, Debug)]
pub struct Unit {
  pub binds: Vec<Fsp<Stmt>>,
  pub mocks: Vec<Fsp<Fun>>,
  pub tests: Vec<Fsp<Fun>>,
  pub span: Span,
}

impl Unit {
  pub const fn new(
    binds: Vec<Fsp<Stmt>>,
    mocks: Vec<Fsp<Fun>>,
    tests: Vec<Fsp<Fun>>,
    span: Span,
  ) -> Self {
    Self {
      binds,
      mocks,
      tests,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Expr {
  pub kind: ExprKind,
  pub span: Span,
}

impl Expr {
  pub const fn new(kind: ExprKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum ExprKind {
  Lit(Fsp<Lit>),
  Identifier(String),
  UnOp(UnOp, Fsp<Expr>),
  BinOp(Fsp<Expr>, BinOp, Fsp<Expr>),
  Call(Fsp<Expr>, Vec<Fsp<Expr>>),
  Decl(Fsp<Decl>),
  Assign(Fsp<Expr>, BinOp, Fsp<Expr>),
  AssignOp(Fsp<Expr>, BinOp, Fsp<Expr>),
  Block(Fsp<Block>),
  Loop(Fsp<Block>),
  While(Fsp<Expr>, Fsp<Block>),
  Until(Fsp<Expr>, Fsp<Block>),
  Return(Option<Fsp<Expr>>),
  Break(Option<Fsp<Expr>>),
  Continue,
  When(Fsp<Expr>, Fsp<Expr>, Fsp<Expr>),
  IfElse(Fsp<Expr>, Fsp<Expr>, Option<Fsp<Expr>>),
  Lambda(Vec<Fsp<Expr>>, Fsp<Expr>),
  Array(Vec<Fsp<Expr>>),
  ArrayAccess(Fsp<Expr>, Fsp<Expr>),
  Tuple(Vec<Fsp<Expr>>),
  TupleAccess(Fsp<Expr>, Fsp<Expr>),
  Stmt(Fsp<Stmt>),
}

#[derive(Clone, Debug)]
pub struct Lit {
  pub kind: LitKind,
  pub span: Span,
}

impl Lit {
  pub const fn new(kind: LitKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum LitKind {
  Bool(bool),
  Int(i64),
  Real(f64),
  Str(String),
}

pub type UnOp = Spanned<UnOpKind>;

#[derive(Clone, Debug)]
pub enum UnOpKind {
  Not,
  Neg,
}

pub type BinOp = Spanned<BinOpKind>;

#[derive(Clone, Debug)]
pub enum BinOpKind {
  Add,    // +
  Sub,    // -
  Mul,    // *
  Div,    // /
  Rem,    // %
  And,    // &&
  Or,     // ||
  Lt,     // <
  Gt,     // >
  Le,     // <=
  Ge,     // >=
  Eq,     // ==
  Ne,     // !=
  Shl,    // <<
  Shr,    // >>
  BitAnd, // &
  BitOr,  // |
  BitXor, // ^
  As,     // as
  Range,  // ..
}

impl BinOpKind {
  pub fn is_assign_op(&self) -> bool {
    matches!(
      self,
      Self::Add
        | Self::Sub
        | Self::Mul
        | Self::Div
        | Self::Rem
        | Self::BitXor
        | Self::BitAnd
        | Self::BitOr
    )
  }
}

pub trait AsTy: Sized {
  fn as_ty(&self) -> Fsp<Ty>;
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

  pub fn kind(&self) -> &TyKind {
    &self.kind
  }

  pub fn is_numeric(&self) -> bool {
    self.kind.is_numeric()
  }

  pub fn is_boolean(&self) -> bool {
    self.kind.is_boolean()
  }

  pub fn is_int(&self) -> bool {
    self.kind.is_int()
  }
}

impl From<Ty> for Fsp<Ty> {
  fn from(ty: Ty) -> Self {
    Fsp(Box::new(ty))
  }
}

impl From<&Ty> for Fsp<Ty> {
  fn from(ty: &Ty) -> Self {
    Fsp(Box::new(ty.clone()))
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TyKind {
  Void,
  Bool,
  Int,
  Real,
  Str,
  Infer,
  Fn(Vec<Fsp<Ty>>, Fsp<Ty>),
  Array(Fsp<Ty>, Option<i64>),
  Tuple(Vec<Fsp<Ty>>),
}

impl TyKind {
  fn is_numeric(&self) -> bool {
    matches!(self, Self::Int | Self::Real)
  }

  fn is_boolean(&self) -> bool {
    matches!(self, Self::Bool)
  }

  pub fn is_int(&self) -> bool {
    matches!(self, Self::Int)
  }
}
