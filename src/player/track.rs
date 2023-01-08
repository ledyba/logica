pub trait Track {
  fn play(&mut self, ts: f64, buff: &mut [f32]);
  fn is_done(&self) -> bool;
}
