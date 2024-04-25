export type BreakLoop = "Identifier.BreakLoop"

--[=[
	@class Winit

	Built-in library for creating applications

	### Example usage

	```lua
	local winit = require("@luneweb/winit")
	local window = winit.create_window()

	winit.event_loop(window, function(break_loop, msg)
		if event == winit.events.CloseRequested then
			window:close()
			return break_loop
		end
	end)

    winit.run()
	```
]=]
local winit = {}

--[=[
	@interface Window
	@within Winit
]=]
export type Window = {
	close: (self: Window) -> (),
	get_webview: (self: Window) -> WebView?,
}

--[=[
	@interface Window
	@within Winit
]=]
export type WebView = {}

--[=[
	@interface Window
	@within Winit
]=]
export type WebViewConfig = {
	init_script: string?,
	url: string,
}

--[=[
	@within Winit

	A Window Event
]=]
export type WindowEvent = {
	get_keycode: (self: WindowEvent) -> string?,
}

--[=[
	@within Winit

	A collection of the Window Events that are received from the event loop

	* `CloseRequested`
	* `KeyCode`
	* `None`
]=]
winit.events = {
	CloseRequested = (nil :: any) :: WindowEvent,
	KeyCode = (nil :: any) :: WindowEvent,
	None = (nil :: any) :: WindowEvent,
}

--[=[
	@within Winit

	### Example usage

	```lua
	winit.event_loop(window, function(break_loop, msg)
		if event == winit.events.CloseRequested then
			window:close()
			return break_loop
		end
	end)
	```
]=]
winit.event_loop = function(
	window: Window,
	callback: (break_loop: BreakLoop, event: WindowEvent) -> BreakLoop?
): Window
	return nil :: any
end

--[=[
	@within Winit

	Creates and returns a Window
	```
]=]
winit.create_window = function(): Window
	return nil :: any
end

--[=[
	@within Winit

	Creates and returns a WebView
	```
]=]
winit.create_webview = function(window: Window, config: WebViewConfig): WebView
	return nil :: any
end

--[=[
	@within Winit

	Start the event loop

    ### Warning !

    Make sure to use this after having atleast 1 event loop connection

    ### Example usage

    ```lua
    winit.event_loop(window, function(break_loop, msg)
		-- handle events ...
	end)

    winit.run()
	```
]=]
winit.run = function(): Window
	return nil :: any
end

return winit
