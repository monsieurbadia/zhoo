pub struct Color;

impl Color {
  /// FRENCH_SKY_BLUE: blue tint
  pub const BLUE_100: ariadne::Color = ariadne::Color::RGB(112, 161, 255);

  // SPIRO_DISCO_BALL: blue tint
  pub const BLUE_200: ariadne::Color = ariadne::Color::RGB(15, 188, 249);

  /// WATERFALL: green tint
  pub const GREEN_100: ariadne::Color = ariadne::Color::RGB(56, 173, 169);

  // ANSI Color 115: green tint
  pub const GREEN_200: ariadne::Color = ariadne::Color::Fixed(115);

  /// CARMINE_PINK: red tint
  pub const RED_100: ariadne::Color = ariadne::Color::RGB(232, 65, 24);

  /// BEEKEEPER: yellow tint
  pub const YELLOW_100: ariadne::Color = ariadne::Color::RGB(246, 229, 141);

  pub const fn error() -> ariadne::Color {
    Self::RED_100
  }

  pub const fn help() -> ariadne::Color {
    Self::YELLOW_100
  }

  pub const fn hint() -> ariadne::Color {
    Self::GREEN_100
  }

  pub const fn note() -> ariadne::Color {
    Self::BLUE_200
  }

  pub const fn title() -> ariadne::Color {
    Self::BLUE_100
  }

  pub const fn warning() -> ariadne::Color {
    Self::YELLOW_100
  }
}
