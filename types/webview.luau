export type WindowEvent = "Enum.WindowEvent"

--[=[
	@class Webview

	Built-in library for creating applications

	### Example usage

	```lua
	local webview = require("@lune/webview")
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

	* `event_loop` ( callback: ( WindowId, WindowEvent ) ) -> ()
]=]
local webview = {}

--[=[
	@interface Window
	@within Webview

	* `run_script` (self, script: string, callback: (string) -> void?) -> void
	* `set_visible` (self, visible: boolean) -> void
]=]
export type Window = {
	run_script: (self: Window, script: string, callback: (string) -> ()?) -> (),
	set_visible: (self: Window, visible: boolean) -> (),
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
	Exit = (nil :: any) :: WindowEvent,
}

--[=[
	@within Webview
]=]
webview.event_loop = function(callback: (window_id: unknown, event: WindowEvent) -> boolean): Window
	return nil :: any
end

webview.new = function(config: WindowConfig): Window
	return nil :: any
end

return webview
