local wry = require("@luneweb/wry")
local window = wry.create_window({
	title = "[experimental] luneweb",
})

local pixels_size = {
	x = 64,
	y = 64,
}

local pixels = wry.create_pixels(window, pixels_size.x, pixels_size.y)
local handle: wry.Connection
local draw = false

local mouse_pos: wry.Dimension?
local old_size: wry.Dimension?
local start: wry.Dimension?

handle = wry.event_loop(window, function(msg)
	if msg.event_type == "CloseRequested" then
		window:close()
		handle.stop()
	elseif msg.event_type == "MouseButton" then
		if msg.mousebutton ~= "left" then
			return
		end

		draw = msg.pressed

		if not draw then
			start = nil
		end
	elseif msg.event_type == "CursorMoved" then
		mouse_pos = pixels:window_to_pixel_coordinates(msg.position)
	else
		local new_size = window.size

		if new_size ~= old_size then
			pixels:resize(new_size.x, new_size.y)
			pixels:render()

			old_size = new_size
		end
	end

	if draw and mouse_pos then
		if start == nil then
			start = mouse_pos
		end

		pixels:clear_frame()

		pixels:draw_rectangle(start, mouse_pos, {
			r = 255,
			g = 255,
			b = 255,
			a = 255,
		})

		pixels:render()
	end
end)
