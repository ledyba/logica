use super::*;

pub struct SinNode {
  freq: f64,
}

impl SinNode {
  pub fn new(freq: f64) -> Self {
    Self {
      freq,
    }
  }
}

impl NodeImpl for SinNode {
  fn title(&self) -> &'static str {
    "SinNode"
  }

  fn ui(&mut self, node: &mut NodeContext) {
    node.constant("freq", &mut self.freq);
    node.output("output");
  }
}
