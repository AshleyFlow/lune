--!nocheck
--!nolint

local task = require("@luneweb/task")
local winit = require("@luneweb/winit")
local messages = 0
local window = winit.new()

winit.event_loop(function(break_loop, msg)
	print(msg == winit.events.None)

	if messages % 500 == 0 and messages > 0 then
		print("received 500 messages")
		return break_loop
	end

	messages += 1
end)

winit.run()
