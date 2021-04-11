use std::error::Error;
use std::borrow::Borrow;

mod egui_nodes;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
  cpal::available_hosts().into_iter().for_each(|it| println!("{}", it.name()));
  println!("Let's dance!");
  egui_glium::run(Box::new(app::App::default()))
}
