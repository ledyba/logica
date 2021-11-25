use std::{path::PathBuf, process::exit, sync::Arc};
use im_native_dialog::ImNativeFileDialog;
use crate::proxy::Parameter;

pub struct Dialog {
  file_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
  parameter: Arc<Parameter>,
}

impl Dialog {
  pub fn new(parameter: Arc<Parameter>) -> Self {
    Self {
      file_path_dialog: Default::default(),
      parameter,
    }
  }
}

impl epi::App for Dialog {
  fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
    egui::panel::CentralPanel::default().show(ctx, |ui| {
      ui.label("Plugin");
      if ui.button("Load Plugin").clicked() {
        // https://github.com/emilk/egui/issues/270
        let repaint_signal = frame.repaint_signal();
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
      ui.horizontal(|ui| {
        let param = self.parameter.0.lock().expect("Failed to lock");
        ui.label("path: ");
        ui.label(param.path.as_str());
      });
    });
  }

  fn name(&self) -> &str {
    "logica"
  }
}