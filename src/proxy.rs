use std::sync::{Arc, Mutex};
use log::info;
use vst::plugin::{Category, HostCallback, Info, PluginParameters};
use serde::{Serialize, Deserialize};
use crate::editor::Editor;

pub struct Plugin {
  host_callback: HostCallback,
  plugin: Option<Box<dyn vst::plugin::Plugin>>,
  parameter: Arc<Parameter>,
}

#[derive(Default)]
pub struct Parameter(pub Mutex<ParameterData>);

#[derive(Default, Serialize, Deserialize)]
pub struct ParameterData {
  pub(crate) name: String,
  pub(crate) path: String,
}

// It just contains one preset, chunked.
impl PluginParameters for Parameter {
  fn get_preset_data(&self) -> Vec<u8> {
    info!("Preset data saved");
    self.save()
  }

  fn get_bank_data(&self) -> Vec<u8> {
    info!("Bank data saved");
    self.save()
  }

  fn load_preset_data(&self, data: &[u8]) {
    self.load(data);
    info!("Preset data loaded");
  }

  fn load_bank_data(&self, data: &[u8]) {
    self.load(data);
    info!("Bank data loaded");
  }
}

impl Parameter {
  fn load(&self, data: &[u8]) {
    let mut params = self.0.lock().expect("Failed to lock");
    *params = bincode::deserialize(data).expect("Failed to load preset data");
  }
  fn save(&self) -> Vec<u8> {
    let params = self.0.lock().expect("Failed to lock");
    bincode::serialize(&*params).expect("Failed to serialize")
  }
}

impl Default for Plugin {
  fn default() -> Self {
    Self {
      host_callback: HostCallback::default(),
      plugin: None,
      parameter: Default::default(),
    }
  }
}

impl vst::plugin::Plugin for Plugin {
  fn get_info(&self) -> Info {
    Info {
      name: "Logica".to_string(),
      vendor: "Logica developers".to_string(),
      presets: 1,
      unique_id: 1145141919, // Used by hosts to differentiate between plugins.
      version: 1,
      category: Category::Synth,
      preset_chunks: true,
      ..Default::default()
    }
  }
  fn new(host_callback: HostCallback) -> Self {
    Self {
      host_callback,
      plugin: None,
      parameter: Default::default(),
    }
  }

  fn init(&mut self) {
    let profile_path = std::env::var("USERPROFILE").expect("No env");
    let log_path = format!("{}/Documents/logica.log", profile_path);
    simple_logging::log_to_file(log_path, log::LevelFilter::Info).expect("Failed to open log file.");
    info!("Logica initialized.");
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.parameter) as Arc<dyn PluginParameters>
  }

  fn get_editor(&mut self) -> Option<Box<dyn vst::editor::Editor>> {
    Some(Box::new(Editor::new(Arc::clone(&self.parameter))))
  }
}
