#[derive(Debug, Clone)]
pub enum WebviewCommand {
    CloseWindow,
    OpenDevtools,
    CloseDevtools,
    ExecuteJavascript(String),
    LoadUrl(String),
}

#[derive(Debug)]
pub enum WebviewEvent {
    Init,
    ClosedWindow,
    ExecutedJavascript(String),
}
