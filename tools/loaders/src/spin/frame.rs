pub use super::spinner::{Spinner, Spinners, SPINNERS, SPINNER_INDEX};

/// an instance of a frame
#[derive(Debug, Clone)]
pub struct Frame {
  index: usize,
  frames: Spinners,
}

impl Frame {
  /// create an instance of a frame
  pub fn new(spinner: Spinner) -> Self {
    Self::new_with_frames(
      SPINNERS
        .get(&spinner)
        .unwrap_or(&Spinner::Arc.to_vec())
        .to_vec(),
    )
  }

  /// create an instance of a frame
  fn new_with_frames(frames: Spinners) -> Self {
    Self {
      index: SPINNER_INDEX,
      frames,
    }
  }

  /// get the next character frame
  pub async fn next(&mut self) -> char {
    match self.frames.get(self.index) {
      Some(character) => {
        self.index += 1;
        *character
      }
      None => {
        self.index = 1;
        self.frames[0]
      }
    }
  }
}
