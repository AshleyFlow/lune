local wry = require("@luneweb/wry")

local window = wry.create_window({
	title = "[experimental] luneweb",
})

local pixels_size = {
	x = 1024 * 4,
	y = 720 * 4,
}

local pixels = wry.create_pixels(window, pixels_size.x, pixels_size.y)
local handle: wry.Connection

local old_size: wry.Dimension?
local old_pos: wry.Dimension?

handle = wry.event_loop(window, function(msg)
	if msg.event_type == "CloseRequested" then
		window:close()
		handle.stop()
	elseif msg.event_type == "CursorMoved" then
		local size = window.size

		if old_size and size ~= old_size then
			pixels:resize(window.size.x, window.size.y)
			old_size = size
		end

		local position = {
			x = math.floor(msg.position.x / size.x * pixels_size.x),
			y = math.floor(msg.position.y / size.y * pixels_size.y),
		}

		if old_pos then
			pixels:draw_line(old_pos, position, {
				r = 255,
				g = 255,
				b = 0,
				a = 255 / 2,
			})
		else
			pixels:draw_pixel(position, {
				r = 255,
				g = 255,
				b = 0,
				a = 255 / 2,
			})
		end

		old_pos = position

		pixels:render()
	end
end)
