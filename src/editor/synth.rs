mod nodes;

use std::rc::Rc;
use eframe::egui;
use crate::player::Player;
use crate::synth::Synth;

pub struct SynthEditor {
  player: Rc<Player>,
  synth: Synth,
  graph_state: nodes::GraphState,
  editor_state: nodes::EditorState,
}

impl SynthEditor {
  pub fn new(player: Rc<Player>) -> Self {
    let synth = Synth::default();
    Self {
      player,
      synth,
      graph_state: nodes::GraphState::default(),
      editor_state: nodes::EditorState::default(),
    }
  }

  pub fn show(&mut self, ui: &mut egui::Ui) {
    let graph_response = self.editor_state.draw_graph_editor(ui, nodes::AllNodeTemplates, &mut self.graph_state);
  }

  pub fn play(&mut self) {
    self.player.register(0.0, Box::new(crate::player::SynthTrack::new(&self.synth)));
  }
}
