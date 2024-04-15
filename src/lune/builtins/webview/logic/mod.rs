pub mod config;

use std::ops::Deref;

use tokio::sync::broadcast::Receiver;
use tokio::sync::watch::Sender;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::Window,
};
use wry::WebView;

use self::config::{WebviewCommand, WebviewEvent};

pub struct Logic<'a> {
    window: &'a Window,
    webview: WebView,
    tx: Sender<WebviewEvent>,
    rx: Receiver<WebviewCommand>,
}

impl<'logic> Logic<'logic> {
    pub fn new(
        window: &'logic Window,
        webview: WebView,
        tx: Sender<WebviewEvent>,
        rx: Receiver<WebviewCommand>,
    ) -> Self {
        Self {
            window,
            webview,
            tx,
            rx,
        }
    }

    fn channel_logic(&mut self, elwt: &EventLoopWindowTarget<()>) {
        if let Ok(cmd) = self.rx.try_recv() {
            println!("{:?}", cmd);

            match cmd {
                WebviewCommand::CloseWindow => {
                    elwt.exit();

                    if self.tx.send(WebviewEvent::ClosedWindow).is_err() {
                        println!("Channel listening to window closing is closed");
                    }
                }
                WebviewCommand::OpenDevtools => self.webview.open_devtools(),
                WebviewCommand::CloseDevtools => self.webview.close_devtools(),
                WebviewCommand::ExecuteJavascript(script) => self
                    .webview
                    .evaluate_script(script.as_str())
                    .expect("Failed to evaluate javacsript code"),
                WebviewCommand::LoadUrl(url) => {
                    self.webview
                        .load_url(url.as_str())
                        .expect("Failed to load url");
                }
                _ => {}
            }
        }
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop
            .run(|event, elwt| {
                self.channel_logic(elwt);

                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        elwt.exit();

                        if self.tx.send(WebviewEvent::ClosedWindow).is_err() {
                            println!("Channel listening to window closing is closed");
                        }
                    }
                    Event::AboutToWait => {
                        self.window.request_redraw();
                    }
                    _ => (),
                }
            })
            .unwrap();

        self.window.set_visible(false);
    }
}
