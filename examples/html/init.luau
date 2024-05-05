local fs = require("@luneweb/fs")
local wry = require("@luneweb/wry")
local window, handle = wry.create_window({}), nil

wry.create_webview(window, {
	html = fs.readFile("./examples/html/index.html"),
})

handle = wry.event_loop(window, function(msg)
	if msg.event_type == "CloseRequested" then
		window:close()
		handle.disconnect()
	end
end)
