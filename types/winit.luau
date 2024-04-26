export type Dimension = {
	x: number,
	y: number,
}

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

	webview: WebView?,
	size: Dimension,
}

--[=[
	@interface WindowConfig
	@within Winit
]=]
export type WindowConfig = {
	title: string?,
}

--[=[
	@interface WebView
	@within Winit
]=]
export type WebView = {
	--[=[
		Runs the provided javascript code inside the WebView, and returns the result as a lua value by using serde.decode

		### Warning

		Make sure to only use this after calling `winit.run()`, otherwise it will yield forever.

		### Example Usage

		```lua
		winit.run()

		local result = webview:evaluate('("b" + "a" + +"a" + "a").toLowerCase()')
		print(result) -- "banana"
		```
	]=]
	evaluate: (self: WebView, javascript: string) -> unknown,
}

--[=[
	@interface WebViewConfig
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
	mousebutton: string?,
	keycode: string?,
	pressed: boolean?,
	position: Dimension?,
}

--[=[
	@within Winit

	The handler returend by creating a event loop
]=]
export type WindowEventLoopHandle = {
	stop: () -> (),
}

--[=[
	@within Winit

	A collection of the Window Events that are received from the event loop
	
	### Warning

	Input related events are not supported for Windows containing a WebView, this is due to how Browsers work

	* `CloseRequested`
	* `MouseButton`
	* `KeyCode`
	* `CursorMoved`
	* `None`
]=]
winit.events = {
	CloseRequested = (nil :: any) :: WindowEvent,
	MouseButton = (nil :: any) :: WindowEvent,
	KeyCode = (nil :: any) :: WindowEvent,
	CursorMoved = (nil :: any) :: WindowEvent,
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
	callback: (event: WindowEvent) -> ()
): WindowEventLoopHandle
	return nil :: any
end

--[=[
	@within Winit

	Creates and returns a Window
	```
]=]
winit.create_window = function(config: WindowConfig): Window
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
