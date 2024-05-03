<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

<img align="right" width="250" src="assets/logo/tilt_svg.svg" alt="Lune logo" />

<h1 align="center">Lune Web</h1>

<div align="center">
 <div>
  <a href="https://github.com/HighFlowey/luneweb/blob/main/LICENSE.txt">
   <img src="https://img.shields.io/github/license/lune-org/lune.svg?label=License&color=informational" alt="License" />
  </a>
 </div>
</div>

<br/>

## Goals

LuneWeb's goal is to provide users with an api for creating web applications and having full control over them.

## UI Framework

We do not provide any kind of framework for creating UI in Luau, it is up to the developers to create bindings for such things, for example:

```lua
-- ! This code is not tested.
local serde = require("@luneweb/serde")
local id = 0

local function title(value: string)
    local value_js = serde.encode("json", value)
    local script = {
        `let el{id} = document.createElement("h1")`,
        `el{id}.innerHTML = {value_js}`,
        `document.body.appendChild(el{id})`
    }

    id += 1

    return table.concat(script, ";")
end
```

```lua
-- ... after creating a window with a webview
webview:evaluate_noresult(title("Hello, World!"))
```

## Documentation

Documentation for all the built-in libraries that already exist in Lune will be here:
[Lune's Documentation](https://lune-org.github.io/docs/)

Documentation for built-in libraries that are only in this repository will be here:
[LuneWeb's Documentation](https://highflowey.github.io/luneweb/)

## Installation

### Aftman

use `aftman add HighFlowey/luneweb@<version> --global` to install luneweb globally on your system (replace \<version> with a valid version, like 0.1.4)

---

### Downloading

Download the executable from [Releases](https://github.com/HighFlowey/luneweb/releases/latest), and add it to PATH

---

### Building

`git clone` this repo and use `cargo build --release` to build an executable, the executable can be found in ./target/release/, now you cann add the executable to PATH

## Platform-specific notes

Here is the underlying web engine each platform uses, and some dependencies you might need to install.

### Linux

Before 0.1.6, there is no support for Linux.

LuneWeb needs WebKitGTK for WebView. So please make sure the following packages are installed:

#### Arch Linux / Manjaro

`sudo pacman -S webkit2gtk-4.1`

Debian / Ubuntu:

`sudo apt install libwebkit2gtk-4.1-dev`

Fedora:

`sudo dnf install gtk3-devel webkit2gtk4.1-devel`

---

#### macOS

macOS is not tested.

WebKit is native on macOS so everything should be fine.

---

#### Windows

WebView2 provided by Microsoft Edge Chromium is used. So wry supports Windows 7, 8, 10 and 11.
