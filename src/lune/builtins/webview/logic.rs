use tokio::sync::watch::{Receiver, Sender};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::Window,
};
use wry::WebView;

use super::CLOSED_WINDOW_MSG;

pub struct Logic<'a> {
    window: &'a Window,
    webview: WebView,
    tx: Sender<String>,
    rx: &'a mut Receiver<String>,
}

impl<'logic> Logic<'logic> {
    pub fn new(
        window: &'logic Window,
        webview: WebView,
        tx: Sender<String>,
        rx: &'logic mut Receiver<String>,
    ) -> Self {
        Self {
            window,
            webview,
            tx,
            rx,
        }
    }

    fn channel_logic(&mut self, elwt: &EventLoopWindowTarget<()>) {
        if self.rx.has_changed().is_ok() && self.rx.has_changed().unwrap() {
            let line = self.rx.borrow_and_update();

            match line.as_str() {
                "^CloseWindow" => {
                    elwt.exit();

                    if self.tx.send(CLOSED_WINDOW_MSG.to_owned()).is_err() {
                        println!("Channel listening to window closing is closed");
                    }
                }
                "^OpenDevtools" => self.webview.open_devtools(),
                "^CloseDevtools" => self.webview.close_devtools(),
                str => {
                    if str.starts_with("^LoadUrl:") {
                        self.webview
                            .load_url(str.replace("^LoadUrl:", "").as_str())
                            .expect("Failed to load url");
                    }
                }
            }
        }
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop
            .run(|event, elwt| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();

                    if self.tx.send(CLOSED_WINDOW_MSG.to_owned()).is_err() {
                        println!("Channel listening to window closing is closed");
                    }
                }
                Event::AboutToWait => {
                    self.window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => self.channel_logic(elwt),
                _ => (),
            })
            .unwrap();

        self.window.set_visible(false);
    }
}
