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
]=]

--[=[
	@prop frame buffer
	@within Pixels
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

return {}
