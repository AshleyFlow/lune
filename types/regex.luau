--[=[
	@class Regex

	Built-in library for regex matching

	### Example usage

	```lua
	local regex = require("@lune/regex")
	local my_regex = regex.new("local\\s+(\\w+)\\s*=\\s*(\\d+)")

    local captures = my_regex:match("local var = 5000")
    print(captures[1]) -- prints local var = 5000
    print(captures[2]) -- prints var
    print(captures[3]) -- prints 5000
	```
]=]
local regex = {}

--[=[
	@interface Regex
	@within Regex

	* `match` (self, string: string) -> { string }
]=]
export type Regex = {
	match: (self: Regex, string: string) -> { string },
}

regex.new = function(pattern: string): Regex
	return nil :: any
end

return regex
