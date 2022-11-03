use super::constant::GCC_PROGRAM;

use lazy_static::lazy_static;
use slowprint::slow_println;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

lazy_static! {
  pub static ref INTERVAL_ARU: Duration = std::time::Duration::from_millis(5);
  pub static ref INTERVAL_DIR: Duration = std::time::Duration::from_millis(10);
  pub static ref INTERVAL_OBJ: Duration = std::time::Duration::from_millis(15);
  pub static ref INTERVAL_EXE: Duration = std::time::Duration::from_millis(20);
  pub static ref INTERVAL_ARD: Duration = std::time::Duration::from_millis(5);
}

/// create a directory and display the directory path
pub fn make_dir(path_directory: &str) {
  slow_println("\n╭", *INTERVAL_ARU);
  if is_dir_exist(path_directory) {
    return slow_println(
      &format!("│ [make] dir: `{path_directory}`"),
      *INTERVAL_DIR,
    );
  }

  match fs::create_dir(path_directory) {
    Ok(_) => {
      slow_println(&format!("│ [make] dir: `{path_directory}`"), *INTERVAL_DIR)
    }
    Err(error) => panic!("error: {error}"),
  }
}

/// create a file and display the file path
pub fn make_file(path_file: &str, bytes_buf: &[u8]) {
  match File::create(path_file) {
    Ok(mut file) => match file.write_all(bytes_buf) {
      Ok(_) => {
        slow_println(&format!("⋮ [make] obj: `{path_file}`"), *INTERVAL_OBJ)
      }
      Err(error) => panic!("error: {error}"),
    },
    Err(error) => panic!("error: {error}"),
  }
}

/// create an executable and print the output path
pub fn make_exe(path_input: &str, path_output: &str) {
  match Command::new(GCC_PROGRAM)
    .args([path_input, "-o", path_output])
    .output()
  {
    Ok(_) => {
      slow_println(&format!("│ [make] exe: `{path_output}`"), *INTERVAL_EXE);
      slow_println("╰\n", *INTERVAL_ARD);
    }
    Err(error) => panic!("error: {error}"),
  }
}

/// create an executable with link (for linux and windows) and print the output path
#[cfg(not(target_os = "macos"))]
pub fn make_exe_with_link(
  path_input: &str,
  path_link: &str,
  path_output: &str,
) {
  // fixme: `ld: warning: PIE disabled. Absolute addressing (perhaps -mdynamic-no-pic) not allowed in code signed PIE`
  match Command::new(GCC_PROGRAM)
    .args([
      "-v",
      "-fno-pie",
      "-pthread",
      "-ldl",
      "-Wl",
      "no-as-needed",
      path_input,
      path_link,
      "-o",
      path_output,
    ])
    .output()
  {
    Ok(_) => {
      slow_println(&format!("│ [make] exe: `{path_output}`"), *INTERVAL_EXE);
      slow_println(&format!("╰\n"), *INTERVAL_ARD);
    }
    Err(error) => panic!("error: {error}"),
  }
}

/// create an executable with link (for macos) and print the output path
#[cfg(target_os = "macos")]
pub fn make_exe_with_link(
  path_input: &str,
  path_link: &str,
  path_output: &str,
) {
  // fixme: `ld: warning: PIE disabled. Absolute addressing (perhaps -mdynamic-no-pic) not allowed in code signed PIE`
  match Command::new(GCC_PROGRAM)
    .args([
      "-v",
      "-fno-pie",
      "-pthread",
      "-ldl",
      path_input,
      path_link,
      "-o",
      path_output,
    ])
    .output()
  {
    Ok(_) => {
      slow_println(&format!("│ [make] exe: `{path_output}`"), *INTERVAL_EXE);
      slow_println("╰\n", *INTERVAL_ARD);
    }
    Err(error) => panic!("error: {error}"),
  }
}

/// check the existence of a directory
fn is_dir_exist(path: &str) -> bool {
  Path::new(path).is_dir()
}
