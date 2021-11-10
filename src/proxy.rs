use vst::plugin::{HostCallback, Info};
use vst::plugin::Plugin as VstPlugin;

use crate::plugin::Plugin;

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

impl VstPlugin for ProxyPlugin {
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
}
