mod converter;
mod player_impl;

use std::sync::{Arc, Mutex};
use cpal::traits::StreamTrait;

use crate::synth::Synth;

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
  pub fn start(&self, synth: Synth) -> anyhow::Result<()> {
    {
      let mut inner = self.inner.lock().expect("[BUG] Lock poisoned");
      inner.register(synth);
    }
    self.stream.play()?;
    Ok(())
  }

  pub fn pause(&self) -> anyhow::Result<()> {
    {
      let mut inner = self.inner.lock().expect("[BUG] Lock poisoned");
      inner.unregister();
    }
    self.stream.pause()?;
    Ok(())
  }
}
