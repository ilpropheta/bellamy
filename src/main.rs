#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use wry::application::window::Fullscreen;

fn main() -> wry::Result<()> {
    use wry::{
      application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
      },
      webview::WebViewBuilder,
    };
  
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title("Bellamy")
      .with_fullscreen(Some(Fullscreen::Borderless(None)))
      .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
      .with_url("https://en.wikipedia.org/wiki/Samuel_Bellamy")?
      .build()?;
  
    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;
  
      match event {
        Event::NewEvents(StartCause::Init) => println!("Bellamy has started!"),
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
      }
    });
  }