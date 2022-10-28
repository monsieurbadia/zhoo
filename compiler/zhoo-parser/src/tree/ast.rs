use crate::tree::pbox::PBox;
use crate::tree::ty::{AsTy, Ty};
use zhoo_util::error::Reporter;
use zhoo_util::span::{Span, Spanned};

// this is used only to avoid the clippy error `very complex type used`
pub type PatternWithTyOrInfer = (Pattern, Option<PBox<Ty>>, PBox<Expr>);
pub type TyOrInfer = (Option<PBox<Ty>>, PBox<Expr>);

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
pub enum Value {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub struct Pattern {
  pub kind: PatternKind,
  pub span: Span,
}

impl Pattern {
  #[inline]
  pub const fn new(kind: PatternKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum PatternKind {
  Underscore,
  Identifier(PBox<Expr>),
  Lit(PBox<Expr>),
  MeLower,
}

#[derive(Debug)]
pub struct Program {
  pub stmts: Vec<PBox<Stmt>>,
  pub reporter: Reporter,
  pub span: Span,
}

impl Program {
  #[inline]
  pub const fn new(
    stmts: Vec<PBox<Stmt>>,
    reporter: Reporter,
    span: Span,
  ) -> Self {
    Self {
      stmts,
      reporter,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Stmt {
  pub kind: StmtKind,
  pub span: Span,
}

impl Stmt {
  #[inline]
  pub const fn new(kind: StmtKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum StmtKind {
  Ext(PBox<Ext>),
  MacroDecl(PBox<MacroDecl>),
  MacroCall(PBox<MacroCall>),
  TyAlias(PBox<TyAlias>),
  Behavior(PBox<Behavior>),
  Enum(PBox<Enum>),
  Struct(PBox<Struct>),
  Apply(PBox<Apply>),
  Val(PBox<Decl>),
  Vals(Vec<PBox<Decl>>), // tmp
  Fun(PBox<Fun>),
  Unit(PBox<Unit>),
}

#[derive(Clone, Debug)]
pub struct Ext {
  pub public: Public,
  pub prototype: Prototype,
  pub body: Option<PBox<Block>>,
  pub span: Span,
}

impl Ext {
  #[inline]
  pub const fn new(
    public: Public,
    prototype: Prototype,
    body: Option<PBox<Block>>,
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
pub struct MacroDecl {
  pub public: Public,
  pub name: PBox<Expr>,
  pub tree: PBox<MacroDeclDef>,
  pub span: Span,
}

impl MacroDecl {
  #[inline]
  pub const fn new(
    public: Public,
    name: PBox<Expr>,
    tree: PBox<MacroDeclDef>,
    span: Span,
  ) -> Self {
    Self {
      public,
      name,
      tree,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct MacroDeclDef {
  pub kind: MacroDeclDefKind,
  pub macro_decl_defs: Vec<PBox<MacroDeclDef>>,
  pub span: Span,
}

impl MacroDeclDef {
  #[inline]
  pub const fn new(
    kind: MacroDeclDefKind,
    macro_decl_defs: Vec<PBox<MacroDeclDef>>,
    span: Span,
  ) -> Self {
    Self {
      kind,
      macro_decl_defs,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub enum MacroDeclDefKind {
  Parenthesis,
  Braces,
  Brackets,
}

#[derive(Clone, Debug)]
pub struct MacroCall {
  pub pattern: Pattern,
  pub macro_decl_def: PBox<MacroDeclDef>,
  pub span: Span,
}

impl MacroCall {
  #[inline]
  pub const fn new(
    pattern: Pattern,
    macro_decl_def: PBox<MacroDeclDef>,
    span: Span,
  ) -> Self {
    Self {
      pattern,
      macro_decl_def,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct TyAlias {
  pub public: Public,
  pub name: PBox<Expr>,
  pub kind: TyAliasKind,
  pub span: Span,
}

impl TyAlias {
  #[inline]
  pub const fn new(
    public: Public,
    name: PBox<Expr>,
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
  Single(PBox<Ty>),
  Group(Vec<PBox<TyAliasField>>),
}

#[derive(Clone, Debug)]
pub struct TyAliasField {
  pub name: PBox<Expr>,
  pub ty: PBox<Ty>,
  pub span: Span,
}

impl TyAliasField {
  #[inline]
  pub const fn new(name: PBox<Expr>, ty: PBox<Ty>, span: Span) -> Self {
    Self { name, ty, span }
  }
}

#[derive(Clone, Debug)]
pub struct Behavior {
  pub public: Public,
  pub identifier: Pattern,
  pub elements: Vec<PBox<BehaviorElement>>,
  pub span: Span,
}

impl Behavior {
  #[inline]
  pub const fn new(
    public: Public,
    identifier: Pattern,
    elements: Vec<PBox<BehaviorElement>>,
    span: Span,
  ) -> Self {
    Self {
      public,
      identifier,
      elements,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub enum BehaviorElement {
  Fun(PBox<Fun>),
  TyAlias(PBox<TyAlias>),
}

#[derive(Clone, Debug)]
pub struct Enum {
  pub public: Public,
  pub name: PBox<Expr>,
  pub variants: Vec<PBox<EnumVariant>>,
  pub span: Span,
}

impl Enum {
  #[inline]
  pub const fn new(
    public: Public,
    name: PBox<Expr>,
    variants: Vec<PBox<EnumVariant>>,
    span: Span,
  ) -> Self {
    Self {
      public,
      name,
      variants,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
  pub name: PBox<Expr>,
  pub arg: Option<PBox<EnumVariantArg>>,
  pub span: Span,
}

impl EnumVariant {
  #[inline]
  pub const fn new(
    name: PBox<Expr>,
    arg: Option<PBox<EnumVariantArg>>,
    span: Span,
  ) -> Self {
    Self { name, arg, span }
  }
}

#[derive(Clone, Debug)]
pub struct EnumVariantArg {
  pub value: PBox<Expr>,
  pub span: Span,
}

impl EnumVariantArg {
  #[inline]
  pub const fn new(value: PBox<Expr>, span: Span) -> Self {
    Self { value, span }
  }
}

#[derive(Clone, Debug)]
pub struct Struct {
  pub public: Public,
  pub name: PBox<Expr>,
  pub kind: StructKind,
  pub span: Span,
}

impl Struct {
  #[inline]
  pub const fn new(
    public: Public,
    name: PBox<Expr>,
    kind: StructKind,
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
pub enum StructKind {
  Init,
  Decl(Vec<PBox<StructDeclField>>),
  Tuple(Vec<PBox<StructTupleField>>),
}

#[derive(Clone, Debug)]
pub struct StructDeclField {
  pub public: Public,
  pub name: PBox<Expr>,
  pub ty: PBox<Ty>,
  pub span: Span,
}

impl StructDeclField {
  #[inline]
  pub const fn new(
    public: Public,
    name: PBox<Expr>,
    ty: PBox<Ty>,
    span: Span,
  ) -> Self {
    Self {
      public,
      name,
      ty,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct StructTupleField {
  pub public: Public,
  pub ty: PBox<Ty>,
  pub span: Span,
}

impl StructTupleField {
  #[inline]
  pub const fn new(public: Public, ty: PBox<Ty>, span: Span) -> Self {
    Self { public, ty, span }
  }
}

#[derive(Clone, Debug)]
pub struct Apply {
  pub unsafeness: Unsafe,
  pub kind: ApplyKind,
  pub elements: Vec<PBox<ImplElement>>,
}

impl Apply {
  #[inline]
  pub const fn new(
    unsafeness: Unsafe,
    kind: ApplyKind,
    elements: Vec<PBox<ImplElement>>,
  ) -> Self {
    Self {
      unsafeness,
      kind,
      elements,
    }
  }
}

#[derive(Clone, Debug)]
pub enum ApplyKind {
  Behavior(Option<PBox<Behavior>>),
  Ty(PBox<Ty>),
}

#[derive(Clone, Debug)]
pub enum ImplElement {
  Fun(PBox<Fun>),
  TyAlias(PBox<TyAlias>),
}

#[derive(Clone, Debug)]
pub struct Decl {
  pub mutability: Mutability,
  pub kind: DeclKind,
  pub pattern: Pattern,
  pub ty: Option<PBox<Ty>>,
  pub value: PBox<Expr>,
  pub span: Span,
}

impl Decl {
  #[inline]
  pub const fn new(
    mutability: Mutability,
    kind: DeclKind,
    pattern: Pattern,
    ty: Option<PBox<Ty>>,
    value: PBox<Expr>,
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
  Decls(Vec<PBox<Decl>>),
}

#[derive(Clone, Debug)]
pub struct Fun {
  pub public: Public,
  pub asyncness: Async,
  pub unsafeness: Unsafe,
  pub wasm: Wasm,
  pub prototype: Prototype,
  pub body: PBox<Block>,
  pub span: Span,
}

impl Fun {
  #[inline]
  pub const fn new(
    public: Public,
    asyncness: Async,
    unsafeness: Unsafe,
    wasm: Wasm,
    prototype: Prototype,
    body: PBox<Block>,
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
  fn as_ty(&self) -> PBox<Ty> {
    self.prototype.as_ty()
  }
}

#[derive(Clone, Debug)]
pub struct Prototype {
  pub pattern: PBox<Expr>,
  pub inputs: Vec<PBox<Arg>>,
  pub output: ReturnTy,
  pub span: Span,
}

impl Prototype {
  #[inline]
  pub const fn new(
    pattern: PBox<Expr>,
    inputs: Vec<PBox<Arg>>,
    output: ReturnTy,
    span: Span,
  ) -> Self {
    Self {
      pattern,
      inputs,
      output,
      span,
    }
  }

  pub fn as_inputs_tys(&self) -> Vec<PBox<Ty>> {
    self
      .inputs
      .iter()
      .map(|input| input.ty.to_owned())
      .collect::<Vec<_>>()
  }
}

impl AsTy for Prototype {
  fn as_ty(&self) -> PBox<Ty> {
    self.output.as_ty()
  }
}

#[derive(Clone, Debug)]
pub struct Arg {
  pub pattern: Pattern,
  pub ty: PBox<Ty>,
  pub span: Span,
}

impl Arg {
  #[inline]
  pub const fn new(pattern: Pattern, ty: PBox<Ty>, span: Span) -> Self {
    Self { pattern, ty, span }
  }
}

#[derive(Clone, Debug)]
pub enum ReturnTy {
  Default(Span),
  Ty(PBox<Ty>),
}

impl AsTy for ReturnTy {
  fn as_ty(&self) -> PBox<Ty> {
    match self {
      Self::Ty(ty) => ty.clone(),
      Self::Default(span) => Ty::with_void(*span).into(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Block {
  pub exprs: Vec<PBox<Expr>>,
  pub span: Span,
}

impl Block {
  #[inline]
  pub const fn new(exprs: Vec<PBox<Expr>>, span: Span) -> Self {
    Self { exprs, span }
  }
}

#[derive(Clone, Debug)]
pub struct Unit {
  pub binds: Vec<PBox<Stmt>>,
  pub mocks: Vec<PBox<Fun>>,
  pub tests: Vec<PBox<Fun>>,
  pub span: Span,
}

impl Unit {
  #[inline]
  pub const fn new(
    binds: Vec<PBox<Stmt>>,
    mocks: Vec<PBox<Fun>>,
    tests: Vec<PBox<Fun>>,
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
  #[inline]
  pub const fn new(kind: ExprKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum ExprKind {
  Stmt(PBox<Stmt>),
  Decl(PBox<Decl>),
  Decls(Vec<PBox<Decl>>), // tmp
  Lit(PBox<Lit>),
  Identifier(String),
  UnOp(UnOp, PBox<Expr>),
  BinOp(PBox<Expr>, BinOp, PBox<Expr>),
  Call(PBox<Expr>, Vec<PBox<Expr>>),
  Assign(PBox<Expr>, BinOp, PBox<Expr>),
  AssignOp(PBox<Expr>, BinOp, PBox<Expr>),
  Return(Option<PBox<Expr>>),
  Block(PBox<Block>),
  Loop(PBox<Block>),
  While(PBox<Expr>, PBox<Block>),
  Until(PBox<Expr>, PBox<Block>),
  Break(Option<PBox<Expr>>),
  Continue,
  Raise(Option<PBox<Expr>>),
  When(PBox<Expr>, PBox<Expr>, PBox<Expr>),
  IfElse(PBox<Expr>, PBox<Expr>, Option<PBox<Expr>>),
  Lambda(Vec<PBox<Expr>>, PBox<Expr>),
  Array(Vec<PBox<Expr>>),
  ArrayAccess(PBox<Expr>, PBox<Expr>),
  Tuple(Vec<PBox<Expr>>),
  TupleAccess(PBox<Expr>, PBox<Expr>),
  Struct(PBox<Struct>),
  StructAccess(PBox<Expr>, PBox<Expr>),
}

#[derive(Clone, Debug)]
pub struct Lit {
  pub kind: LitKind,
  pub span: Span,
}

impl Lit {
  #[inline]
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
