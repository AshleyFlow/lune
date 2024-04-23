local task = require("@luneweb/task")
local webview = require("@luneweb/webview")

local window1 = webview.new({
	title = "Main Window",
})

local window2 = webview.new({
	title = "Popup Window",
})

webview.event_loop(function(id, event)
	if event == webview.events.Exit then
		if id == window1.id then
			window1.set_visible(false)
			window2.set_visible(false)
			return true
		else
			window2.set_visible(false)
		end
	end

	return false
end)

print("Hello, World!")

task.wait(1)

print("Everything is working!")
