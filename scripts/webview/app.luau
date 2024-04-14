--!strict
local fs = require("@lune/fs")
local net = require("@lune/net")
local webview = require("@lune/webview")

local port = net.findAvailablePort("127.0.0.1")
local debug_dir: string = (debug.info :: any)(1, "sln")
local __dirname = string.sub(debug_dir, 12, #debug_dir - 2 - 4)

local files = {
	["/"] = "/index.html",
}

local handle = net.serve(port, function(req)
	local dir = __dirname .. (files[req.path] or (req.path or "/"))

	return {
		body = fs.isFile(dir) and fs.readFile(dir) or nil,
	}
end)

local app = webview.build({
	exit = function()
		handle.stop()
	end,
	url = string.format("http://localhost:%d/", port),
})

app:open_devtools()
-- app:close_devtools()
-- app:load_url("https://www.roblox.com/")
-- app:exit()

print(string.format("Opened server on port: %d", port))
