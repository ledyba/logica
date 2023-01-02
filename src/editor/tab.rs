mod synth;

use eframe::egui;
use synth::SynthTab;
use crate::synth::Synth;

pub enum Tab {
  Synth(SynthTab)
}

impl Tab {
  pub fn new_synth_tab() -> Self {
    Self::Synth(SynthTab::new(Synth::new()))
  }
}

pub struct TabViewer{

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
    ui.label("Hey");
  }

  fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
    match tab {
      Tab::Synth(_) => egui::WidgetText::from("Synth"),
    }
  }
}
