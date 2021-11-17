mod app;

use winapi::shared::windef::HWND;
use winit::platform::windows::WindowBuilderExtWindows;

use app::App;

fn main() -> anyhow::Result<()> {
  println!("Let's dance!");
  let app = App::default();
  let native_options = epi::NativeOptions::default();
  egui_glium::run(Box::new(app), &native_options);
}

pub struct Editor {
}

impl Editor {
  pub fn new() -> Self {
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
    let event_loop = winit::event_loop::EventLoop::with_user_event();
    let window_builder =
        winit::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("Logica")
        .with_parent_window(unsafe { std::mem::transmute(parent) });
    let context_builder = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    let display =
      glium::Display::new(window_builder, context_builder, &event_loop)
      .expect("Failed to create display");

      true
  }

  fn is_open(&mut self) -> bool {
    todo!()
  }

  fn idle(&mut self) {}

  fn close(&mut self) {}
}
