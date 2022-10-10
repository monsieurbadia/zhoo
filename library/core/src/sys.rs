use super::util::to_str;

use std::fs::File;

#[no_mangle]
fn exit(code: isize) {
  std::process::exit(code as i32)
}

#[no_mangle]
fn create(path: *const i8, source: *const i8) {
  use std::io::Write;

  let path = to_str(path);
  let source = to_str(source);

  let mut file = match File::create(path) {
    Ok(f) => f,
    Err(error) => panic!("{error}"),
  };

  match file.write_all(source.as_bytes()) {
    Ok(_) => {}
    Err(error) => panic!("{error}"),
  }
}

#[no_mangle]
fn open(path: *const i8) -> String {
  use std::io::Read;

  let path = to_str(path);

  let mut file = match File::open(path) {
    Ok(f) => f,
    Err(error) => panic!("{error}"),
  };

  let mut contents = String::new();

  match file.read_to_string(&mut contents) {
    Ok(_) => contents,
    Err(error) => panic!("{error}"),
  }
}
