local wry = require("@luneweb/wry")
local window, handle = wry.create_window({}), nil

local webview = wry.create_webview(window, {
	custom_protocol_name = "lune",
	custom_protocol_handler = function(request)
		if request.path == "/home" then
			return "Welcome!"
		end

		return "Hello, World!"
	end,
	url = "http://lune.app/home",
})

handle = wry.event_loop(window, function(msg)
	if msg == wry.events.CloseRequested then
		window:close()
		handle.stop()
	end
end)

wry.run()

require("@luneweb/task").delay(1, function()
	webview:load_url("http://lune.app/")
end)
