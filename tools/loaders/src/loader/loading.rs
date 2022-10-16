use super::icon::Icon;
use super::message::Message;

use crate::spin::{Frame, Spinner};

use flume::Sender;
use pollster::block_on;

use std::io::Write;
use std::time::Duration;
use std::{io, thread};

const INTERVAL: u64 = 80;
const EOF: char = '\0';

pub fn loading(spinner: Spinner) -> Loading {
  Loading::new(spinner, None)
}

pub fn loading_with_writer<W: Write + Send + 'static>(
  spinner: Spinner,
  writer: W,
) -> Loading {
  Loading::with_writer(spinner, writer)
}

#[derive(Debug)]
pub struct Loading {
  sender: Sender<Message>,
}

impl Loading {
  pub fn new(spinner: Spinner, writer: Option<Box<dyn Write + Send>>) -> Self {
    let (sender, receiver) = flume::unbounded::<Message>();
    let csender = sender.clone();

    thread::spawn(move || {
      pollster::block_on(async move {
        let mut spinner_frame = EOF;
        let mut current_text = String::new();

        let mut writer = match writer {
          Some(w) => w,
          None => Box::new(io::stdout()), // default
        };

        while let Ok(message) = receiver.recv_async().await {
          match message {
            Message::WithFrame(character) => {
              write(&mut writer, format!("{} {}", spinner_frame, current_text))
                .await
                .unwrap();

              spinner_frame = character;
            }
            Message::WithText(content) => {
              write(&mut writer, format!("{}, {}", spinner_frame, content))
                .await
                .unwrap();

              current_text = content;
            }
            Message::Next(icon, content) => {
              write(&mut writer, format!("{} {}\n", icon, content))
                .await
                .unwrap();
            }
            Message::Stop => {
              write(&mut writer, EOF).await.unwrap();
              break;
            }
          }
        }
      })
    });

    thread::spawn(move || {
      pollster::block_on(async move {
        let mut frame = Frame::new(spinner);

        while sender
          .send_async(Message::WithFrame(frame.next().await))
          .await
          .is_ok()
        {
          thread::sleep(Duration::from_millis(INTERVAL));
        }
      });
    });

    Self { sender: csender }
  }

  pub fn with_writer<W: Write + Send + 'static>(
    spinner: Spinner,
    writer: W,
  ) -> Self {
    Self::new(spinner, Some(Box::new(writer)))
  }

  pub fn with_error<T: Into<String>>(&self, content: T) {
    block_on(send_with_error(&self.sender, content));
  }

  pub fn with_icon<T: Into<String>>(&self, icon: T, content: T) {
    block_on(send_with_icon(&self.sender, icon, content));
  }

  pub fn with_info<T: Into<String>>(&self, content: T) {
    block_on(send_with_info(&self.sender, content));
  }

  pub fn send_with_success<T: Into<String>>(&self, content: T) {
    block_on(send_with_success(&self.sender, content));
  }

  pub fn with_text<T: Into<String>>(&self, content: T) {
    block_on(send_with_text(&self.sender, content));
  }

  pub fn with_time<T: Into<String>>(&self, content: T) {
    block_on(send_with_time(&self.sender, content));
  }

  pub fn with_warning<T: Into<String>>(&self, content: T) {
    block_on(send_with_warning(&self.sender, content));
  }

  pub fn stop(self) {
    block_on(send_stop(&self.sender))
  }
}

async fn write<T: Into<String>, W: Write + Send>(
  mut writer: W,
  contents: T,
) -> io::Result<()> {
  write!(writer, "\x1B[2K\x1B[0G")?;
  write!(writer, "{}", contents.into())?;
  writer.flush()
}

async fn send_with_error<T: Into<String>>(
  sender: &Sender<Message>,
  content: T,
) {
  sender
    .send_async(Message::Next(Icon::Error, content.into()))
    .await
    .expect("event to have been sent")
}

async fn send_with_icon<T: Into<String>>(
  sender: &Sender<Message>,
  icon: T,
  content: T,
) {
  sender
    .send_async(Message::Next(Icon::Custom(icon.into()), content.into()))
    .await
    .expect("event to have been sent")
}

async fn send_with_info<T: Into<String>>(sender: &Sender<Message>, content: T) {
  sender
    .send_async(Message::Next(Icon::Info, content.into()))
    .await
    .expect("event to have been sent")
}

async fn send_with_success<T: Into<String>>(
  sender: &Sender<Message>,
  content: T,
) {
  sender
    .send_async(Message::Next(Icon::Success, content.into()))
    .await
    .expect("event to have been sent")
}

async fn send_with_text<T: Into<String>>(sender: &Sender<Message>, content: T) {
  sender
    .send_async(Message::WithText(content.into()))
    .await
    .expect("event to have been sent");
}

async fn send_with_time<T: Into<String>>(sender: &Sender<Message>, content: T) {
  sender
    .send_async(Message::Next(Icon::Time, content.into()))
    .await
    .expect("event to have been sent")
}

async fn send_with_warning<T: Into<String>>(
  sender: &Sender<Message>,
  content: T,
) {
  sender
    .send_async(Message::Next(Icon::Warning, content.into()))
    .await
    .expect("event to have been sent")
}

async fn send_stop(sender: &Sender<Message>) {
  sender
    .send_async(Message::Stop)
    .await
    .expect("event to have been sent");

  thread::sleep(Duration::from_millis(INTERVAL));
}
