mod app;

use std::{sync::Mutex, time::Instant};

use egui_glium::EguiGlium;
use epi::App;
use glium::Display;
use winit::{dpi::LogicalSize, event_loop::{ControlFlow, EventLoop, EventLoopProxy}, platform::{run_return::EventLoopExtRunReturn, windows::{WindowBuilderExtWindows, EventLoopExtWindows}}};
pub struct Editor {
  inner: Option<EditorImpl>,
}

struct EditorImpl {
  app: app::App,
  event_loop: EventLoop<RequestRepaintEvent>,
  display: Display,
  repaint_signal: std::sync::Arc<RepaintSignalImpl>,
  egui: EguiGlium,
  focused: bool,
  previous_frame_time: Option<f32>,
}

impl Editor {
  pub fn new() -> Self {
    Self {
      inner: None,
    }
  }
}

#[derive(Debug)]
struct RequestRepaintEvent;

struct RepaintSignalImpl {
  event_loop_proxy: Mutex<EventLoopProxy<RequestRepaintEvent>>,
}

impl epi::RepaintSignal for RepaintSignalImpl {
  fn request_repaint(&self) {
    self.event_loop_proxy
      .lock()
      .expect("Failed to lock proxy")
      .send_event(RequestRepaintEvent)
      .expect("Failed to send event");
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
        .with_decorations(false)
        .with_parent_window(unsafe { std::mem::transmute(parent) });

    let context_builder = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    // FIXME: ここでDPI awareでないソフトだとDPI awareになって小さくなってしまう
    // let event_loop: EventLoop<RequestRepaintEvent> = EventLoop::with_user_event();
    let event_loop: EventLoop<RequestRepaintEvent> = EventLoop::new_dpi_unaware();

    let display =
        glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Failed to create display");
    

    let repaint_signal =
      std::sync::Arc::new(RepaintSignalImpl {
        event_loop_proxy: std::sync::Mutex::new(event_loop.create_proxy())
      });
  
    let mut egui = EguiGlium::new(&display);

    let mut app = app::App::default();

    {
      let (ctx, painter) = egui.ctx_and_painter_mut();
      let mut app_output = epi::backend::AppOutput::default();
      let mut frame = epi::backend::FrameBuilder {
          info: integration_info(&display, None),
          tex_allocator: painter,
          output: &mut app_output,
          repaint_signal: repaint_signal.clone(),
      }
      .build();
      app.setup(ctx, &mut frame, None);
    }

    self.inner = Some(EditorImpl {
      app,
      event_loop,
      display,
      repaint_signal,
      egui,
      focused: true,
      previous_frame_time: None,
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
    let egui = &mut inner.egui;
    let display = &mut inner.display;
    let repaint_signal = &mut inner.repaint_signal;
    let app = &mut inner.app;
    let previous_frame_time = &mut inner.previous_frame_time;

    event_loop.run_return(|event, _window_target, control_flow| {
      let mut draw = || {
        let frame_start = std::time::Instant::now();
        egui.begin_frame(&display);
        let (ctx, painter) = egui.ctx_and_painter_mut();
        let mut app_output = epi::backend::AppOutput::default();
        let mut frame = epi::backend::FrameBuilder {
            info: integration_info(&display, *previous_frame_time),
            tex_allocator: painter,
            output: &mut app_output,
            repaint_signal: repaint_signal.clone(),
        }
        .build();
        app.update(ctx, &mut frame);
        let (needs_repaint, shapes) = egui.end_frame(&display);

        let frame_time = (Instant::now() - frame_start).as_secs_f64() as f32;
        *previous_frame_time = Some(frame_time);

        {
          use glium::Surface as _;
          let mut target = display.draw();
          let color = app.clear_color();
          target.clear_color(color[0], color[1], color[2], color[3]);
          egui.paint(&display, &mut target, shapes);
          target.finish().unwrap();
        }

        {
          egui_winit::epi::handle_app_output(
            display.gl_window().window(),
            egui.ctx().pixels_per_point(),
            app_output.clone(),
          );

          *control_flow =
            if app_output.quit {
              glium::glutin::event_loop::ControlFlow::Exit
            } else if needs_repaint {
              display.gl_window().window().request_redraw();
              glium::glutin::event_loop::ControlFlow::Poll
            } else {
              glium::glutin::event_loop::ControlFlow::Wait
            };
        }
      };
    use winit::event;
      match event {
        event::Event::NewEvents(_) => {},
        event::Event::WindowEvent { window_id, event } => {
          if egui.is_quit_event(&event) {
            app.on_exit();
            *control_flow = ControlFlow::Exit;
          }
          if let event::WindowEvent::Focused(new_focused) = event {
            inner.focused = new_focused;
          }
          egui.on_event(&event);
          display.gl_window().window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
        },
        event::Event::DeviceEvent { device_id, event } => {},
        event::Event::UserEvent(_) => {},
        event::Event::Suspended => {},
        event::Event::Resumed => {},
        event::Event::MainEventsCleared => {},
        event::Event::RedrawRequested(_) => {},
        event::Event::RedrawEventsCleared => {
          if inner.focused {
            draw();
          }
          *control_flow = ControlFlow::Exit;
        },
        event::Event::LoopDestroyed => {
          app.on_exit();
        },
      }
    });
  }

  fn close(&mut self) {
    self.inner = None;
  }
}

fn integration_info(
  display: &glium::Display,
  previous_frame_time: Option<f32>,
) -> epi::IntegrationInfo {
  epi::IntegrationInfo {
      name: "egui_logica",
      web_info: None,
      prefer_dark_mode: None, // TODO: figure out system default
      cpu_usage: previous_frame_time,
      native_pixels_per_point: Some(egui_winit::native_pixels_per_point(
          display.gl_window().window(),
      )),
  }
}
