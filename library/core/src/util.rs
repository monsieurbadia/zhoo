use std::ffi;

#[inline]
pub fn to_str<'a>(string: *const i8) -> &'a str {
  let cstr = unsafe { ffi::CStr::from_ptr(string) };

  // String::from_utf8_lossy(cstr.to_bytes_with_nul()).to_string()

  match cstr.to_str() {
    Ok(str_ref) => str_ref,
    Err(error) => panic!("{error}"),
  }
}
