use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use eframe::egui::{Color32, RichText, Stroke, Ui, Vec2, Pos2};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Slot {
  Output(usize, String),
  Input(usize, String),
}

pub struct Stage {
  next_id: usize,
  slots: HashSet<Slot>,
  holes: HashMap<Slot, Pos2>,
  conn: HashSet<(Slot, Slot)>,
}

impl Stage {
  pub fn new() -> Self {
    Self {
      next_id: 0,
      slots: HashSet::new(),
      holes: HashMap::new(),
      conn: HashSet::new(),
    }
  }

  pub fn start_frame(&mut self) {
    self.slots.clear();
    self.holes.clear();
  }

  pub fn output(&mut self, ui: &mut Ui, title: &str) -> Slot {
    let id = self.next_id();
    ui.label(RichText::from(title).size(16.0));
    let pos = ui.cursor().right_top() + Vec2::new(-5.0, -8.0);
    ui.painter().circle_stroke(pos, 8.0, Stroke::new(2.0,Color32::DARK_GRAY));
    let slot = Slot::Output(id, title.to_string());
    self.slots.insert(slot.clone());
    self.holes.insert(slot.clone(), pos);
    slot
  }

  pub fn input(&mut self, ui: &mut Ui, title: &str) -> Slot {
    let id = self.next_id();
    ui.label(RichText::from(title).size(16.0));
    let pos = ui.cursor().right_top() + Vec2::new(-5.0, -8.0);
    ui.painter().circle_stroke(pos, 8.0, Stroke::new(2.0,Color32::DARK_GRAY));
    let slot = Slot::Output(id, title.to_string());
    self.slots.insert(slot.clone());
    self.holes.insert(slot.clone(), pos);
    slot
  }

  pub fn ui(&mut self, ui: &mut Ui) {
    self.conn.retain(|(input, output)| {
      self.slots.contains(input) && self.slots.contains(output)
    });
    for (input, output) in &self.conn {
      let from = self.holes.get(input).expect("[BUG]");
      let to = self.holes.get(output).expect("[BUG]");
      // Draw line.
    }
  }

  fn next_id(&mut self) -> usize {
    let id = self.next_id;
    self.next_id += 1;
    id
  }
}
