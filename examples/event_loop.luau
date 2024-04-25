local winit = require("@luneweb/winit")

local window = winit.create_window()

print(window:has_webview())

winit.event_loop(window, function(break_loop, msg)
	if msg == winit.events.CloseRequested then
		window:close()
		return break_loop
	end

	return nil
end)

local window = winit.create_window()
local webview = winit.create_webview(window)

print(window:has_webview())
print(window:get_webview())

winit.event_loop(window, function(break_loop, msg)
	if msg == winit.events.CloseRequested then
		window:close()
		return break_loop
	end

	return nil
end)

winit.run()
