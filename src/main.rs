
mod app;
use app::App;

fn main() -> anyhow::Result<()> {
  cpal::available_hosts().into_iter().for_each(|it| println!("{}", it.name()));
  println!("Let's dance!");
  let app = App::default();
  let native_options = eframe::NativeOptions::default();
  eframe::run_native(Box::new(app), native_options)
}
