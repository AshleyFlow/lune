--[=[
	@class Webview

	Built-in webview library
]=]
local webview = {}

--[=[
    @interface Config
    @within Webview

    Config for building webview applications

	* `onExit` () -> void
	* `url` string (e.g. http://localhost:8080)
]=]
export type Config = {
	onExit: () -> ()?,
	url: string?,
}

--[=[
	@within Webview

	Spawns a window that contains a webview.

	@param config [W.I.P] configuration
	@return Window
]=]
webview.build = function(config: Config): Window
	return nil :: any
end

--[=[
	@interface Window
	@within Webview

	A window containing a webview

	* `exit` (self) -> void
	* `openDevtools` (self) -> void
	* `closeDevtools` (self) -> void
	* `executeJavascript` (self, script: string) -> void
	* `loadUrl` (self, url: string) -> void
	* `onExit` (self, callback: () -> void)) -> void
]=]
export type Window = {
	-- methods
	exit: (Window) -> (),
	openDevtools: (Window) -> (),
	closeDevtools: (Window) -> (),
	executeJavascript: (Window, script: string) -> (),
	loadUrl: (Window, url: string) -> (),
	-- callbacks
	onExit: (Window, callback: () -> ()) -> (),
}

return webview
