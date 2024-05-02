--[=[
	@class Window

	### Example

	```lua
	local window = Wry.create_window({})
	local handle

	handle = Wry.event_loop(window, function(msg)
		if msg == "CloseRequested" then
			window:close()
			handle.stop()
		end
	end)
	```
]=]

--[=[
	@method close
	@within Window

	Closes the window
]=]

--[=[
	@readonly
	@prop webview WebView?
	@within Window

	This property keeps the attached webview from being garbage collected
]=]

--[=[
	@readonly
	@prop size Dimension
	@within Window

	* Does not account for scrollbar in the webview

	### WebView alternative

	```lua
	local size: Dimension = window.webview:evaluate("{ x: window.innerWidth, y: window.innerHeight }")
	```
]=]

return {}
