use std::thread;

use tokio::sync::broadcast::Receiver;
use tokio::sync::watch::Sender;
use winit::{
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows,
    window::WindowBuilder,
};
use wry::WebViewBuilder;

use super::logic::{
    self,
    config::{WebviewCommand, WebviewEvent},
};

pub struct BuilderConfig {
    pub url: Option<String>,
}

pub fn start(
    tx: Sender<WebviewEvent>,
    rx: Receiver<WebviewCommand>,
    config: BuilderConfig,
) -> Result<(), String> {
    thread::spawn(move || {
        let event_loop = EventLoopBuilder::new()
            .with_any_thread(true)
            .build()
            .expect("Failed to build event loop for webview.");

        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let webview = WebViewBuilder::new(&window)
            .build()
            .expect("Failed to build webview");

        if let Some(url) = config.url {
            webview
                .load_url(url.as_str())
                .expect("Failed to load url into webview");
        }

        let logic = logic::Logic::new(&window, webview, tx, rx);

        event_loop.set_control_flow(ControlFlow::Poll);
        logic.run(event_loop);
    });

    Ok(())
}
