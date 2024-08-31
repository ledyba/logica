pub enum Oscillator {
  Sin,
}

pub enum Node {
  MidiInput,
  Output,
  Oscillator(Oscillator),
}

impl Node {
  pub fn is_midi_input(&self) -> bool {
    match self {
      &Node::MidiInput => true,
      _ => false,
    }
  }
  pub fn is_output(&self) -> bool {
    match self {
      &Node::Output => true,
      _ => false,
    }
  }
}
