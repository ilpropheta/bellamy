# bellamy

Open URL in chromeless window.

```
Usage: `bellamy.exe [OPTIONS] [URL]`

Arguments:
  [URL]  URL to open [default: https://ilpropheta.github.io/bellamy]

Options:
      --maximized      Create maximized window
      --fullscreen     Create borderless fullscreens on current monitor
      --devtools       Enable developer tools
      --unclosable     Disable the close button
      --title <TITLE>  Window title [default: Bellamy]
      --icon <ICON>    Path to the window icon [default: ]
  -h, --help           Print help
  -V, --version        Print version
```

## Installation

`bellamy` is just an executable that depends on:
- [Visual C++ runtime](https://aka.ms/vs/17/release/vc_redist.x64.exe)
- [WebView2 runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

Be sure you have both installed on your system to use `bellamy`.
