--[=[
	@class Webview

	Built-in webview spawner
]=]
local webview = {}

--[=[
    @interface Config
    @within Webview

    Config for building applications

	* `exit` () -> void
	* `url` string (e.g. http://localhost:8080)
]=]
export type Config = {
	exit: () -> (),
	url: string,
}

--[=[
	@within Webview

	Spawns a new webview.

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

	* `exit` () -> void
	* `open_devtools` () -> void
	* `close_devtools` () -> void
	* `load_url` (url: string) -> void
]=]
export type Window = {
	exit: () -> (),
	open_devtools: () -> (),
	close_devtools: () -> (),
	load_url: (url: string) -> (),
}

return webview
