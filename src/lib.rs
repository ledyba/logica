pub mod app;

use vst::plugin::{HostCallback, Info, Plugin};

struct LogicaProxy {
}

impl Default for LogicaProxy {
  fn default() -> Self {
    Self {

    }
  }
}

impl Plugin for LogicaProxy {
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

vst::plugin_main!(LogicaProxy);
