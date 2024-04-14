--[=[
	@class Webview

	Built-in webview library
]=]
local webview = {}

--[=[
    @interface Config
    @within Webview

    Config for building webview applications

	* `exit` () -> void
	* `url` string (e.g. http://localhost:8080)
]=]
export type Config = {
	exit: () -> (),
	url: string,
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
	* `open_devtools` (self) -> void
	* `close_devtools` (self) -> void
	* `load_url` (self, url: string) -> void
]=]
export type Window = {
	exit: (Window) -> (),
	open_devtools: (Window) -> (),
	close_devtools: (Window) -> (),
	load_url: (Window, url: string) -> (),
}

return webview
