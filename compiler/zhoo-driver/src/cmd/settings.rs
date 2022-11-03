pub mod compile;

#[derive(Debug)]
pub(crate) enum Backend {
  Cranelift,
  Llvm,
}

impl From<&String> for Backend {
  fn from(backend: &String) -> Self {
    match backend.as_str() {
      "llvm" => Self::Llvm,
      "cranelift" => Self::Cranelift,
      _ => panic!("wrong backend, expected: [cranelift]"),
    }
  }
}
