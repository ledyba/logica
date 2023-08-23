mod editor;
mod nodes;
mod slot;

use std::rc::Rc;
use eframe::egui::{Layout, Ui, Vec2, WidgetText};
use crate::editor::synth::nodes::{Node, SinNode};
use crate::player::Player;
use crate::synth::Synth;

use editor::Editor;

pub struct SynthEditor {
  player: Rc<Player>,
  synth: Synth,
  editor: Editor,
}

impl SynthEditor {
  pub fn new(player: Rc<Player>) -> Self {
    let synth = Synth::default();
    let mut editor = Editor::new();
    editor.add_node(Node::new(Vec2::new(0.0, 0.0), SinNode::new(440.0)));
    Self {
      player,
      synth,
      editor,
    }
  }

  pub fn ui(&mut self, ui: &mut Ui) {
    let max_rect = ui.available_rect_before_wrap();
    let mut ui = ui.child_ui(max_rect, Layout::default());
    self.editor.ui(&mut ui);
  }

  pub fn play(&mut self) {
    self.player.register(0.0, Box::new(crate::player::SynthTrack::new(&self.synth)));
  }
}
