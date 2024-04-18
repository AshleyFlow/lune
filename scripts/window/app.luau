local task = require("@lune/task")
local window = require("@lune/window")

local my_window = window.new({
	title = "My window",
})

my_window:run_script("console.log('Hello, World!')")

my_window:run_script("'50' + 150", function(unknown)
	print("'50' + 150 =", unknown)
end)

task.spawn(function()
	while true do
		local event = my_window:process_events()

		if event == window.events.Exit then
			my_window:set_visible(false)
			break
		end
	end
end)

for i = 10, 0, -1 do
	task.wait(1)
	print(i)
end
