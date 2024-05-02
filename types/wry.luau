local http = require("http")

export type Dimension = {
	x: number,
	y: number,
}

local wry = {}

export type Window = {
	close: (self: Window) -> (),

	webview: WebView?,
	size: Dimension,
}

export type WindowConfig = {
	title: string?,
	size: Dimension?,
}

export type WebView = {
	evaluate_noresult: (self: WebView, javascript: string) -> (),
	evaluate_callback: (self: WebView, javascript: string, callback: (unknown) -> ()) -> (),
	evaluate: (self: WebView, javascript: string) -> unknown,
	ipc_handler: (self: WebView, callback: (message: unknown) -> ()) -> Connection,
	load_url: (self: WebView, url: string) -> (),
	load_url_with_headers: (self: WebView, url: string, headers: { [string]: string }) -> (),
}

export type WebViewConfig = {
	with_devtools: boolean?,
	init_script: string?,
	html: string?,
	url: string?,
	headers: { [string]: string }?,
	custom_protocols: { [string]: http.ServeHttpHandler }?,
}

export type WindowEvent = {
	event_type: "CloseRequested",
} | {
	event_type: "MouseButton",
	mousebutton: string,
	pressed: boolean,
} | {
	event_type: "KeyCode",
	keycode: string,
	pressed: boolean,
} | {
	event_type: "CursorMoved",
	position: Dimension,
} | {
	event_type: "None",
}

export type Connection = {
	stop: () -> (),
	is_running: () -> boolean,
}

wry.event_loop = function(window: Window, callback: (event: WindowEvent) -> ()): Connection
	return nil :: any
end

wry.create_window = function(config: WindowConfig): Window
	return nil :: any
end

wry.create_webview = function(window: Window, config: WebViewConfig): WebView
	return nil :: any
end

--[=[
	@deprecated

	event_loop automatically calls this function
]=]
wry.run = function(): Window
	return nil :: any
end

return wry
