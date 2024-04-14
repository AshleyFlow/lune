use std::thread;

use tokio::sync::watch::{Receiver, Sender};
use winit::{
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows,
    window::WindowBuilder,
};
use wry::WebViewBuilder;

use super::logic;

pub struct BuilderConfig {
    pub url: String,
}

pub fn start(
    tx: Sender<String>,
    rx: Receiver<String>,
    config: BuilderConfig,
) -> Result<(), String> {
    thread::spawn(move || {
        let event_loop = EventLoopBuilder::new()
            .with_any_thread(true)
            .build()
            .expect("Failed to build event loop for webview.");

        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let webview = WebViewBuilder::new(&window)
            .with_url(config.url)
            .build()
            .expect("Failed to build webview");

        let logic = logic::Logic::new(&window, webview, tx, rx);

        event_loop.set_control_flow(ControlFlow::Poll);
        logic.run(event_loop);
    });

    Ok(())
}
