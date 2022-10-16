use super::util;

#[no_mangle]
fn print(ptr: *const i8) {
  let string = util::to_str(ptr);

  print!("{string}");
}

#[no_mangle]
fn println(ptr: *const i8) {
  let string = util::to_str(ptr);

  println!("{string}");
}

#[no_mangle]
fn printi(num: isize) {
  print!("{num}");
}

#[no_mangle]
fn printiln(num: isize) {
  println!("{num}");
}
