local winit = require("@luneweb/winit")

local window = winit.create_window({
	title = "Hello, Lune!",
})

print(window:get_webview())

winit.event_loop(window, function(break_loop, msg)
	if msg == winit.events.CloseRequested then
		window:close()
		return break_loop
	elseif msg == winit.events.KeyCode then
		print(msg:get_keycode(), msg:get_pressed())
	elseif msg == winit.events.MouseButton then
		print(msg:get_mousebutton(), msg:get_pressed())
	elseif msg == winit.events.CursorMoved then
		print(msg:get_position())
	end

	return nil
end)

local window = winit.create_window({})
local init_script = [[
	document.addEventListener('DOMContentLoaded', function() {
		document.body.style.transform = 'translateX(250px)'
	})
]]

local webview = winit.create_webview(window, {
	init_script = init_script,
	url = "https://tauri.app",
})

print(window:get_webview())

winit.event_loop(window, function(break_loop, msg)
	if msg == winit.events.CloseRequested then
		window:close()
		return break_loop
	end

	return nil
end)

winit.run()
