use iced::{
  Align, Column, Container, Element, Length, Sandbox, Settings, Text,
};

// State
pub(crate) struct App {

}

// Message
#[derive(Debug, Clone, Copy)]
pub enum Message {

}

//
impl Sandbox for App {
  type Message = Message;

  fn new() -> Self {
    App {}
  }

  fn title(&self) -> String {
    String::from("logica")
  }

  /* update */
  fn update(&mut self, message: Message) {
    match message {
    }
  }

  // view logic
  fn view(&mut self) -> Element<Message> {
    Text::new("text").into()
  }
}