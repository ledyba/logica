mod dialog;

use std::{sync::{Arc, Mutex}, time::Instant};

use egui_glium::EguiGlium;
use epi::App;
use epi::backend::RepaintSignal;
use glium::Display;
use log::info;
use winit::{dpi::LogicalSize, event_loop::{ControlFlow, EventLoop, EventLoopProxy}, platform::{run_return::EventLoopExtRunReturn, windows::{WindowBuilderExtWindows, EventLoopExtWindows}}};
use winit::event::WindowEvent;

use crate::proxy::Parameter;
pub struct Editor {
  inner: Option<EditorImpl>,
  parameter: Arc<Parameter>,
}

struct EditorImpl {
  app: crate::editor::dialog::Dialog,
  egui: EguiGlium,
  display: Display,
  frame: epi::Frame,
  event_loop: EventLoop<RequestRepaintEvent>,
  focused: bool,
  quit: bool,
}

impl Editor {
  pub fn new(parameter: Arc<Parameter>) -> Self {
    Self {
      inner: None,
      parameter,
    }
  }
}

#[derive(Debug)]
struct RequestRepaintEvent;

struct RepaintSignalImpl {
  event_loop_proxy: Mutex<EventLoopProxy<RequestRepaintEvent>>,
}

impl epi::backend::RepaintSignal for RepaintSignalImpl {
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
    (400, 300)
  }

  fn position(&self) -> (i32, i32) {
    (0,0)
  }

  fn idle(&mut self) {
    let inner = if let Some(inner) = self.inner.as_mut() {
      inner
    } else {
      return;
    };
    let app = &mut inner.app;
    let egui = &mut inner.egui;
    let display = &mut inner.display;
    let frame = &mut inner.frame;
    let event_loop = &mut inner.event_loop;
    info!("Idle");
    event_loop.run_return(|event, _window_target, control_flow| {
      info!("Loop");
      let mut draw = || {
        let frame_start = std::time::Instant::now();

        let raw_input = egui.egui_winit.take_egui_input(display.gl_window().window());
        let (egui_output, shapes) =
          egui.egui_ctx.run(raw_input, |egui_ctx| {
            app.update(egui_ctx, frame);
          });

        let needs_repaint = egui_output.needs_repaint;
        egui.egui_winit.handle_output(display.gl_window().window(), &egui.egui_ctx, egui_output);

        let app_output = frame.take_app_output();
        inner.quit |= app_output.quit;
        let mut tex_allocation_data =
          egui_winit::epi::handle_app_output(
            display.gl_window().window(),
            egui.egui_ctx.pixels_per_point(),
            app_output
          );

        let frame_time = (std::time::Instant::now() - frame_start).as_secs_f64() as f32;
        frame.lock().info.cpu_usage = Some(frame_time);

        let clipped_meshes = egui.egui_ctx.tessellate(shapes);
        for (id, image) in tex_allocation_data.creations {
          egui.painter.set_texture(display, id, &image);
        }

        {
          use glium::Surface as _;
          let mut target = display.draw();
          let color = app.clear_color();
          target.clear_color(color[0], color[1], color[2], color[3]);
          egui.painter.paint_meshes(
            &display,
            &mut target,
            egui.egui_ctx.pixels_per_point(),
            clipped_meshes,
            &egui.egui_ctx.font_image(),
          );
          target.finish().unwrap();
        }

        for id in tex_allocation_data.destructions.drain(..) {
          egui.painter.free_texture(id);
        }

        if needs_repaint {
          display.gl_window().window().request_redraw();
          *control_flow =ControlFlow::Poll;
        } else {
          *control_flow =ControlFlow::Exit;
        }
      };
      use winit::event;
      // https://github.com/emilk/egui/blob/0.16.1/egui_glium/src/epi_backend.rs
      match event {
        event::Event::WindowEvent { window_id: _, event } => {
          match event {
            winit::event::WindowEvent::CloseRequested | winit::event::WindowEvent::Destroyed => {
              inner.quit = true;
              *control_flow = ControlFlow::Exit;
            }
            winit::event::WindowEvent::Focused(new_focused) => {
              inner.focused = new_focused;
            }
            _ => (),
          }
          egui.on_event(&event);
          display.gl_window().window().request_redraw();
        },
        event::Event::RedrawRequested(_) => {},
        event::Event::RedrawEventsCleared => {
          if inner.focused {
            draw();
          }
        },
        event::Event::LoopDestroyed => {
          inner.quit = true;
          *control_flow = ControlFlow::Exit;
        },
        event::Event::UserEvent(RequestRepaintEvent) => {
          // Repaint Signalを送るとここに飛んでくる
          display.gl_window().window().request_redraw();
        },
        _ => (),
      }
    });
    if inner.quit {
      app.on_exit();
      self.inner = None;
    }
  }

  fn close(&mut self) {
    info!("Closed");
    if let Some(inner) = self.inner.as_mut() {
      inner.quit = true;
    }
  }

  fn open(&mut self, parent: *mut std::ffi::c_void) -> bool {
    //egui_glium::run(Box::new(App::default()), &epi::NativeOptions::default());
    let window_builder =
        winit::window::WindowBuilder::new()
        .with_title("Logica")
        .with_inner_size(LogicalSize::new(400, 300))
        .with_resizable(false)
        .with_decorations(false)
        .with_parent_window(unsafe { std::mem::transmute(parent) });

    let context_builder = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    // FIXME(ledyba): ここでDPI awareでないソフトだとDPI awareになって小さくなってしまう
    // let event_loop: EventLoop<RequestRepaintEvent> = EventLoop::with_user_event();
    let event_loop: EventLoop<RequestRepaintEvent> = EventLoop::new_dpi_unaware();

    let display =
        glium::Display::new(window_builder, context_builder, &event_loop)
          .expect("Failed to create display");

    let repaint_signal: Arc<dyn RepaintSignal> =
      std::sync::Arc::new(RepaintSignalImpl {
        event_loop_proxy: std::sync::Mutex::new(event_loop.create_proxy())
      });

    //egui_winit::epi::EpiIntegration::new();
    let frame = epi::Frame::new(epi::backend::FrameData {
      info: epi::IntegrationInfo {
        name: "egui_logica",
        web_info: None,
        prefer_dark_mode: None, // TODO: figure out system default
        cpu_usage: None,
        native_pixels_per_point: Some(egui_winit::native_pixels_per_point(display.gl_window().window())),
      },
      output: Default::default(),
      repaint_signal: Arc::clone(&repaint_signal),
    });

    let mut egui = EguiGlium::new(&display);

    let mut app = dialog::Dialog::new(
      repaint_signal,
      Arc::clone(&self.parameter)
    );

    { // setup
      app.setup(&egui.egui_ctx, &frame, None);
      let app_output = frame.take_app_output();
      let tex_alloc_data = egui_winit::epi::handle_app_output(
        display.gl_window().window(),
        egui.egui_ctx.pixels_per_point(),
        app_output
      );
      frame.lock().output.tex_allocation_data = tex_alloc_data;
    }

    self.inner = Some(EditorImpl {
      app,
      egui,
      display,
      frame,
      event_loop,
      focused: true,
      quit: false,
    });

    true
  }

  fn is_open(&mut self) -> bool {
    self.inner.is_some()
  }
}

