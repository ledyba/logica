use std::marker::PhantomData;
use cpal::SampleFormat;
use cpal::traits::{DeviceTrait, HostTrait};

struct PlayerImpl<T> {
  _phantom: PhantomData<T>,
  host: cpal::Host,
  device: cpal::Device,
  config: cpal::SupportedStreamConfig,
}

pub trait Player {
  fn run(&mut self) -> anyhow::Result<()>;
}

pub fn initialize() -> anyhow::Result<Box<dyn Player>> {
  let host = cpal::default_host();
  let device = host.default_output_device()?;
  let config = device.default_output_config()?;
  let player: Box<dyn Player> = match config.sample_format() {
    SampleFormat::I16 => Box::new(PlayerImpl::<i16>::new(host, device, config)),
    SampleFormat::U16 => Box::new(PlayerImpl::<u16>::new(host, device, config)),
    SampleFormat::F32 => Box::new(PlayerImpl::<f32>::new(host, device, config)),
  };
  Ok(player)
}

impl <T> PlayerImpl<T> {
  fn new(
    host: cpal::Host,
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
  ) -> Self {
    Self {
      _phantom: PhantomData::default(),
      host,
      device,
      config,
    }
  }
}

impl <T> Player for PlayerImpl<T> {
  fn run(&mut self) -> anyhow::Result<()> {
    Ok(())
  }
}

