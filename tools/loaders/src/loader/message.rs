use super::icon::Icon;

/// an message enumeration
#[derive(Debug)]
pub(crate) enum Message {
  Stop,
  Next(Icon, String),
  WithFrame(char),
  WithText(String),
}
