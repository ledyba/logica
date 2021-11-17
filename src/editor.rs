mod app;

use egui_glium::EguiGlium;
use glium::Display;
use winit::{
  dpi::LogicalSize,
  event_loop::{ControlFlow, EventLoop},
  platform::{run_return::EventLoopExtRunReturn, windows::WindowBuilderExtWindows}
};
pub struct Editor {
  inner: Option<EditorImpl>,
}

struct EditorImpl {
  app: app::App,
  event_loop: EventLoop<()>,
  display: Display,
  egui: EguiGlium,
}

impl Editor {
  pub fn new() -> Self {
    Self {
      inner: None,
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
    //egui_glium::run(Box::new(App::default()), &epi::NativeOptions::default());
    let window_builder =
        winit::window::WindowBuilder::new()
        .with_title("Logica")
        .with_inner_size(LogicalSize::new(800, 600))
        .with_resizable(false)
        .with_parent_window(unsafe { std::mem::transmute(parent) });

    let context_builder = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    let event_loop = winit::event_loop::EventLoop::new();

    let display =
        glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Failed to create display");

    let egui = EguiGlium::new(&display);
    
    self.inner = Some(EditorImpl {
      app: app::App::default(),
      event_loop,
      display,
      egui
    });

    true
  }

  fn is_open(&mut self) -> bool {
    self.inner.is_some()
  }

  fn idle(&mut self) {
    let inner = if let Some(inner) = self.inner.as_mut() {
      inner
    } else {
      return;
    };
    let event_loop = &mut inner.event_loop;
    use winit::event;
    event_loop.run_return(|event, _window_target, control_flow| {
      let mut exit_loop = || { *control_flow = ControlFlow::Exit };
      match event {
        event::Event::NewEvents(_) => todo!(),
        event::Event::WindowEvent { window_id, event } => todo!(),
        event::Event::DeviceEvent { device_id, event } => todo!(),
        event::Event::UserEvent(_) => todo!(),
        event::Event::Suspended => todo!(),
        event::Event::Resumed => todo!(),
        event::Event::MainEventsCleared => {
          // Nothing to do
        },
        event::Event::RedrawRequested(_) => todo!(),
        event::Event::RedrawEventsCleared => exit_loop(),
        event::Event::LoopDestroyed => todo!(),
      }
    });
  }

  fn close(&mut self) {
    self.inner = None;
  }
}
