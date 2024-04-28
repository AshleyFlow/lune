pub mod config;

pub static JAVASCRIPT_API: &str = r#"
let __LUNE__ = {}

__LUNE__.postMessage = (message) => {
    let stringified = JSON.stringify(message)
    window.ipc.postMessage(stringified)
}

__LUNE__.postInternalMessage = (message) => {
    message.__internal = true
    __LUNE__.postMessage(message)
}

window.addEventListener("mousemove", (event) => {
    let message = {
        position: {
            x: event.clientX,
            y: event.clientY,
        }
    }
    __LUNE__.postInternalMessage(message)
})

window.addEventListener("mousedown", () => {
    let message = {
        mousebutton: "left",
        pressed: true,
    }
    __LUNE__.postInternalMessage(message)
})

window.addEventListener("mouseup", () => {
    let message = {
        mousebutton: "left",
        pressed: false,
    }
    __LUNE__.postInternalMessage(message)
})

window.addEventListener("keydown", (event) => {
    let message = {
        keycode: event.key,
        pressed: true,
    }
    __LUNE__.postInternalMessage(message)
})

window.addEventListener("keyup", (event) => {
    let message = {
        keycode: event.key,
        pressed: false,
    }
    __LUNE__.postInternalMessage(message)
})

window.lune = __LUNE__
window.luneweb = window.lune
"#;
