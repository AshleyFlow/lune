local winit = require("@luneweb/winit")
local window = winit.create_window({})
winit.create_webview(window, {
	url = "https://tauri.app/",
})

local handle
handle = winit.event_loop(window, function(msg)
	if msg == winit.events.CloseRequested then
		window:close()
		handle.stop()
	elseif msg ~= winit.events.None then
		print(msg)
	end
end)

winit.run()
