local winit = require("@luneweb/winit")
local window = winit.create_window({})

local handle
handle = winit.event_loop(window, function(msg)
	if msg == winit.events.CloseRequested then
		window:close()
		handle.stop()
	end
end)

winit.run()
