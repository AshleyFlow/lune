--[=[
    ### Results

	first: 0.0000013999961083754897
	avrg: 0.00010399999882793054
	last: 0.024510199997166637

    the delay between the two event loops keeps getting worse
]=]

local task = require("@luneweb/task")
local webview = require("@luneweb/webview")

local limit = 1000
local output1 = os.clock()
local output2 = os.clock()
local index1 = 0
local index2 = 0

webview.event_loop(function()
	output1 = os.clock()
	index1 += 1

	if index1 > limit then
		return true
	end

	return false
end)

webview.event_loop(function()
	output2 = os.clock()
	index2 += 1

	if index2 > limit then
		return true
	end

	return false
end)

local results = {}

while true do
	table.insert(results, output2 - output1)

	if index1 > limit and index2 > limit then
		break
	end

	task.wait()
end

table.sort(results)
print("first:", results[1])
print("avrg:", results[math.floor(#results / 2)])
print("last:", results[#results])
