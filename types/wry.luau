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
}

export type WebView = {
	evaluate: (self: WebView, javascript: string) -> unknown,
	ipc_handler: (self: WebView, callback: (message: unknown) -> ()) -> WebViewIPCHandle,
	load_url: (self: WebView, url: string) -> (),
	load_url_with_headers: (self: WebView, url: string, headers: { [string]: string }) -> (),
}

export type WebViewConfig = {
	init_script: string?,
	html: string?,
	url: string?,
	headers: { [string]: string }?,
	custom_protocols: { [string]: http.ServeHttpHandler }?,
}

export type WindowEvent = {
	mousebutton: string?,
	keycode: string?,
	pressed: boolean?,
	position: Dimension?,
}

export type WebViewIPCHandle = {
	stop: () -> (),
}

export type WindowEventLoopHandle = {
	stop: () -> (),
}

wry.events = {
	CloseRequested = (nil :: any) :: WindowEvent,
	MouseButton = (nil :: any) :: WindowEvent,
	KeyCode = (nil :: any) :: WindowEvent,
	CursorMoved = (nil :: any) :: WindowEvent,
	None = (nil :: any) :: WindowEvent,
}

wry.event_loop = function(
	window: Window,
	callback: (event: WindowEvent) -> ()
): WindowEventLoopHandle
	return nil :: any
end

wry.create_window = function(config: WindowConfig): Window
	return nil :: any
end

wry.create_webview = function(window: Window, config: WebViewConfig): WebView
	return nil :: any
end

wry.run = function(): Window
	return nil :: any
end

return wry
