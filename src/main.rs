#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::error::Error;
use wry::{
    webview::WebViewBuilder,
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{WindowBuilder, Fullscreen},
    },
};
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// URL to open
    url: String,

    /// Create borderless fullscreens on current monitor
    #[arg(long)]
    fullscreen: bool,

    /// Window title
    #[arg(long, default_value = "Bellamy")]
    title: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(cli.title)
        .with_fullscreen(match cli.fullscreen {
            true => Some(Fullscreen::Borderless(None)),
            false => None
        })
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_url(&cli.url)?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("Bellamy has started!")
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit
            }
            _ => (),
        };
    });
}
