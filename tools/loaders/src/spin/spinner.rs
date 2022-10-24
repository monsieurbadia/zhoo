use fnv::FnvHashMap;
use lazy_static::lazy_static;

pub type Spinners = Vec<char>;

/// the default index spinner
pub const SPINNER_INDEX: usize = 0;

lazy_static! {
  /// a dictionary of spiders
  pub static ref SPINNERS: FnvHashMap<Spinner, Spinners> = {
    let mut spinners = FnvHashMap::default();

    spinners.insert(Spinner::Arc, Spinner::Arc.to_vec());
    spinners.insert(Spinner::Arrow, Spinner::Arrow.to_vec());
    spinners.insert(Spinner::Card, Spinner::Card.to_vec());
    spinners.insert(Spinner::Moon, Spinner::Moon.to_vec());
    spinners.insert(Spinner::Trigram, Spinner::Trigram.to_vec());

    spinners
  };
}

/// an spinner enumeration
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Spinner {
  Arc,
  Arrow,
  Card,
  Moon,
  Trigram,
}

impl Spinner {
  /// conversion of a spinner to a vector of frames
  pub fn to_vec(&self) -> Spinners {
    match self {
      Self::Arc => vec!['◜', '◠', '◝', '◞', '◡', '◟'],
      Self::Arrow => vec!['↑', '→', '↓', '←'],
      Self::Card => vec!['♥', '♦', '♣', '♠'],
      Self::Moon => vec!['●', '◐', '◑', '◒', '◓'],
      Self::Trigram => vec!['☰', '☱', '☲', '☴', '☷'],
    }
  }
}
