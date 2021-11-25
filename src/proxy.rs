use std::{collections::HashMap, sync::{atomic::AtomicI32, Mutex}};
use vst::plugin::{Category, HostCallback, Info, PluginParameters};
use crate::editor::Editor;

pub struct Plugin {
  host_callback: HostCallback,
  plugin: Option<Box<dyn vst::plugin::Plugin>>,
}

struct Parameters {
  current_preset_id: AtomicI32,
  saved_presets: Mutex<HashMap<i32, PresetParameter>>,
  current_preset: Mutex<PresetParameter>,
}

struct PresetParameter {
  name: String,
  path: String,
}

impl PluginParameters for Parameters {
  fn change_preset(&self, preset: i32) {
    self.current_preset_id.store(preset, std::sync::atomic::Ordering::Relaxed);
  }

  fn get_preset_num(&self) -> i32 {
    self.current_preset_id.load(std::sync::atomic::Ordering::Relaxed)
  }

  fn set_preset_name(&self, name: String) {
    let preset = self.current_preset.lock().expect("Failed to lock");
    preset.name = name;
  }

  /// Get the name of the preset at the index specified by `preset`.
  fn get_preset_name(&self, preset: i32) -> String {
    let preset = self.current_preset.lock().expect("Failed to lock");
    preset.name.clone()
  }

  fn load_preset_data(&self, data: &[u8]) {
    let preset = self.current_preset.lock().expect("Failed to lock");
  }
  fn get_preset_data(&self) -> Vec<u8> {
    let preset = self.current_preset.lock().expect("Failed to lock");
  }
}

impl Default for Plugin {
  fn default() -> Self {
    Self {
      host_callback: HostCallback::default(),
      plugin: None,
    }
  }
}

impl vst::plugin::Plugin for Plugin {
  fn get_info(&self) -> Info {
    Info {
      name: "Logica".to_string(),
      vendor: "Logica Developers".to_string(),
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
    }
  }

  fn get_editor(&mut self) -> Option<Box<dyn vst::editor::Editor>> {
      Some(Box::new(Editor::new()))
  }
}
