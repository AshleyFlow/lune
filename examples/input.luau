local winit = require("@luneweb/winit")
local window, handle: winit.WindowEventLoopHandle = winit.create_window({}), nil
local webview = winit.create_webview(window, {
	url = "https://tauri.app/",
})

handle = winit.event_loop(window, function(msg)
	if msg == winit.events.CloseRequested then
		window:close()
		handle.stop()
	elseif msg == winit.events.CursorMoved and msg.position then
		local alpha: winit.Dimension = {
			x = msg.position.x / window.size.x,
			y = msg.position.y / window.size.y,
		}

		local multiplier = -25
		local transform =
			`translateY({alpha.y * multiplier}px) translateX({alpha.x * multiplier}px)`
		local script = `document.body.style.transform = "{transform}"`

		webview:evaluate(script)
	end
end)

winit.run()
