local wry = require("@luneweb/wry")
local window, handle = wry.create_window({}), nil

wry.create_webview(window, {
	custom_protocol_name = "lune",
	custom_protocol_handler = function(req)
		print(req.headers.print, req.headers.time)

		return "Hello!"
	end,
	url = "http://lune.app/",
	headers = {
		print = "Hey!",
		time = tostring(os.clock())
	}
})

handle = wry.event_loop(window, function(msg)
	if msg == wry.events.CloseRequested then
		window:close()
		handle.stop()
	end
end)

wry.run()
