--[=[
	@class Window

	Built-in library for creating applications
]=]
local webview = {}

export type Window = {
	process_events: (self: Window) -> typeof(webview.events),
	run_script: (self: Window, script: string) -> (),
}

webview.events = {
	Nothing = nil :: unknown,
	Redraw = nil :: unknown,
	Exit = nil :: unknown,
}

webview.new = function(): Window
	return nil :: any
end

return webview
