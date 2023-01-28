mod nodes;

use std::rc::Rc;
use eframe::egui;
use crate::player::Player;
use crate::synth::Synth;

pub struct SynthTab {
  player: Rc<Player>,
  synth: Synth,
  graph_state: nodes::GraphState,
  editor_state: nodes::EditorState,
}

impl SynthTab {
  pub fn new(player: Rc<Player>) -> Self {
    let synth = Synth::default();
    Self {
      player,
      synth,
      graph_state: nodes::GraphState::default(),
      editor_state: nodes::EditorState::default(),
    }
  }

  pub fn ui(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
      if ui.button("▶ Play").clicked() {
        self.player.start().expect("[BUG] Failed to play");
        self.player.register(0.0, Box::new(crate::player::SynthTrack::new(&self.synth)))
      }
      if ui.button("■ Stop").clicked() {
        self.player.pause().expect("[BUG] Failed to pause");
      }
    });
    ui.separator();
    let graph_response = self.editor_state
      .draw_graph_editor(ui, nodes::AllNodeTemplates, &mut self.graph_state);
  }

  pub fn title(&mut self) -> egui::WidgetText {
    egui::WidgetText::from("Synth")
  }
}
