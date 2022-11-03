use zhoo_ast::ast::Ty;
use zhoo_ast::ptr::Fsp;

use fxhash::FxHashMap;

#[derive(Clone, Debug, Default)]
struct Scope {
  decls: FxHashMap<String, Fsp<Ty>>,
  funs: FxHashMap<String, (Vec<Fsp<Ty>>, Fsp<Ty>)>,
}

impl Scope {
  fn decl(&self, name: &str) -> Option<&Fsp<Ty>> {
    self.decls.get(name)
  }

  fn fun(&self, name: &str) -> Option<&(Vec<Fsp<Ty>>, Fsp<Ty>)> {
    self.funs.get(name)
  }

  fn remove_decl(&mut self, name: &str) -> Option<Fsp<Ty>> {
    self.decls.remove(name)
  }

  fn set_decl(&mut self, name: String, ty: Fsp<Ty>) -> Result<(), String> {
    match self.decls.get(&name) {
      Some(_) => Err(format!("variable `{name}` already exists")),
      None => {
        self.decls.insert(name, ty);
        Ok(())
      }
    }
  }

  fn set_fun(
    &mut self,
    name: String,
    ty: (Vec<Fsp<Ty>>, Fsp<Ty>),
  ) -> Result<(), String> {
    match self.funs.get(&name) {
      Some(_) => Err(format!("function `{name}` already exists")),
      None => {
        self.funs.insert(name, ty);
        Ok(())
      }
    }
  }
}

#[derive(Clone, Debug)]
pub(crate) struct ScopeMap {
  maps: Vec<Scope>,
}

impl ScopeMap {
  pub fn enter_scope(&mut self) {
    self.maps.push(Scope::default());
  }

  pub fn exit_scope(&mut self) {
    if self.maps.len() > 1 {
      self.maps.pop();
    }
  }

  pub fn decl(&self, name: &str) -> Option<&Fsp<Ty>> {
    for map in self.maps.iter().rev() {
      if let Some(decl) = map.decl(name) {
        return Some(decl);
      }
    }

    None
  }

  pub fn fun(&self, name: &str) -> Option<&(Vec<Fsp<Ty>>, Fsp<Ty>)> {
    for map in self.maps.iter().rev() {
      if let Some(fun) = map.fun(name) {
        return Some(fun);
      }
    }

    None
  }

  pub fn remove_decl(&mut self, name: &str) -> Option<Fsp<Ty>> {
    for map in self.maps.iter_mut().rev() {
      if let Some(decl) = map.remove_decl(name) {
        return Some(decl);
      }
    }

    None
  }

  pub fn set_decl(&mut self, name: String, ty: Fsp<Ty>) -> Result<(), String> {
    match self.maps.last_mut() {
      Some(map) => map.set_decl(name, ty),
      None => Err(format!("variable `{name}` value do not exist")),
    }
  }

  pub fn set_fun(
    &mut self,
    name: String,
    ty: (Vec<Fsp<Ty>>, Fsp<Ty>),
  ) -> Result<(), String> {
    match self.maps.last_mut() {
      Some(map) => map.set_fun(name, ty),
      None => Err(format!("function `{name}` value do not exist")),
    }
  }
}

impl Default for ScopeMap {
  fn default() -> Self {
    Self {
      maps: vec![Scope::default()],
    }
  }
}
