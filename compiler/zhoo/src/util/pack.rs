use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn make_dir(path_directory: &str) {
  println!("\n╭");
  if is_dir_exist(path_directory) {
    return println!("│ [make] dir: `{path_directory}`");
  }

  match fs::create_dir(path_directory) {
    Ok(_) => println!("│ [make] dir: `{path_directory}`"),
    Err(error) => panic!("ERROR: {error}"),
  }
}

pub fn make_file(path_file: &str, bytes_buf: &[u8]) {
  match File::create(path_file) {
    Ok(mut file) => match file.write_all(bytes_buf) {
      Ok(_) => println!("⋮ [make] obj: `{path_file}`"),
      Err(error) => panic!("ERROR: {error}"),
    },
    Err(error) => panic!("ERROR: {error}"),
  }
}

pub fn make_exe(path_input: &str, path_output: &str) {
  match Command::new("gcc")
    .args([path_input, "-o", path_output])
    .output()
  {
    Ok(_) => {
      println!("│ [make] exe: `{path_output}`",);
      println!("╰");
    }
    Err(error) => panic!("ERROR: {error}"),
  }
}

pub fn make_exe_with_link(
  path_input: &str,
  path_link: &str,
  path_output: &str,
) {
  // fixme: `ld: warning: PIE disabled. Absolute addressing (perhaps -mdynamic-no-pic) not allowed in code signed PIE`
  match Command::new("gcc")
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
      println!("│ [make] exe: `{path_output}`",);
      println!("╰");
    }
    Err(error) => panic!("ERROR: {error}"),
  }
}

fn is_dir_exist(path: &str) -> bool {
  Path::new(path).is_dir()
}
