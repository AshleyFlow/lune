export type WindowEvent = "Enum.WindowEvent"

--[=[
	@class Window

	Built-in library for creating applications

	### Example usage

	```lua
	local window = require("@lune/window")
	local my_window = window.new({
		title = "Hello, World!",
	})

	while true do
		local event = my_window:process_events()

		if event == window.events.Exit then
			-- Exit
			my_window:set_visible(false)
			break
		end
	end
	```
]=]
local webview = {}

--[=[
	@interface Window
	@within Webview

	* `process_events` (callback: (WindowEvent) -> boolean) -> void
	* `run_script` (script: string, callback: (string) -> void?) -> void
	* `set_visible` (visible: boolean) -> void
]=]
export type Window = {
	--[=[
		Will run your callback in a loop
		
		Return true in the callback to break the loop
	]=]
	process_events: (callback: (WindowEvent) -> boolean) -> (),
	run_script: (script: string, callback: (string) -> ()?) -> (),
	set_visible: (visible: boolean) -> (),
}

--[=[
	@interface WindowConfig
	@within Webview

	* `reload_script` string
	* `title` string
	* `html` string
	* `url` string
]=]
export type WindowConfig = {
	reload_script: string?,
	title: string?,
	html: string?,
	url: string?,
}

--[=[
	@within Webview

	Events received from window loop event
]=]
webview.events = {
	Nothing = (nil :: any) :: WindowEvent,
	Redraw = (nil :: any) :: WindowEvent,
	Exit = (nil :: any) :: WindowEvent,
}

webview.new = function(config: WindowConfig): Window
	return nil :: any
end

return webview
