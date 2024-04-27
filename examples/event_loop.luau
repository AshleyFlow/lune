local wry = require("@luneweb/wry")
local window = wry.create_window({})
local webview = wry.create_webview(window, {
	url = "https://tauri.app/",
})

local ipc
ipc = webview:ipc_handler(function(msg)
	print(msg)
end)

local handle
handle = wry.event_loop(window, function(msg)
	if msg == wry.events.CloseRequested then
		window:close()
		ipc.stop()
		handle.stop()
	end
end)

wry.run()
