--[=[
	@class Wry

	### Built-in library for creating web applications.

	```lua
	local Wry = require("@luneweb/wry")
	```

	## Example

	```lua
	local window = Wry.create_window({
		title = "Hello, Lune!"
	})

	local handle
	handle = Wry.event_loop(window, function(msg)
		if msg == "CloseRequested" then
			handle.stop()
		end
	end)

	Wry.run()
	```
]=]

--[=[
	@interface Dimension
	@within Wry
	.x number
	.y number

	2D Dimension used by Wry for mouse position
]=]

--[=[
	@interface WindowConfig
	@within Wry
	.title string?
	.size Dimension? -- ? default window size represented in physical pixels
]=]

--[=[
	@function create_window
	@within Wry

	@param config WindowConfig
	@return Window

	## Example

	```lua
	local window = Wry.create_window({
		title = "Hello, Lune!"
	})
	```
]=]

--[=[
	@type WebViewHeaders { [string]: string }
	@within Wry

	Set headers used when loading the requested url

	### Limitation
	
	#### Will error if it contains more than 1 header value
	
	Not sure why this happens, but after lots of debugging I'm certain it's an issue with wry and not luneweb
]=]

--[=[
	@interface WebViewConfig
	@within Wry
	.with_devtools boolean?
	.init_script string?
	.html string?
	.url string?
	.headers WebViewHeaders? -- ! having more than 1 header value will error
	.custom_protocols { [string]: HttpHandler }?
]=]

--[=[
	@function create_webview
	@within Wry

	@param config WebViewConfig
	@return WebView

	## Example

	```lua
	Wry.create_webview(window, {
		html = "<h1>Hello, Lune!</h1>"
	})
	```
]=]

--[=[
	@interface WindowEvent
	@within Wry
	.event_type string
	.mousebutton string?
	.keycode string?
	.pressed boolean?
	.position Dimension?
]=]

--[=[
	@interface Connection
	@within Wry
	.stop () -> ()
	.is_running () -> boolean
]=]

--[=[
	@function event_loop
	@within Wry
	
	### Example

	```lua
	local handle
	handle = Wry.event_loop(window, function(msg)
		if msg == "CloseRequested" then
			window:close()
			handle.stop()
		elseif msg == "KeyCode" then
			print(msg)
		elseif msg == "MouseButton" then
			print(msg)
		elseif msg == "CursorMoved" then
			print(msg)
		end
	end)
	```

	@param window Window
	@param callback (WindowEvent) -> ()
	@return Connection
]=]

--[=[
	@function run
	@within Wry

	Runs the main event loop of your application

	## Requirement

	### Must be run when there is atleast one active event loop connection
]=]

return {}
