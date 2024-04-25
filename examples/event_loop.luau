--!nocheck
--!nolint

local task = require("@luneweb/task")
local winit = require("@luneweb/winit")

local window = winit.new()

winit.event_loop(window, function(break_loop, msg)
	if msg == winit.events.CloseRequested then
		window:close()
		return break_loop
	end
end)

local window = winit.new()

winit.event_loop(window, function(break_loop, msg)
	if msg == winit.events.CloseRequested then
		window:close()
		return break_loop
	end
end)

winit.run()
