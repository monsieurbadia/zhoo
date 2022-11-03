use super::icon::Icon;

#[derive(Debug)]
pub(crate) enum Message {
  Stop,
  Next(Icon, String),
  WithFrame(char),
  WithText(String),
}
