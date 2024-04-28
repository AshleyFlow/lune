local wry = require("@luneweb/wry")
local window, handle = wry.create_window({}), nil

wry.create_webview(window, {
	custom_protocol_name = "lune",
	custom_protocol_handler = function(_req)
		return {
			body = "Hey!",
			headers = {
				["Access-Control-Allow-Origin"] = "*",
				["Access-Control-Allow-Headers"] = "*",
			},
		}
	end,
	url = "http://lune.app/",
})

handle = wry.event_loop(window, function(msg)
	if msg == wry.events.CloseRequested then
		window:close()
		handle.stop()
	end
end)

wry.run()
