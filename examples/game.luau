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
local old_size: wry.Dimension
local frame = buffer.create(16384)

handle = wry.event_loop(window, function(msg)
	if msg.event_type == "CloseRequested" then
		window:close()
		handle.stop()
	elseif msg.event_type == "CursorMoved" then
		local started = os.clock()
		local size = window.size

		if old_size and size ~= old_size then
			pixels:resize(window.size.x, window.size.y)
		end

		old_size = size

		local position = {
			x = math.floor(msg.position.x / size.x * pixels_size.x),
			y = math.floor(msg.position.y / size.y * pixels_size.y),
		}

		pixels:mutate_frame(function()
			for i = 0, buffer.len(frame) - 1, 4 do
				local x = math.floor(math.floor(i / 4) % pixels_size.x)
				local y = math.floor(math.floor(i / 4) / pixels_size.x)

				local same_x = x == position.x
				local same_y = y == position.y

				if same_x or same_y then
					buffer.writeu8(frame, i + 0, 255)
					buffer.writeu8(frame, i + 1, 255)
					buffer.writeu8(frame, i + 2, 255)
					buffer.writeu8(frame, i + 3, if same_x and same_y then 255 else 125)
				else
					buffer.writeu8(frame, i + 0, 75)
					buffer.writeu8(frame, i + 1, 75)
					buffer.writeu8(frame, i + 2, 75)
					buffer.writeu8(frame, i + 3, 255)
				end
			end

			return frame
		end)

		pixels:render()

		local finished = os.clock()
		print("took", finished - started)
	end
end)
