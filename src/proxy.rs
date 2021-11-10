use vst::plugin::{HostCallback, Info, Plugin};

pub struct ProxyPlugin {
}

impl Default for ProxyPlugin {
  fn default() -> Self {
    Self {

    }
  }
}

impl Plugin for ProxyPlugin {
  fn new(_host: HostCallback) -> Self {
    Self {
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
