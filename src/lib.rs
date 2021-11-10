pub mod app;

use vst::plugin::{HostCallback, Info, Plugin};

struct LogicaPlugin {

}

impl Default for LogicaPlugin {
  fn default() -> Self {
    Self {
      
    }
  }
}

impl Plugin for LogicaPlugin {
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

vst::plugin_main!(LogicaPlugin);
