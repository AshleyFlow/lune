local wry = require("@luneweb/wry")

local window1, handle1 = wry.create_window({}), nil
local window2, handle2 =
	wry.create_window({
		title = "Tauri",
		size = {
			x = 500,
			y = 750,
		},
	}), nil

handle1 = wry.event_loop(window1, function(msg)
	if msg == wry.events.CloseRequested then
		window1:close()
		handle1.stop()
	end
end)

handle2 = wry.event_loop(window2, function(msg)
	if msg == wry.events.CloseRequested then
		window2:close()
		handle2.stop()
	end
end)

local webview = wry.create_webview(window2, {
	url = "https://tauri.app/",
	init_script = [[
		window.onload = () => {
			document.body.style.transform = 'translateX(250px)'
		}
	]],
})

wry.run()

print(webview:evaluate('("b" + "a" + +"a" + "a").toLowerCase()'))
