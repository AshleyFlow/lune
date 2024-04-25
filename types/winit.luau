export type WindowEvent = "Enum.WindowEvent"
export type BreakLoop = "Identifier.BreakLoop"

--[=[
	@class Winit

	Built-in library for creating applications

	### Example usage

	```lua
	local winit = require("@luneweb/winit")
	local window = winit.new()

	winit.event_loop(window, function(break_loop, msg)
		if event == winit.events.CloseRequested then
			window:close()
			return break_loop
		end
	end)

    winit.run()
	```
]=]
local winit = {}

--[=[
	@interface Window
	@within Winit
]=]
export type Window = {
	close: (self: Window) -> (),
}

--[=[
	@within Winit

	Events received from the event loop
]=]
winit.events = {
	CloseRequested = (nil :: any) :: WindowEvent,
	None = (nil :: any) :: WindowEvent,
}

--[=[
	@within Winit

	### Example usage

	```lua
	winit.event_loop(window, function(break_loop, msg)
		if event == winit.events.CloseRequested then
			window:close()
			return break_loop
		end
	end)
	```
]=]
winit.event_loop = function(
	window: Window,
	callback: (break_loop: BreakLoop, event: WindowEvent) -> BreakLoop?
): Window
	return nil :: any
end

--[=[
	@within Winit

	Creates and returns a Window
	```
]=]
winit.new = function(): Window
	return nil :: any
end

--[=[
	@within Winit

	Start the event loop

    ### Warning !

    Make sure to use this after having atleast 1 event loop connection

    ### Example usage

    ```lua
    winit.event_loop(window, function(break_loop, msg)
		-- handle events ...
	end)

    winit.run()
	```
]=]
winit.run = function(): Window
	return nil :: any
end

return winit
