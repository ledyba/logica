use std::{path::PathBuf, process::exit};
use eframe::{egui, epi};
use im_native_dialog::ImNativeFileDialog;

pub struct App {
  on: bool,
  file_path_dialog: ImNativeFileDialog<Option<PathBuf>>,
}

impl Default for App {
  fn default() -> Self {
    Self {
      on: false,
      file_path_dialog: Default::default(),
    }
  }
}

impl epi::App for App {
  fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
    egui::TopPanel::top("wrap_app_top_bar").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        egui::menu::menu(ui, "File", |ui| {
          if ui.button("Save").clicked() {

            let repaint_signal = frame.repaint_signal();
            self.file_path_dialog
                .with_callback(move |_| repaint_signal.request_repaint())
                .open_single_file(Some(std::env::current_dir().unwrap()))
                .expect("Unable to open file_path dialog");
          }
          ui.separator();
          if ui.button("Exit").clicked() {
            exit(0);
          }
        });
      });
    });
    egui::Window::new("Box")
      .show(ctx, |ui| {
        ui.label("hey");
        ui.set_width(320.0);

        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        if response.clicked() {
            self.on = !self.on;
            response.mark_changed();
        }
        response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, self.on, ""));
    
        let how_on = ui.ctx().animate_bool(response.id, self.on);
        let visuals = ui.style().interact_selectable(&response, self.on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);        
    });
  }

  fn name(&self) -> &str {
    "logica"
  }
}