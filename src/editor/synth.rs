use eframe::egui;

pub struct Synth {
}

impl Synth {
  pub fn new() -> Self {
    Self {
    }
  }
}

pub struct SynthTab {
}

impl SynthTab {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl egui_dock::TabViewer for SynthTab {
  type Tab = Synth;

  fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
    ui.label("Hey");
  }

  fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
    "Synth".into()
  }
}
