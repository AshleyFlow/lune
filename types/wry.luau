export type Dimension = {
	x: number,
	y: number,
}

export type HttpMethod = "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "OPTIONS" | "PATCH"

--[=[
	@interface ServeRequest
	@within Net

	Data type for requests in `net.serve`.

	This is a dictionary containing the following values:

	* `path` - The path being requested, relative to the root. Will be `/` if not specified
	* `query` - A table of key-value pairs representing query parameters in the request path
	* `method` - The HTTP method verb, such as `"GET"`, `"POST"`, `"PATCH"`, `"PUT"`, or `"DELETE"`. Will always be uppercase
	* `headers` - A table of key-value pairs representing headers
	* `body` - The request body, or an empty string if one was not given
]=]
export type ServeRequest = {
	path: string,
	query: { [string]: string? },
	method: HttpMethod,
	headers: { [string]: string },
	body: string,
}

--[=[
	@interface ServeResponse
	@within Net

	Response type for requests in `net.serve`.

	This is a dictionary that may contain one or more of the following values:

	* `status` - The status code for the request, in the range `100` -> `599`
	* `headers` - A table of key-value pairs representing headers
	* `body` - The response body
]=]
export type ServeResponse = {
	status: number?,
	headers: { [string]: string }?,
	body: (string | buffer)?,
}

--[=[
	@class Wry

	Built-in library for creating applications

	### Example usage

	```lua
	local wry = require("@luneweb/wry")
	local window = wry.create_window()

	wry.event_loop(window, function(break_loop, msg)
		if event == wry.events.CloseRequested then
			window:close()
			return break_loop
		end
	end)

    wry.run()
	```
]=]
local wry = {}

--[=[
	@interface Window
	@within Wry
]=]
export type Window = {
	close: (self: Window) -> (),

	webview: WebView?,
	size: Dimension,
}

--[=[
	@interface WindowConfig
	@within Wry
]=]
export type WindowConfig = {
	title: string?,
}

--[=[
	@interface WebView
	@within Wry
]=]
export type WebView = {
	--[=[
		Runs the provided javascript code inside the WebView, and returns the result as a lua value by using serde.decode

		### Warning

		Make sure to only use this after calling `wry.run()`, otherwise it will yield forever.

		### Example Usage

		```lua
		wry.run()

		local result = webview:evaluate('("b" + "a" + +"a" + "a").toLowerCase()')
		print(result) -- "banana"
		```
	]=]
	evaluate: (self: WebView, javascript: string) -> unknown,
	ipc_handler: (self: WebView, callback: (message: unknown) -> ()) -> WebViewIPCHandle,
}

--[=[
	@interface WebViewConfig
	@within Wry
]=]
export type WebViewConfig = {
	init_script: string?,
	html: string?,
	url: string?,
	custom_protocol_name: string?,
	custom_protocol_handler: ((request: ServeRequest) -> ServeResponse | string)?,
}

--[=[
	@within Wry

	A Window Event
]=]
export type WindowEvent = {
	mousebutton: string?,
	keycode: string?,
	pressed: boolean?,
	position: Dimension?,
}

--[=[
	@within Wry

	The handler returend by creating a ipc handler
]=]
export type WebViewIPCHandle = {
	stop: () -> (),
}

--[=[
	@within Wry

	The handler returend by creating a event loop
]=]
export type WindowEventLoopHandle = {
	stop: () -> (),
}

--[=[
	@within Wry

	A collection of the Window Events that are received from the event loop
	
	### Warning

	Input related events are not supported for Windows containing a WebView, this is due to how Browsers work

	* `CloseRequested`
	* `MouseButton`
	* `KeyCode`
	* `CursorMoved`
	* `None`
]=]
wry.events = {
	CloseRequested = (nil :: any) :: WindowEvent,
	MouseButton = (nil :: any) :: WindowEvent,
	KeyCode = (nil :: any) :: WindowEvent,
	CursorMoved = (nil :: any) :: WindowEvent,
	None = (nil :: any) :: WindowEvent,
}

--[=[
	@within Wry

	### Example usage

	```lua
	wry.event_loop(window, function(break_loop, msg)
		if event == wry.events.CloseRequested then
			window:close()
			return break_loop
		end
	end)
	```
]=]
wry.event_loop = function(
	window: Window,
	callback: (event: WindowEvent) -> ()
): WindowEventLoopHandle
	return nil :: any
end

--[=[
	@within Wry

	Creates and returns a Window
	```
]=]
wry.create_window = function(config: WindowConfig): Window
	return nil :: any
end

--[=[
	@within Wry

	Creates and returns a WebView
	```
]=]
wry.create_webview = function(window: Window, config: WebViewConfig): WebView
	return nil :: any
end

--[=[
	@within Wry

	Start the event loop

    ### Warning !

    Make sure to use this after having atleast 1 event loop connection

    ### Example usage

    ```lua
    wry.event_loop(window, function(break_loop, msg)
		-- handle events ...
	end)

    wry.run()
	```
]=]
wry.run = function(): Window
	return nil :: any
end

return wry
