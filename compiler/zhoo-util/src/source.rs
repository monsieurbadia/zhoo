use super::span::Span;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

/// an instance of a source file
#[derive(Clone, Debug)]
pub struct Source {
  pub id: usize,
  pub path: PathBuf,
}

impl Source {
  /// create a instance of a source file
  pub const fn new(id: usize, path: PathBuf) -> Self {
    Self { id, path }
  }
}

/// an instance of a source map
///
/// it contains all the source of a program
#[derive(Clone, Debug, Default)]
pub struct SourceMap {
  pub code: String,
  pub sources: Vec<Box<Source>>,
}

impl SourceMap {
  /// add a source from a path
  pub fn add(&mut self, path: PathBuf) -> io::Result<u32> {
    let source_id = self.sources.len() as u32;
    let offset = self.code.len();
    let mut file = File::open(&path)?;

    file.read_to_string(&mut self.code)?;
    self.sources.push(Box::new(Source::new(offset, path)));

    Ok(source_id)
  }

  /// get the source code from the source id
  pub fn code(&self, source_id: u32) -> &str {
    let source_id = source_id as usize;

    let end = self
      .sources
      .get(source_id + 1)
      .map(|s| s.id)
      .unwrap_or(self.code.len());

    &self.code[self.sources[source_id].id..end]
  }

  /// get the source id from the span
  pub fn source_id(&self, span: Span) -> u32 {
    self
      .sources
      .iter()
      .enumerate()
      .find(|(_, s)| s.id > span.lo as usize)
      .map(|(i, _)| i - 1)
      .unwrap_or(self.sources.len() - 1) as u32
  }

  /// get the path from the span
  pub fn path(&self, span: Span) -> &Path {
    &self.sources[self.source_id(span) as usize].path
  }
}
