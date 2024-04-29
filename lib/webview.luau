--[=[
	@class WebView

	### Example

	```lua
	local webview = Wry.create_webview(window, {
		custom_protocols = {
			lune = function(request)
				if request.path == "/print" then
					print("Hello, Lune!")
				end

				return "Hello, Lune!"
			end,
		},
		url = "http://lune.app/"
	})

	webview:evaluate("fetch('http://lune.app/print')")
	```
]=]

--[=[
	@interface WebViewIPCHandle
	@within WebView
	.stop () -> ()
]=]

--[=[
	@yields
	@method evaluate
	@within WebView

	Runs the provided javascript code through the webview and returns the last statement as a lua value

	### Example

	```lua
	local result = webview:evaluate([[
		let value = { a: 5000 };
		value.a += 1000;

		value
	]])

	print(result.a) -- 6000
	```

	@param script string
	@return unknown
]=]

--[=[
	@method ipc_handler
	@within WebView

	Receive messages from javascript

	* Messages will automatically be converted into luau values

	### Example

	```lua
	-- luau
	webview:ipc_handler(function(message)
		print(message.print_this) -- "Hello, World!"
	end)
	```

	```js
	// javascript
	window.luneweb.postMessage({ print_this: 'Hello, World!' })
	```

	@param callback ( message: unknown ) -> void
	@return WebViewIPCHandle
]=]

--[=[
	@method load_url
	@within WebView

	Navigate to the specified url

	@param url string
]=]

--[=[
	@method load_url_with_headers
	@within WebView

	Navigate to the specified url using the specified headers

	@param url string
	@param headers WebViewHeaders -- ! having more than 1 header value will error
]=]

return {}
