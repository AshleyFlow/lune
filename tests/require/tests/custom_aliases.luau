--!nocheck
local n = require("@test/number")
local t = require("@test/table")

assert(typeof(t) == "table")
assert(typeof(n) == "number")
assert(n == 500)
