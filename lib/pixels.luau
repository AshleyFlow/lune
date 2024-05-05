--[=[
	@class Pixels

	### Example

	```lua
	local task = require("@luneweb/task")
	local pixels = Wry.create_pixels(window, 64, 64)

	while true do
		pixels:render()
		task.wait()
	end
	```

	# Caution

	always use the [Pixels:render] method after modifying the frame in any kind of way
	
	even the built-in methods like [Pixels:draw_pixel] and [Pixels:resize] don't render automatically
]=]

--[=[
	@prop frame buffer
	@within Pixels
]=]

--[=[
	@method clear_frame
	@within Pixels
]=]

--[=[
	@method resize
	@within Pixels

	This method is recommened to use when window gets resized, so the pixels can maintain their original aspect ratio.

	### Use with caution

	This method will lead to crashes if used repeatedly, so only use this method when window gets resized

	@param width number -- should be set to window width
	@param height number -- should be set to window height
]=]

--[=[
	@method window_to_pixel_coordinates
	@within Pixels

	Turn a position relative to window, into pixel coordinates.

	@param window_pos Dimension
	@return Dimension
]=]

--[=[
	@method render
	@within Pixels

	Update's the internal buffer to use the new [Pixels.frame], and then renders it onto the screen.
]=]

--[=[
	@method _mutate_frame
	@within Pixels
	
	This method has very bad performance, use [Pixels:mutate_frame] instead.

	@param handler (i: number, pixel: buffer) -> ()
]=]

--[=[
	@method mutate_frame
	@within Pixels

	It's recommend to modify [Pixels.frame] and use [Pixels:render] instead.

	@param frame buffer
]=]

--[=[
	@method draw_pixel
	@within Pixels

	@param position Dimension
	@param rgba RGBA
]=]

--[=[
	@method draw_line
	@within Pixels

	@param position1 Dimension
	@param position2 Dimension
	@param rgba RGBA
]=]

--[=[
	@method draw_rectangle
	@within Pixels

	@param position1 Dimension
	@param position2 Dimension
	@param rgba RGBA
]=]

return {}
