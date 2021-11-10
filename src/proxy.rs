use vst::plugin::{HostCallback, Info};

use crate::{editor::Editor, plugin::Plugin};

pub struct ProxyPlugin {
  host_callback: HostCallback,
  plugin: Option<Box<dyn Plugin>>,
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
  fn new(host_callback: HostCallback) -> Self {
    Self {
      host_callback,
      plugin: None,
    }
  }

  fn get_info(&self) -> Info {
    Info {
      name: "Logica".to_string(),
      unique_id: 1145131919, // Used by hosts to differentiate between plugins.
      ..Default::default()
    }
  }

  fn get_editor(&mut self) -> Option<Box<dyn vst::editor::Editor>> {
      Some(Box::new(Editor::new()))
  }
}

