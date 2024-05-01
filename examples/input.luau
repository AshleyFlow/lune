local wry = require("@luneweb/wry")
local window, handle: wry.Connection = wry.create_window({}), nil
local webview = wry.create_webview(window, {
	url = "https://tauri.app/",
})

handle = wry.event_loop(window, function(msg)
	if msg == wry.events.CloseRequested then
		window:close()
		handle.stop()
	elseif msg == wry.events.CursorMoved and msg.position then
		local alpha: wry.Dimension = {
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

wry.run()
