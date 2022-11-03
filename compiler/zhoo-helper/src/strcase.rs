use inflector::cases::pascalcase;
use inflector::cases::screamingsnakecase;
use inflector::cases::snakecase;
use inflector::string::pluralize;
use inflector::string::singularize;

use std::fmt;

/// the pascal case name
const PASCAL_CASE: &str = "pascal case";

/// the snake case name
const SNAKE_CASE: &str = "snake case";

/// the screaming snake case name
const SNAKE_SCREAMING_CASE: &str = "screaming snake case";

pub enum StrCase {
  Pascal,
  Snake,
  SnakeScreaming,
}

impl fmt::Display for StrCase {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Pascal => write!(f, "{PASCAL_CASE}"),
      Self::Snake => write!(f, "{SNAKE_CASE}"),
      Self::SnakeScreaming => write!(f, "{SNAKE_SCREAMING_CASE}"),
    }
  }
}

pub fn is_pascal_case<T: AsRef<str>>(text: T) -> bool {
  pascalcase::is_pascal_case(text.as_ref())
}

pub fn to_pascal_case<T: AsRef<str>>(text: T) -> String {
  pascalcase::to_pascal_case(text.as_ref())
}

pub fn is_snake_case<T: AsRef<str>>(text: T) -> bool {
  snakecase::is_snake_case(text.as_ref())
}

pub fn to_snake_case<T: AsRef<str>>(text: T) -> String {
  snakecase::to_snake_case(text.as_ref())
}

pub fn is_snake_screaming_case<T: AsRef<str>>(text: T) -> bool {
  screamingsnakecase::is_screaming_snake_case(text.as_ref())
}

pub fn to_snake_screaming_case<T: AsRef<str>>(text: T) -> String {
  screamingsnakecase::to_screaming_snake_case(text.as_ref())
}

fn to_singular<T: AsRef<str>>(text: T) -> String {
  singularize::to_singular(text.as_ref())
}

fn to_plural<T: AsRef<str>>(text: T) -> String {
  pluralize::to_plural(text.as_ref())
}

pub fn to_plural_or_singular<T: AsRef<str>>(
  number_of_words: usize,
  text: T,
) -> String {
  if number_of_words > 1 {
    return to_plural(text.as_ref());
  }

  to_singular(text.as_ref())
}
