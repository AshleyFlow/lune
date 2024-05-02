<!-- markdownlint-disable MD010 -->

# Examples

## Creating a basic Window with a WebView

```lua title="src/init.luau"
local wry = require("@luneweb/wry")

local loop: wry.Connection
local window = wry.create_window({
  title = "Lune"
})

wry.create_webview(window, {
  url = "https://roblox.com/"
})


--[[
  if we don't create an event loop, our window will be closed
  the moment we run the apllication
]]
loop = wry.event_loop(window, function(msg)
  if msg == "CloseRequested" then
    window:close()
    loop.stop()
  end
end)

--[[
  this method will start all the previously created event loops
  without this, our event loops will never start and the app will crash
]]
wry.run()
```

## Creating a basic UI component

This example does not cover how to handle a window and it's event loop.

```js title="src/javascript/hello.js"
/*
    we'll try to not create any variables in the main scope
    because when wry evaluates a javascript code into the webview
    it keeps the variables for the whole session

    for example, if we do `let a = 100` twice
    the second one will fail because `a` is already declared.
*/

(() => {
  /*
    the reason for creating a function here is because we want
    the try and catch block to return either { success: true } or { success: false } for us
    so in luau code we know if creating the element was successful or not
  */
  try {
    let el = document.createElement("h1");
    el.innerHTML = "Hello, Lune!";
    document.body.appendChild(el);
    return { success: true };
  } catch {
    return { success: false };
  }

  /*
    if we want luau to get { success: boolean }
    we'll need the returned value to be the last statement in the code
    so we call the function directly at the last part of the code
  */
})();
```

```lua title="lib/init.luau"
--[[
    we'll be using the fs library to read the javascript code
    and use the regex library to replace "Hello, Lune!" with whatever string we want
]]
local fs = require("@luneweb/fs")
local regex = require("@luneweb/regex")

--[[
    since our javascript code returns { success: boolean }
    and the webview library turns javascript values into luau values automatically
    we can create a type for it so we can get typechecking in our code
]]
type Status = {
	success: boolean,
}

local function label(text: string): Status
    --[[
        we'll use fs.readFile to get the javascript as a luau string
        and then we'll use regex to replace "Hello, Lune!"
        with the provided string in the function paramters
    ]]
	local code = fs.readFile("src/javascript/hello.js")
	code = regex.new("Hello, Lune!"):replaceAll(code, text)

    --[[
        we'll evaluate the javascript code and return it's result
    ]]
	return webview:evaluate(code)
end

local status = label("Hello, World!")

--[[
    since our javascript code returns { success: boolean }
    we can check the result and print out a warning if .success was false
]]
if status.success == false then
	warn("Failed to create label")
end
```
