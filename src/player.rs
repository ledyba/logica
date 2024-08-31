mod converter;
mod player;
mod player_impl;

use std::sync::{Arc, Mutex};
use cpal::traits::StreamTrait;

pub use player::SynthPlayer;

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
  pub fn start(&self) -> anyhow::Result<()> {
    self.stream.play()?;
    Ok(())
  }

  pub fn pause(&self) -> anyhow::Result<()> {
    self.stream.pause()?;
    Ok(())
  }

  pub fn register(&self, offset: f64, track: Box<SynthPlayer>) {
    let mut inner = self.inner.lock().expect("[BUG] Lock poisoned");
    inner.register(offset, track);
  }
}
