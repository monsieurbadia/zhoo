use super::icon::Icon;
use super::message::Message;

use crate::spin::{Frame, Spinner};

use flume::Sender;
use pollster::block_on;

use std::io::Write;
use std::time::Duration;
use std::{io, thread};

/// the sleep duration
const INTERVAL: u64 = 80;

/// the end of file character
const EOF: char = '\0';

/// create the loading instance
pub fn loading(spinner: Spinner) -> Loading {
  Loading::new(spinner, None)
}

/// create the loading instance with writer
pub fn loading_with_writer<W: Write + Send + 'static>(
  spinner: Spinner,
  writer: W,
) -> Loading {
  Loading::with_writer(spinner, writer)
}

/// an instance of loading
#[derive(Debug)]
pub struct Loading {
  sender: Sender<Message>,
}

impl Loading {
  /// create an instance of loading
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

  /// create an instance of loading with a writer
  pub fn with_writer<W: Write + Send + 'static>(
    spinner: Spinner,
    writer: W,
  ) -> Self {
    Self::new(spinner, Some(Box::new(writer)))
  }

  /// send an error message
  pub fn with_error<T: Into<String>>(&self, content: T) {
    block_on(send_with_error(&self.sender, content));
  }

  /// send an icon message
  pub fn with_icon<T: Into<String>>(&self, icon: T, content: T) {
    block_on(send_with_icon(&self.sender, icon, content));
  }

  /// send an info message
  pub fn with_info<T: Into<String>>(&self, content: T) {
    block_on(send_with_info(&self.sender, content));
  }

  /// send a success message
  pub fn send_with_success<T: Into<String>>(&self, content: T) {
    block_on(send_with_success(&self.sender, content));
  }

  /// send a text message
  pub fn with_text<T: Into<String>>(&self, content: T) {
    block_on(send_with_text(&self.sender, content));
  }

  /// send a time message
  pub fn with_time<T: Into<String>>(&self, content: T) {
    block_on(send_with_time(&self.sender, content));
  }

  /// send a warning message
  pub fn with_warning<T: Into<String>>(&self, content: T) {
    block_on(send_with_warning(&self.sender, content));
  }

  /// stop the spinner
  pub fn stop(self) {
    block_on(send_stop(&self.sender))
  }
}

/// write a message to a writer
async fn write<T: Into<String>, W: Write + Send>(
  mut writer: W,
  contents: T,
) -> io::Result<()> {
  write!(writer, "\x1B[2K\x1B[0G")?;
  write!(writer, "{}", contents.into())?;
  writer.flush()
}

/// send an error message
async fn send_with_error<T: Into<String>>(
  sender: &Sender<Message>,
  content: T,
) {
  sender
    .send_async(Message::Next(Icon::Error, content.into()))
    .await
    .expect("event to have been sent")
}

/// send an icon message
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

/// send an info message
async fn send_with_info<T: Into<String>>(sender: &Sender<Message>, content: T) {
  sender
    .send_async(Message::Next(Icon::Info, content.into()))
    .await
    .expect("event to have been sent")
}

/// send a success message
async fn send_with_success<T: Into<String>>(
  sender: &Sender<Message>,
  content: T,
) {
  sender
    .send_async(Message::Next(Icon::Success, content.into()))
    .await
    .expect("event to have been sent")
}

/// send a text message
async fn send_with_text<T: Into<String>>(sender: &Sender<Message>, content: T) {
  sender
    .send_async(Message::WithText(content.into()))
    .await
    .expect("event to have been sent");
}

/// send a time message
async fn send_with_time<T: Into<String>>(sender: &Sender<Message>, content: T) {
  sender
    .send_async(Message::Next(Icon::Time, content.into()))
    .await
    .expect("event to have been sent")
}

/// send a warning message
async fn send_with_warning<T: Into<String>>(
  sender: &Sender<Message>,
  content: T,
) {
  sender
    .send_async(Message::Next(Icon::Warning, content.into()))
    .await
    .expect("event to have been sent")
}

/// stop the spinner
async fn send_stop(sender: &Sender<Message>) {
  sender
    .send_async(Message::Stop)
    .await
    .expect("event to have been sent");

  thread::sleep(Duration::from_millis(INTERVAL));
}
