use super::ast::{
  Arg, Async, BinOpKind, Block, Decl, Expr, ExprKind, Ext, Fun, Lit, LitKind,
  Mutability, Pattern, PatternKind, Program, Prototype, Public, ReturnTy, Stmt,
  StmtKind, Ty, TyKind, UnOpKind, Unit, Unsafe, Wasm,
};

use std::fmt;

pub struct Sep<'a, T: 'a>(pub &'a [T], pub &'a str);

impl<'a, T: fmt::Display> fmt::Display for Sep<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let nodes = self
      .0
      .iter()
      .map(|node| node.to_string())
      .collect::<Vec<String>>()
      .join(self.1);

    write!(f, "{nodes}")
  }
}

impl fmt::Display for Public {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Yes(_) => write!(f, "pub"),
      Self::No => write!(f, ""),
    }
  }
}

impl fmt::Display for Async {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Yes(_) => write!(f, "async"),
      Self::No => write!(f, ""),
    }
  }
}

impl fmt::Display for Unsafe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Yes(_) => write!(f, "unsafe"),
      Self::No => write!(f, ""),
    }
  }
}

impl fmt::Display for Wasm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Yes(_) => write!(f, "wasm"),
      Self::No => write!(f, ""),
    }
  }
}

impl fmt::Display for Mutability {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Yes(_) => write!(f, "mut"),
      Self::No => write!(f, ""),
    }
  }
}

impl fmt::Display for Pattern {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.kind)
  }
}

impl fmt::Display for PatternKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Underscore => write!(f, "_"),
      Self::Identifier(name) => write!(f, "{name}"),
      Self::Lit(lit) => write!(f, "{lit}"),
      Self::MeLower => write!(f, "me"),
    }
  }
}

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", Sep(&self.stmts, "\n"))
  }
}

impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.kind)
  }
}

impl fmt::Display for StmtKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Ext(ext) => write!(f, "{ext}"),
      Self::Val(decl) => write!(f, "{decl}"),
      Self::Fun(fun) => write!(f, "{fun}"),
      _ => panic!(),
    }
  }
}

impl fmt::Display for Ext {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} ext {}", self.public, self.prototype)?;

    let Some(body) = &self.body else { return write!(f, ";"); };

    write!(f, " {body}")
  }
}

impl fmt::Display for Decl {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.pattern).ok();

    if let Some(ty) = &self.ty {
      write!(f, ": {ty}").ok();
    } else {
      write!(f, " :=").ok();
    }

    write!(f, "{};", self.value)
  }
}

impl fmt::Display for Fun {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self.public {
      Public::Yes(_) => write!(f, "pub ")?,
      Public::No => write!(f, "")?,
    };

    match &self.asyncness {
      Async::Yes(_) => write!(f, "async ").ok(),
      Async::No => write!(f, "").ok(),
    };

    match &self.unsafeness {
      Unsafe::Yes(_) => write!(f, "unsafe ").ok(),
      Unsafe::No => write!(f, "").ok(),
    };

    match &self.wasm {
      Wasm::Yes(_) => write!(f, "wasm ").ok(),
      Wasm::No => write!(f, "").ok(),
    };

    write!(f, "fun {} {}", self.prototype, self.body)
  }
}

impl fmt::Display for Prototype {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} ({}) {}",
      self.name,
      Sep(&self.inputs, ", "),
      self.output
    )
  }
}

impl fmt::Display for Arg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.pattern, self.ty)
  }
}

impl fmt::Display for ReturnTy {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Ty(ty) => write!(f, ": {ty}"),
      Self::Default(_) => write!(f, ""),
    }
  }
}

impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.exprs.is_empty() {
      write!(f, "{{}}")
    } else {
      write!(f, "{{\n{}\n}}", Sep(&self.exprs, "\n"))
    }
  }
}

impl fmt::Display for Unit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let binds = Sep(&self.binds, "\n");

    let mocks = self
      .mocks
      .iter()
      .map(|mock| format!("mock {} {}", mock.prototype, mock.body))
      .collect::<Vec<_>>()
      .join("\n");

    let tests = self
      .tests
      .iter()
      .map(|test| format!("test {} {}", test.prototype, test.body))
      .collect::<Vec<_>>()
      .join("\n");

    write!(f, "unit {{\n{} {} {}\n}}", binds, mocks, tests)
  }
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.kind)
  }
}

impl fmt::Display for ExprKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Lit(lit) => write!(f, "{lit}"),
      Self::Identifier(identifier) => write!(f, "{identifier}"),
      Self::Call(callee, args) => write!(f, "{callee}({})", Sep(args, ", ")),
      Self::UnOp(op, rhs) => write!(f, "{}({})", op.node, rhs),
      Self::BinOp(lhs, op, rhs) => write!(f, "({lhs} {op} {rhs})"),
      Self::Assign(lhs, op, rhs) => write!(f, "{lhs} {op} {rhs}"),
      Self::AssignOp(lhs, op, rhs) => write!(f, "{lhs} {op} {rhs}"),
      Self::Block(body) => write!(f, "{body}"),
      Self::Loop(body) => write!(f, "for {body}"),
      Self::While(condition, body) => write!(f, "while {condition} {body}"),
      Self::Until(condition, body) => write!(f, "until {condition} {body}"),
      Self::Return(maybe_expr) => {
        let Some(expr) = maybe_expr else { return write!(f, "return;"); };

        write!(f, "return {expr};")
      }
      Self::Break(maybe_expr) => {
        let Some(expr) = maybe_expr else { return write!(f, "break;"); };

        write!(f, "break {expr};")
      }
      Self::Continue => write!(f, "continue"),
      Self::When(condition, consequence, alternative) => {
        write!(f, "when {condition} ? {consequence} : {alternative}")
      }
      Self::IfElse(condition, consequence, maybe_alternative) => {
        write!(f, "if {condition} {consequence}")?;

        let Some(alternative) = maybe_alternative else { return write!(f, ""); };

        write!(f, " {alternative}")
      }
      Self::Lambda(args, expr) => {
        write!(f, "fn({}) -> {}", Sep(args, ", "), expr)
      }
      Self::Array(element) => write!(f, "[{}]", Sep(element, ", ")),
      Self::ArrayAccess(indexed, index) => write!(f, "{}[{}]", indexed, index),
      Self::Tuple(element) => write!(f, "({})", Sep(element, ", ")),
      Self::TupleAccess(tuple, access) => write!(f, "{tuple}.{access}"),
      Self::Stmt(stmt) => write!(f, "{stmt}"),
      Self::Decl(decl) => write!(f, "{decl}"),
    }
  }
}

impl fmt::Display for Lit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.kind)
  }
}

impl fmt::Display for LitKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Bool(boolean) => write!(f, "{boolean}"),
      Self::Int(int) => write!(f, "{int}"),
      Self::Real(real) => write!(f, "{real}"),
      Self::Str(string) => write!(f, "{string}"),
    }
  }
}

impl fmt::Display for BinOpKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Add => write!(f, "+"),
      Self::Sub => write!(f, "-"),
      Self::Mul => write!(f, "*"),
      Self::Div => write!(f, "/"),
      Self::Rem => write!(f, "%"),
      Self::And => write!(f, "&&"),
      Self::Or => write!(f, "||"),
      Self::Lt => write!(f, "<"),
      Self::Gt => write!(f, ">"),
      Self::Le => write!(f, "<="),
      Self::Ge => write!(f, ">="),
      Self::Eq => write!(f, "=="),
      Self::Ne => write!(f, "!="),
      Self::Shl => write!(f, "<<"),
      Self::Shr => write!(f, ">>"),
      Self::BitAnd => write!(f, "&"),
      Self::BitOr => write!(f, "|"),
      Self::BitXor => write!(f, "^"),
      Self::Range => write!(f, ".."),
      Self::As => write!(f, "as"),
    }
  }
}

impl fmt::Display for UnOpKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Neg => write!(f, "-"),
      Self::Not => write!(f, "!"),
    }
  }
}

impl fmt::Display for Ty {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.kind)
  }
}

impl fmt::Display for TyKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Void => write!(f, "void"),
      Self::Bool => write!(f, "bool"),
      Self::Int => write!(f, "int"),
      Self::Real => write!(f, "real"),
      Self::Str => write!(f, "str"),
      Self::Infer => write!(f, "infer"),
      Self::Fn(args, ty) => write!(f, "Fn({}): {ty}", Sep(args, ", ")),
      Self::Array(indexed, maybe_size) => {
        write!(f, "{indexed}")?;

        let Some(size) = maybe_size else { return write!(f, "[]"); };

        write!(f, "[{size}]")
      }
      Self::Tuple(tys) => write!(f, "({})", Sep(tys, ", ")),
    }
  }
}
