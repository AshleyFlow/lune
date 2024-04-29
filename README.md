<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

<img align="right" width="250" src="assets/logo/tilt_svg.svg" alt="Lune logo" />

<h1 align="center">Lune WebView</h1>

<div align="center">
 <div>
  <a href="https://github.com/HighFlowey/lune-webview/blob/main/LICENSE.txt">
   <img src="https://img.shields.io/github/license/lune-org/lune.svg?label=License&color=informational" alt="Lune WebView license" />
  </a>
 </div>
</div>

<br/>

## LuneWeb

LuneWeb adds built-in libraries for creating cross-platform web applications to Lune

## Not a UI library, But

LuneWeb does not provide a library for creating UI elements directly, but it does include a method for running javascript code through webviews, so you can create UI elements like this:

```lua
local function label(text: string)
    local code = "let element = document.createElement('h1');"
    code ..= `element.innerHTML = {text};`
    code ..= "document.body.appendChild(element);"
    webview:evaluate(code)
end

label("Hello, Lune!")
```

## Documentation

### Documentation for Lune: [Lune](https://lune-org.github.io/docs/)

### Documentation for LuneWeb's additions to Lune: [LuneWeb](https://highflowey.github.io/luneweb/)
