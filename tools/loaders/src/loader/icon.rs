use std::fmt;

/// an icon enumeration
#[derive(Debug)]
pub(crate) enum Icon {
  /// an icon variant for error
  Error,

  /// an icon variant for info
  Info,

  /// an icon variant for success
  Success,

  /// an icon variant for time
  Time,

  /// an icon variant for warning
  Warning,

  /// an icon variant for custom
  Custom(String),
}

impl fmt::Display for Icon {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Error => write!(f, "\x1B[31m✖\x1B[0m"),
      Self::Info => write!(f, "\x1B[34mℹ\x1B[0m"),
      Self::Success => write!(f, "\x1B[32m✔\x1B[0m"),
      Self::Time => write!(f, "\x1B[33m⚡\x1B[0m"),
      Self::Warning => write!(f, "\x1B[33m⚠\x1B[0m"),
      Self::Custom(icon) => write!(f, "\x1B[33m{}\x1B[0m", icon),
    }
  }
}
