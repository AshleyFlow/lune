---
sidebar_position: 1
---

# Examples / Guides

## Creating a basic UI Library

This guide will not show you how to create a window and handle it's events, it will only show you how to make a simple function that attaches a text to the webview.

---

The WebView API provides us with a method that lets us run Javascript code, directly from Luau.  
we will use that to create UI elements like this:

```lua title="src/init.luau"
webview:evaluate_noresult([[
let el = document.createElement("h1")
el.innerHTML = "Hello, Lune!"
document.body.appendChild(el)
]])
```

This code is basically running a javascript code through the webview, which creates a "Hello, Lune!" header and attaches it to the UI.

---

We should make our code a little more safe by waiting for an answer from the webview, like this:

```lua title="src/init.luau"
type Status = {
    success: boolean,
}

local status: Status = webview:evaluate([[
let response = (
    () => {
        try {
            let el = document.createElement("h1")
            el.innerHTML = "Hello, Lune!"
            document.body.appendChild(el)
            return { success: true }
        } catch {
            return { success: false }
        }
    }
)()

response
]])

if status.success == false then
    print("Failed to create web element")
end
```

webview:evaluate(...) returns the last statement in the provided javascript code as a Luau value, so in our javascript code we can return a table that tells us if the element has been created or not.

This javascript code might look a _bit_ confusing, it basically creates a function and calls it directly  
The function runs a try catch block trying to create an element, if it succeeds it returns { success: true }, otherwise { success: false }

---

Debugging a string is the worst thing that can happen to a developer, so let's put our javascript code in it's own file

```js title="src/javascript/hello.js"
let response = (() => {
  try {
    let el = document.createElement("h1");
    el.innerHTML = "Hello, Lune!";
    document.body.appendChild(el);
    return { success: true };
  } catch {
    return { success: false };
  }
})();

response;
```

```lua title="src/init.luau"
local fs = require("@luneweb/fs")
local code = fs.readFile("src/javascript/hello.js")

type Status = {
    success: boolean,
}

local status: Status = webview:evaluate(code)

if status.success == false then
    print("Failed to create web element")
end
```

---

Ok, now let's make it a bit more customizable!

```lua title="src/init.luau"
local regex = require("@luneweb/regex")
local code = fs.readFile("src/javascript/hello.js")
code = regex.new("Hello, Lune!"):replaceAll(code, "Hello, World!")
```

Here we used the regex library to replace "Hello, Lune!" with "Hello, World!"

---

We can now wrap this all up into a function

```lua title="lib/init.luau"
type Status = {
    success: boolean,
}

local function label(text: string): Status
    local fs = require("@luneweb/fs")
    local regex = require("@luneweb/regex")
    local code = fs.readFile("src/javascript/hello.js")
    code = regex.new("Hello, Lune!"):replaceAll(code, text)

    webview:evaluate(code)
end

local status = label("Hello, World!")

if status.success == false then
    print("Failed to create web element")
end
```
