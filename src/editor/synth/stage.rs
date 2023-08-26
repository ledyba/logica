use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use eframe::egui::{Ui, Vec2};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Slot {
  Output(usize, String),
  Input(usize, String),
}

pub struct Stage {
  slots: HashSet<Slot>,
  holes: HashMap<Slot, Vec2>,
  conn: HashSet<(Slot, Slot)>,
}

impl Stage {
  pub fn new() -> Self {
    Self {
      slots: HashSet::new(),
      holes: HashMap::new(),
      conn: HashSet::new(),
    }
  }

  pub fn start_frame(&mut self) {
    self.slots.clear();
    self.holes.clear();
  }

  pub fn output(&mut self, id: usize, title: &str, pos: Vec2) {
    let slot = Slot::Output(id, title.to_string());
    self.slots.insert(slot.clone());
    self.holes.insert(slot, pos);
  }

  pub fn input(&mut self, id: usize, title: &str, pos: Vec2) {
    let slot = Slot::Output(id, title.to_string());
    self.slots.insert(slot.clone());
    self.holes.insert(slot, pos);
  }

  pub fn ui(&mut self, ui: &mut Ui) {

  }

  pub fn render(&mut self, ui: &mut Ui) {
    self.conn.retain(|(input, output)| {
      self.slots.contains(input) && self.slots.contains(output)
    });
    for (input, output) in &self.conn {
      let from = self.holes.get(input).expect("[BUG]");
      let to = self.holes.get(output).expect("[BUG]");
      // Draw line.
    }
  }
}
