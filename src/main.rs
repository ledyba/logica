
use logica::app::App;

fn main() -> anyhow::Result<()> {
  println!("Let's dance!");
  let app = App::default();
  let native_options = eframe::NativeOptions::default();
  eframe::run_native(Box::new(app), native_options)
}
