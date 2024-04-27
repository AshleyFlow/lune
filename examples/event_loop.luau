local winit = require("@luneweb/winit")
local window = winit.create_window({})
local webview = winit.create_webview(window, {
	url = "https://tauri.app/",
})

local ipc
ipc = webview:ipc_handler(function(msg)
	print(msg)
end)

local handle
handle = winit.event_loop(window, function(msg)
	if msg == winit.events.CloseRequested then
		window:close()
		ipc.stop()
		handle.stop()
	end
end)

winit.run()
