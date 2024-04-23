--[=[
    ### Results

    first: 0.00012250000145286322
    avrg: 0.004748699997435324
    last: 0.0287188000002061

    the delay between the two event loops keeps getting worse
]=]

local webview = require("@luneweb/webview")

local limit = 1000
local last_output = os.clock()
local index = 0

local results = {}

webview.event_loop(function()
	local output = os.clock()
	local delay = output - last_output
	last_output = output
	index += 1

	table.insert(results, delay)

	if index > limit then
		table.sort(results)
		print("first:", results[1])
		print("avrg:", results[math.floor(#results / 2)])
		print("last:", results[#results])

		return true
	end

	return false
end)
