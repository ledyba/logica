use cpal::{SampleFormat, Stream};
use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::{Arc, Mutex};
use log::{debug, error};

use crate::synth::Synth;
use super::converter::{
  Converter,
  ConverterImpl,
};

pub struct PlayerImpl {
  config: cpal::StreamConfig,
  total_samples: usize,
  synth: Option<Synth>,
}

pub fn setup() -> anyhow::Result<(Stream, Arc<Mutex<PlayerImpl>>)> {
  let host = cpal::default_host();
  debug!("Host name = {}", host.id().name());
  let device = host.default_output_device().ok_or(anyhow::Error::msg("Failed to get default output device"))?;
  let config = device.default_output_config()?;
  let player = Arc::new(Mutex::new(PlayerImpl::new(config.config())));
  let player_data = player.clone();
  let player_err = player.clone();
  let stream = match config.sample_format() {
    SampleFormat::I16 =>
      device.build_output_stream(
        &config.config(),
        move |buff, info| data_callback::<i16>(&player_data, buff, info),
        move |err| error_callback(&player_err, err),
        None,
      )?,
    SampleFormat::U16 =>
      device.build_output_stream(
        &config.config(),
        move |buff, info| data_callback::<u16>(&player_data, buff, info),
        move |err| error_callback(&player_err, err),
        None,
      )?,
    SampleFormat::F32 =>
      device.build_output_stream(
        &config.config(),
        move |buff, info| {
          let mut player = player_data.lock().expect("Poisoned");
          player.on_play(buff, info);
        },
        move |err| error_callback(&player_err, err),
        None,
      )?,
    _ => todo!(),
  };
  Ok((stream, player))
}

fn data_callback<T>(player: &Arc<Mutex<PlayerImpl>>, buf: &mut [T], info: &cpal::OutputCallbackInfo)
  where
    T: cpal::Sample + Sync + Send + 'static,
    ConverterImpl<T> : Converter<T>,
{
  let mut player = player.lock().expect("Poisoned");
  let mut buf_f32 = vec![0.0_f32; buf.len()];
  player.on_play(&mut buf_f32, info);
  <ConverterImpl<T> as Converter<T>>::convert(&buf_f32, buf);
}

fn error_callback(player: &Arc<Mutex<PlayerImpl>>, err: cpal::StreamError) {
  let player = player.lock().expect("Poisoned");
  player.on_error(err);
}

impl PlayerImpl {
  fn new(
    config: cpal::StreamConfig,
  ) -> Self {
    Self {
      config,
      total_samples: 0,
      synth: None,
    }
  }

  fn on_play(&mut self, buff: &mut [f32], _info: &cpal::OutputCallbackInfo) {
    if let Some(synth) = &mut self.synth {
      synth.play(&self.config, buff, self.total_samples);
      self.total_samples += buff.len();
    }
  }

  fn on_error(self: &Self, err: cpal::StreamError) {
    error!("{}", err);
  }

  pub fn register(&mut self, synth: Synth) {
    self.total_samples = 0;
    self.synth = Some(synth);
  }

  pub fn unregister(&mut self) {
    self.total_samples = 0;
    self.synth = None;
  }
}
