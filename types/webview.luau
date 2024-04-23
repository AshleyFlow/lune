export type WindowEvent = "Enum.WindowEvent"

--[=[
	@class Webview

	Built-in library for creating applications

	### Example usage

	```lua
	local webview = require("@lune/webview")
	local window = webview.new({
		title = "Hello, World!",
	})

	webview.event_loop(function(id, event)
		if event == window.events.Exit and id == window.id then
			-- Exit
			window:set_visible(false)
			return true
		end

		return false
	end)
	```

	* `event_loop` ( callback: ( WindowId, WindowEvent ) -> boolean ) -> void
]=]
local webview = {}

--[=[
	@interface WindowId
	@within Webview
]=]
export type WindowId = {}

--[=[
	@interface Window
	@within Webview

	* `run_script` (self, script: string, callback: (string) -> void?) -> void
	* `set_visible` (self, visible: boolean) -> void
]=]
export type Window = {
	run_script: (self: Window, script: string, callback: (string) -> ()?) -> (),
	set_visible: (self: Window, visible: boolean) -> (),
	id: WindowId,
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

	### Warning!
	
	Using more than one event_loop will result in unexpected behaviour

	### Example usage

	Return true in the callback function to exit the loop

	```lua
	webview.event_loop(function(id, event)
		if event == window.events.Exit and id == window.id then
			-- Exit
			window:set_visible(false)
			return true
		end

		return false
	end)
	```
]=]
webview.event_loop = function(
	callback: (window_id: WindowId, event: WindowEvent) -> boolean
): Window
	return nil :: any
end

webview.new = function(config: WindowConfig): Window
	return nil :: any
end

return webview
