use std::rc::Rc;
use eframe::egui::Visuals;
use crate::editor::Editor;

mod editor;
mod player;
mod synth;

fn setup_logger() -> Result<(), fern::InitError> {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{}[{}][{}] {}",
        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
        record.target(),
        record.level(),
        message,
      ))
    })
    .level(log::LevelFilter::Debug)
    .chain(std::io::stdout())
    //.chain(fern::log_file("output.log")?)
    .apply()?;
  Ok(())
}

fn main() -> anyhow::Result<()> {
  setup_logger()?;
  let options = eframe::NativeOptions {
    ..Default::default()
  };
  let player = player::setup()?;
  eframe::run_native(
    "Logica",
    options,
    Box::new(|cc| {
      cc.egui_ctx.set_visuals(Visuals::dark());
      Ok(Box::new(Editor::new(Rc::new(player))))
    }),
  ).map_err(|err| anyhow::Error::msg(err.to_string()))?;
  Ok(())
}
