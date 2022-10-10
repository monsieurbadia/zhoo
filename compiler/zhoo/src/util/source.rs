use super::span::Span;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Source {
  pub id: usize,
  pub path: PathBuf,
}

impl Source {
  pub fn new(id: usize, path: PathBuf) -> Self {
    Self { id, path }
  }
}

#[derive(Clone, Debug, Default)]
pub struct SourceMap {
  pub code: String,
  pub sources: Vec<Box<Source>>,
}

impl SourceMap {
  pub fn add(&mut self, path: PathBuf) -> io::Result<u32> {
    let source_id = self.sources.len() as u32;
    let offset = self.code.len();
    let mut f = File::open(&path)?;

    f.read_to_string(&mut self.code)?;
    self.sources.push(Box::new(Source::new(offset, path)));

    Ok(source_id)
  }

  pub fn code(&self, source_id: u32) -> &str {
    let source_id = source_id as usize;

    let end = self
      .sources
      .get(source_id + 1)
      .map(|s| s.id)
      .unwrap_or(self.code.len());

    &self.code[self.sources[source_id].id..end]
  }

  pub fn source(&self, span: Span) -> u32 {
    self
      .sources
      .iter()
      .enumerate()
      .find(|(_, s)| s.id > span.lo as usize)
      .map(|(i, _)| i - 1)
      .unwrap_or(self.sources.len() - 1) as u32
  }

  pub fn path(&self, span: Span) -> &Path {
    &self.sources[self.source(span) as usize].path
  }
}
