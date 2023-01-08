mod converter;
mod track;

use cpal::{SampleFormat, Stream};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use log::error;

use track::Track;

pub struct Player {
  config: cpal::StreamConfig,
  total_samples: usize,
  tracks: Vec<(usize, Box<dyn Track + Send + Sync + 'static>)>,
}

pub fn setup() -> anyhow::Result<(Stream, Arc<Mutex<Player>>)> {
  let host = cpal::default_host();
  let device = host.default_output_device().ok_or(anyhow::Error::msg("Failed to get default output device"))?;
  let config = device.default_output_config()?;
  let player = Arc::new(Mutex::new(Player::new(config.config())));
  let player_data = player.clone();
  let player_err = player.clone();
  let stream = match config.sample_format() {
    SampleFormat::I16 =>
      device.build_output_stream(
        &config.config(),
        move |buf, info| data_callback::<i16>(&player_data, buf, info),
        move |err| error_callback(&player_err, err)
      )?,
    SampleFormat::U16 =>
      device.build_output_stream(
        &config.config(),
        move |buf, info| data_callback::<u16>(&player_data, buf, info),
        move |err| error_callback(&player_err, err)
      )?,
    SampleFormat::F32 =>
      device.build_output_stream(
        &config.config(),
        move |buf, info| {
          let mut player = player_data.lock().expect("Poisoned");
          player.on_play(buf, info);
        },
        move |err| error_callback(&player_err, err)
      )?,
  };
  Ok((stream, player))
}

fn data_callback<T>(player: &Arc<Mutex<Player>>, buf: &mut [T], info: &cpal::OutputCallbackInfo)
  where
    T: cpal::Sample + Sync + Send + 'static,
    converter::ConverterImpl<T> : converter::Converter<T>,
{
  let mut player = player.lock().expect("Poisoned");
  let mut buf_f32 = vec![0.0_f32; buf.len()];
  player.on_play(&mut buf_f32, info);
  <converter::ConverterImpl<T> as converter::Converter<T>>::convert(&buf_f32, buf);
}

fn error_callback(player: &Arc<Mutex<Player>>, err: cpal::StreamError)
{
  let player = player.lock().expect("Poisoned");
  player.on_error(err);
}

impl Player {
  fn new(
    config: cpal::StreamConfig,
  ) -> Self {
    Self {
      config,
      total_samples: 0,
      tracks: Vec::new(),
    }
  }

  fn on_play(&mut self, buf: &mut [f32], _info: &cpal::OutputCallbackInfo) {
    for (from, track) in &mut self.tracks {
      if *from < self.total_samples {
        continue;
      }
      let ts = (*from - self.total_samples) as f64 / self.config.sample_rate.0 as f64;
      track.play(ts, buf);
    }
    self.tracks.retain(|(_, it)| !it.is_done());
    self.total_samples += buf.len();
  }

  fn on_error(self: &Self, err: cpal::StreamError) {
    error!("{}", err);
  }

  pub fn register(&mut self, offset: f64, track: Box<dyn Track + Send + Sync + 'static>) {
    self.tracks.push((self.total_samples + (offset * self.config.sample_rate.0 as f64) as usize, track));
  }
}
