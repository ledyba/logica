mod app;

use app::App;

fn main() -> anyhow::Result<()> {
  println!("Let's dance!");
  let app = App::default();
  let native_options = eframe::NativeOptions::default();
  eframe::run_native(Box::new(app), native_options)
}

pub struct Editor {
}

impl Editor {
  fn new() -> Self {
    Self {

    }
  }
}

impl vst::editor::Editor for Editor {
  fn size(&self) -> (i32, i32) {
    (800, 600)
  }

  fn position(&self) -> (i32, i32) {
    (0,0)
  }

  fn open(&mut self, parent: *mut std::ffi::c_void) -> bool {
    todo!()
  }

  fn is_open(&mut self) -> bool {
    todo!()
  }

  fn idle(&mut self) {}

  fn close(&mut self) {}
}
