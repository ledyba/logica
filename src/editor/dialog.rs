use std::{path::PathBuf, sync::Arc};
use epi::backend::RepaintSignal;
use im_native_dialog::ImNativeFileDialog;
use crate::proxy::Parameter;

pub struct Dialog {
  file_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
  repaint_signal: Arc<dyn RepaintSignal>,
  parameter: Arc<Parameter>,
}

impl Dialog {
  pub fn new(repaint_signal: Arc<dyn RepaintSignal>, parameter: Arc<Parameter>) -> Self {
    Self {
      file_path_dialog: Default::default(),
      repaint_signal,
      parameter,
    }
  }
}

impl epi::App for Dialog {
  fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
    egui::panel::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Logica");
      ui.separator();
      ui.label("Your plugin path:");
      {
        let param = self.parameter.0.lock().expect("Failed to lock");
        let path = param.path.as_str();
        if path.is_empty() {
          ui.label("<not loaded>");
        } else {
          ui.label(path);
        }
      }
      if ui.button("Load Plugin").clicked() {
        // https://github.com/emilk/egui/issues/270
        let repaint_signal = self.repaint_signal.clone();
        let parameter = self.parameter.clone();
        self.file_path_dialog
            .with_callback(move |result| {
              if let Ok(Some(path)) = result {
                let mut param = parameter.0.lock().expect("Failed to lock");
                param.path = path.to_string_lossy().to_string();
              }
              repaint_signal.request_repaint();
            })
            .open_single_file(Some(std::env::current_dir().unwrap()))
            .expect("Unable to open file_path dialog");
      }
    });
  }

  fn name(&self) -> &str {
    "logica"
  }
}