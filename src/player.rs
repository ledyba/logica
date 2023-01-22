use std::sync::{Arc, Mutex};
use cpal::traits::StreamTrait;

mod converter;
mod track;
mod player_impl;

pub use track::Track;

pub struct Player {
  stream: cpal::Stream,
  inner: Arc<Mutex<player_impl::PlayerImpl>>,
}

pub fn setup() -> anyhow::Result<Player> {
  let (stream, inner) = player_impl::setup()?;
  Ok(Player {
    stream,
    inner,
  })
}

impl Player {
  pub fn register(&self, offset: f64, track: Box<dyn Track + Send + Sync + 'static>) {
    let mut inner = self.inner.lock().expect("[BUG] Lock poisoned");
    inner.register(offset, track);
  }
  pub fn start(&self) -> anyhow::Result<()> {
    self.stream.play()?;
    Ok(())
  }
  pub fn pause(&self) -> anyhow::Result<()> {
    self.stream.pause()?;
    Ok(())
  }
}
