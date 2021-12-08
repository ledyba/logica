use std::sync::{Arc, Mutex};
use log::info;
use serde::{Deserialize, Serialize};
use vst::plugin::{Category, HostCallback, Info, PluginParameters};
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

impl PluginParameters for Parameter {
  fn load_bank_data(&self, data: &[u8]) {
    info!("Preset data loaded");
    let mut preset = self.0.lock().expect("Failed to lock");
    *preset = bincode::deserialize(data).expect("Failed to load preset data");
  }

  fn get_bank_data(&self) -> Vec<u8> {
    info!("Preset data saved");
    let preset = self.0.lock().expect("Failed to lock");
    bincode::serialize(&*preset).expect("Failed to serialize")
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
  fn init(&mut self) {
    simple_logging::log_to_file("logica.log", log::LevelFilter::Info).expect("Failed to open log file.");
    info!("Logica initialized.");
  }
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

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.parameter) as Arc<dyn PluginParameters>
  }

  fn get_editor(&mut self) -> Option<Box<dyn vst::editor::Editor>> {
    Some(Box::new(Editor::new(Arc::clone(&self.parameter))))
  }
}
