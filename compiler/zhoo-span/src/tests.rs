use super::source::SourceMap;
use super::span::Span;

use std::fs;
use std::path::Path;

const PATH: &str = "../../samples/playground/basic.zo";

#[test]
fn test_merge_span() {
  let span_a = Span::new(0, 3);
  let span_b = Span::new(3, 6);
  let span_actual = Span::merge(&span_a, &span_b);
  let span_expected = Span::new(0, 6);

  assert_eq!(span_actual, span_expected);
}

#[test]
fn test_add_source_file() {
  let mut source_map = SourceMap::default();
  let path_buf = Path::new(PATH).to_path_buf();

  let source_id = match source_map.add(path_buf) {
    Ok(id) => id,
    Err(error) => panic!("{error}"),
  };

  assert_eq!(source_id, 0);
}

#[test]
fn test_get_source_code() {
  let mut source_map = SourceMap::default();
  let path_buf = Path::new(PATH).to_path_buf();

  let source_id = match source_map.add(path_buf) {
    Ok(id) => id,
    Err(error) => panic!("{error}"),
  };

  let source_code = source_map.code(source_id);

  let code = match fs::read_to_string(PATH) {
    Ok(f) => f,
    Err(error) => panic!("{error}"),
  };

  assert_eq!(source_code, code);
}

#[test]
fn test_get_source_id() {
  let mut source_map = SourceMap::default();
  let path_buf = Path::new(PATH).to_path_buf();
  let _ = source_map.add(path_buf);
  let source_id = source_map.source_id(Span::ZERO);

  assert_eq!(source_id, 0);
}

#[test]
fn test_get_source_path() {
  let mut source_map = SourceMap::default();
  let path = Path::new(PATH);
  let path_buf = path.to_path_buf();
  let _ = source_map.add(path_buf);
  let source_path = source_map.path(Span::ZERO);

  assert_eq!(source_path, path);
}
