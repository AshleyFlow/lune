local regex = require("@lune/regex")
local my_regex = regex.new("local\\s+(\\w+)\\s*=\\s*(\\d+)")

local captures = my_regex.match("local var = 5000")
print(#captures) -- prints 3
print(captures[1]) -- prints local var = 5000
print(captures[2]) -- prints var
print(captures[3]) -- prints 5000

local _captures = my_regex.match("local var =")
print(#_captures) -- prints 0
