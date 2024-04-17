local window = require("@lune/window")

local my_window = window.new()

my_window:run_script("console.log('Hello, World!')")

require("@lune/task").spawn(function()
	while true do
		local event = my_window:process_events()

		if event == window.events.Exit then
			break
		end
	end
end)

print("1")

require("@lune/task").wait(1)

print("2")
