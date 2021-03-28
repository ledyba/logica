use iced::{Settings, Sandbox};
use std::error::Error;

mod widgets;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
  println!("Let's dance!");
  app::App::run(Settings{
    window: Default::default(),
    flags: (),
    default_font: None,
    default_text_size: 0,
    antialiasing: false
  }).map_err(|e| e.into())
}
