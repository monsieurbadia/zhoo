use super::icon::Icon;

#[derive(Debug)]
pub enum Message {
  Stop,
  Next(Icon, String),
  WithFrame(char),
  WithText(String),
}
