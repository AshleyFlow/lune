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
	@method evaluate_noresult
	@within WebView

	Similar to `WebView:evaluate(...)`, but does not yield and does not return any result

	### Bug(s)
	* Will not work if you run it before calling wry.run()

	@param script string
]=]

--[=[
	@method evaluate_callback
	@within WebView

	Similar to `WebView:evaluate(...)`, but instead of yielding to return result, it passes the result to the callback function

	### Bug(s)
	* Will not work if you run it before calling wry.run()

	@param script string
	@param callback (result: unknown) -> ()
]=]

--[=[
	@yields
	@method evaluate
	@within WebView

	Runs the provided javascript code through the webview and returns the last statement as a lua value

	### Bug(s)
	* Will yield forever if you run it before calling wry.run()
	* Will yield forever if you run it while custom protocol is sending response to webview for **loading a page**

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
	@return Connection
]=]

--[=[
	@method load_url
	@within WebView

	Navigate to the specified url

	@param url string
]=]

--[=[
	@error "<parsed-header> has invalid header value." -- Occurs when headers contains more than 1 header value
	@method load_url_with_headers
	@within WebView

	Navigate to the specified url using the specified headers

	@param url string
	@param headers WebViewHeaders
]=]

return {}
