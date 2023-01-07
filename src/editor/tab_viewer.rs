use super::tab::Tab;
use eframe::egui;

pub struct TabViewer {
}

impl TabViewer {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl egui_dock::TabViewer for TabViewer {
  type Tab = Tab;

  fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
    match tab {
      Tab::Synth(tab) => tab.ui(ui),
    }
  }

  fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
    match tab {
      Tab::Synth(tab) => tab.title(),
    }
  }
}
