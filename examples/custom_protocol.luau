local wry = require("@luneweb/wry")
local window, handle = wry.create_window({}), nil

local webview = wry.create_webview(window, {
	with_devtools = true,
	custom_protocols = {
		lune = function(_req)
			return {
				body = "Hey (1)!",
				headers = {
					["Access-Control-Allow-Origin"] = "*",
					["Access-Control-Allow-Headers"] = "*",
				},
			}
		end,
		luneweb = function(_req)
			return {
				body = "Hey (2)!",
				headers = {
					["Access-Control-Allow-Origin"] = "*",
					["Access-Control-Allow-Headers"] = "*",
				},
			}
		end,
	},
	url = "http://lune.app/",
})

handle = wry.event_loop(window, function(msg)
	if msg.event_type == "CloseRequested" then
		window:close()
		handle.stop()
	end
end)

require("@luneweb/task").wait(2)

webview:load_url("http://luneweb.app/")
