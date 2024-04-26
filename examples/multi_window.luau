local winit = require("@luneweb/winit")

local window1, handle1 = winit.create_window({}), nil
local window2, handle2 = winit.create_window({
	title = "Tauri",
}), nil

handle1 = winit.event_loop(window1, function(msg)
	if msg == winit.events.CloseRequested then
		window1:close()
		handle1.stop()
	end
end)

handle2 = winit.event_loop(window2, function(msg)
	if msg == winit.events.CloseRequested then
		window2:close()
		handle2.stop()
	end
end)

local init_script = [[
	document.addEventListener('DOMContentLoaded', function() {
		document.body.style.transform = 'translateX(250px)'
	})
]]

local webview = winit.create_webview(window2, {
	url = "https://tauri.app/",
	init_script = init_script,
})

winit.run()

print(webview:evaluate('("b" + "a" + +"a" + "a").toLowerCase()'))
