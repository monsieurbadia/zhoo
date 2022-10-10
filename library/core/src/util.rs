use std::ffi;

#[inline]
pub fn to_str<'a>(string: *const i8) -> &'a str {
  let c_str = unsafe { ffi::CStr::from_ptr(string) };

  match c_str.to_str() {
    Ok(r_str) => r_str,
    Err(e) => panic!("{e}"),
  }
}
