use eframe::egui;
use egui_snarl::{ui::{SnarlStyle, SnarlViewer}, Snarl};

pub struct NodeEnum {

}

pub struct SynthViewer {
}

pub struct SynthEditor {
  viewer: SynthViewer,
  snarl: Snarl<NodeEnum>,
  snarl_style: SnarlStyle,
}

impl SnarlViewer<NodeEnum> for SynthViewer {
    fn title(&mut self, node: &NodeEnum) -> String {
      "Synth".to_string()
    }

    fn outputs(&mut self, node: &NodeEnum) -> usize {
      0
    }

    fn inputs(&mut self, node: &NodeEnum) -> usize {
      0
    }

    fn show_input(
      &mut self,
      pin: &egui_snarl::InPin,
      ui: &mut eframe::egui::Ui,
      scale: f32,
      snarl: &mut Snarl<NodeEnum>
    ) -> egui_snarl::ui::PinInfo {
      egui_snarl::ui::PinInfo::star()
    }

    fn show_output(
      &mut self,
      pin: &egui_snarl::OutPin,
      ui: &mut eframe::egui::Ui,
      scale: f32,
      snarl: &mut Snarl<NodeEnum>,
    ) -> egui_snarl::ui::PinInfo {
      egui_snarl::ui::PinInfo::star()
    }
}

impl SynthViewer {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl SynthEditor {
    pub fn new() -> Self {
      Self {
        viewer: SynthViewer::new(),
        snarl: Default::default(),
        snarl_style: SnarlStyle::new(),
      }
    }

    pub fn ui(&mut self, ui: &mut eframe::egui::Ui) {
      self.snarl.show(
        &mut self.viewer,
        &self.snarl_style,
        egui::Id::new("snarl"),
        ui
      )
    }
}
