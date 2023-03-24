#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::error::Error;
use wry::{
    webview::WebViewBuilder,
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{WindowBuilder, Fullscreen},
    },
};
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// URL to open
    #[arg(default_value = "https://ilpropheta.github.io/bellamy")]
    url: String,

    /// Create maximized window
    #[arg(long)]
    maximized: bool,

    /// Create borderless fullscreens on current monitor
    #[arg(long)]
    fullscreen: bool,

    /// Enable developer tools
    #[arg(long, default_value="false")]
    devtools: bool,

    /// Disable the close button
    #[arg(long, default_value="false")]
    unclosable: bool,

    /// Window title
    #[arg(long, default_value = "Bellamy")]
    title: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(cli.title)
        .with_maximized(cli.maximized)
        .with_fullscreen(match cli.fullscreen {
            true => Some(Fullscreen::Borderless(None)),
            false => None
        })
        .with_closable(!cli.unclosable)
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_url(&cli.url)?
        .with_devtools(cli.devtools)
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit
            }
            _ => (),
        };
    });
}
