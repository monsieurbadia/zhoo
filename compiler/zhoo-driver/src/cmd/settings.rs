pub mod compile;

/// a backend enumeration
#[derive(Debug)]
pub enum Backend {
  Cranelift,
}

impl From<String> for Backend {
  fn from(backend: String) -> Self {
    match backend.as_str() {
      "cranelift" => Self::Cranelift,
      _ => panic!("wrong backend, expected: [cranelift]"),
    }
  }
}
