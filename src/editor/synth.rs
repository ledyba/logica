mod nodes;

use std::rc::Rc;
use eframe::egui;
use eframe::egui::{Ui, WidgetText};
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

  pub fn ui(&mut self, ui: &mut Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
      let resp =
        self.editor_state.draw_graph_editor(
          ui,
          nodes::AllNodeTemplates,
          &mut self.graph_state,
        );
    });
  }

  pub fn title(&mut self) -> WidgetText {
    WidgetText::from("Synth")
  }

  pub fn play(&mut self) {
    self.player.register(0.0, Box::new(crate::player::SynthTrack::new(&self.synth)));
  }
}
