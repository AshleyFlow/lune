--!strict
local webview = require("@luneweb/webview")

--[=[
	reload_script in WindowConfig only runs when the page reloads

	so we have to run it manually when window gets created
]=]
local init_script = 'document.body.style.backgroundColor = "black"'
local my_window = webview.new({
	--[[
		reload_script runs before window loads
		so we have to do our stuff in the onload event
	]]
	reload_script = `window.onload = () => {init_script}`,
})

my_window:run_script(init_script)
my_window:process_events(function(event)
	if event == webview.events.Exit then
		my_window:set_visible(false)

		-- returning true will break the loop
		return true
	end

	return false
end)
