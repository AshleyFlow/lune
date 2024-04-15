--!strict
local fs = require("@lune/fs")
local net = require("@lune/net")
local webview = require("@lune/webview")

local port = net.findAvailablePort("127.0.0.1")
local debug_dir: string = (debug.info :: any)(1, "sln")
local __dirname = string.sub(debug_dir, 12, #debug_dir - 2 - 4)

local app = webview.build({})

local handle = net.serve(port, function(req)
	local dir = __dirname .. req.path

	if req.path == "/" then
		app:executeJavascript([[
			let element = document.createElement('h1')
			element.innerHTML = 'Hello, World!'
			document.body.appendChild(element)
			
			console.log('Hello, World!')
		]])

		return {
			body = fs.isFile(dir) and fs.readFile(dir) or nil,
		}
	end

	return "Hello, World!"
end)

app:onExit(function()
	handle.stop()
end)

app:loadUrl(string.format("http://localhost:%d/", port))
app:openDevtools()
-- app:closeDevtools()
-- app:exit()

print(string.format("Opened server on port: %d", port))
