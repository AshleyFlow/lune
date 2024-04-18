--!strict
local task = require("@lune/task")
local window = require("@lune/window")
local my_window = window.new({
	init_script = "let ELEMENTS = []; console.log('STARTED')",
})

local counter = 0
my_window.run_script("let ELEMENTS = []; console.log('STARTED')")

local CREATE_ELEMENT = function(tag: string, innerHTML: string?)
	local text = `var element = document.createElement('{tag}')\n`

	if innerHTML then
		text ..= `element.innerHTML = '{innerHTML}'\n`
	end

	text ..= `document.body.appendChild(element)\n`

	return text
end

local function createElement(tag: string, innerHTML: string?)
	local id = counter

	my_window.run_script(CREATE_ELEMENT(tag, innerHTML) .. "ELEMENTS.push(element)")

	counter += 1

	return {
		style = (setmetatable({}, {
			__newindex = function(_, i, v)
				my_window.run_script(`ELEMENTS[{id}].style.{i} = "{v}"`, function()
					print("style")
				end)
			end,
		}) :: any) :: {
			[string]: string,
		},
	}
end

task.delay(1, function()
	local header = createElement("h1", "Hello, World!")
	createElement("h2", "um...")
	createElement("h3", "um..")
	header.style.color = "red"
end)

while true do
	local event = my_window.process_events()

	if event == window.events.Exit then
		my_window.set_visible(false)
		break
	end
end
