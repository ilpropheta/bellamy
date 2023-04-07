#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod msgbox;

use std::error::Error;
use wry::{
    webview::WebViewBuilder,
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{WindowBuilder, Fullscreen}, platform::windows::{IconExtWindows},
    },
};
use clap::Parser;
use wry::application::window::Icon;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// URL to open
    url: String,

    /// Create maximized window
    #[arg(long)]
    maximized: bool,

    /// Create borderless fullscreens on current monitor
    #[arg(long)]
    fullscreen: bool,

    /// Enable developer tools
    #[arg(long)]
    devtools: bool,

    /// Disable the close button
    #[arg(long)]
    unclosable: bool,

    /// Window title
    #[arg(long, default_value = "Bellamy")]
    title: String,

    /// Path to the window icon
    #[arg(long)]
    icon: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::try_parse().map_err(|e| {
        msgbox::show("Bellamy", e.to_string());
        e
    })?;

    let event_loop = EventLoop::new();

    let icon = cli.icon
        .map(|icon| Icon::from_path(icon, None))
        .transpose()?;

    let window = WindowBuilder::new()
        .with_title(cli.title)
        .with_maximized(cli.maximized)
        .with_fullscreen(match cli.fullscreen {
            true => Some(Fullscreen::Borderless(None)),
            false => None
        })
        .with_closable(!cli.unclosable)
        .with_window_icon(icon)
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
