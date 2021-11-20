use vst::plugin::{Category, HostCallback, Info, PluginParameters};

use crate::{editor::Editor};
use logica_bridge::Plugin;

pub struct ProxyPlugin {
  host_callback: HostCallback,
  plugin: Option<Box<dyn Plugin>>,
}

struct ProxyPluginParameters {
  path: std::sync::Mutex<String>,
}

impl PluginParameters for ProxyPluginParameters {
  fn load_preset_data(&self, data: &[u8]) {
    let mut lock = self.path.lock().expect("Failed to lock");
    *lock = String::from_utf8(data.to_vec()).expect("Invalid data");
  }
  fn get_preset_data(&self) -> Vec<u8> {
    Vec::from(self.path.lock().expect("Failed to lock").as_bytes())
  }
}

impl Default for ProxyPlugin {
  fn default() -> Self {
    Self {
      host_callback: HostCallback::default(),
      plugin: None,
    }
  }
}

impl vst::plugin::Plugin for ProxyPlugin {
  fn get_info(&self) -> Info {
    Info {
      name: "Logica".to_string(),
      vendor: "Logica Developers".to_string(),
      unique_id: 1145131919, // Used by hosts to differentiate between plugins.
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
    }
  }

  fn get_editor(&mut self) -> Option<Box<dyn vst::editor::Editor>> {
      Some(Box::new(Editor::new()))
  }
}

