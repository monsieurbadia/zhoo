//! this module is used to colorize for the report error messages

/// the color instance
pub(crate) struct Color;

impl Color {
  /// FRENCH_SKY_BLUE: blue tint
  const BLUE_100: ariadne::Color = ariadne::Color::RGB(112, 161, 255);

  // SPIRO_DISCO_BALL: blue tint
  const BLUE_200: ariadne::Color = ariadne::Color::RGB(15, 188, 249);

  /// WATERFALL: green tint
  const GREEN_100: ariadne::Color = ariadne::Color::RGB(56, 173, 169);

  /// CARMINE_PINK: red tint
  const RED_100: ariadne::Color = ariadne::Color::RGB(232, 65, 24);

  /// BEEKEEPER: yellow tint
  const YELLOW_100: ariadne::Color = ariadne::Color::RGB(246, 229, 141);

  /// the color for an `error` case
  pub const fn error() -> ariadne::Color {
    Self::RED_100
  }

  /// the color for an `help` case
  pub const fn help() -> ariadne::Color {
    Self::YELLOW_100
  }

  /// the color for an `hint` case
  pub const fn hint() -> ariadne::Color {
    Self::GREEN_100
  }

  /// the color for an `note` case
  pub const fn note() -> ariadne::Color {
    Self::BLUE_200
  }

  /// the color for an `title` case
  pub const fn title() -> ariadne::Color {
    Self::BLUE_100
  }

  /// the color for an `warning` case
  pub const fn warning() -> ariadne::Color {
    Self::YELLOW_100
  }
}
