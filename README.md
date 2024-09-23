<h3 align="center">thqm</h3>
<h3 align="center"><img src="https://i.imgur.com/8VpsYG4.png" width="150"></h3>
<h5 align="center">A simple HTTP server to serve a dynamic menu web page.</h5>
<p align="center">
  <a href="https://github.com/loiccoyle/thqm-rs/actions/workflows/ci.yml"><img src="https://github.com/loiccoyle/thqm-rs/actions/workflows/ci.yml/badge.svg"></a>
  <a href="https://crates.io/crates/thqm"><img src="https://img.shields.io/crates/v/thqm.svg"></a>
  <a href="https://aur.archlinux.org/packages/thqm/"><img src="https://img.shields.io/aur/version/thqm"></a>
  <a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20macOS%20%7C%20windows-informational">
</p>
<hr>

<img src="https://i.imgur.com/lYwkjzP.png" align="right" width='170px'>
<img src="https://i.imgur.com/ezJgbhX.png" align="right" width='170px'>

> `thqm` takes its name from the arabic تحكم, pronounced tahakoom, meaning control.

`thqm` is a nifty little HTTP server. It dynamically generates a web page menu based on the provided `stdin` and outputs any selections to `stdout`.
In a sense, it functions similarly to [`dmenu`](https://tools.suckless.org/dmenu/)/[`rofi`](https://github.com/davatorium/rofi) but the menu is web page served on the local network.

This makes it perfect to control scripts over the network.

**See the [examples](./examples) folder for some example scripts.**

## 📦 Installation

### Manual

To compile and install manually from this repo, you'll need `rust` installed.

To compile the binary:

```console
git clone https://github.com/loiccoyle/thqm-rs
cd thqm-rs
cargo build --release
```

The compiled binary will be located at `./target/release/thqm`.
Just place this binary somewhere in your `$PATH`.

Or to install it straight from `cargo`:

```console
cargo install thqm
```

Make sure you have `cargo`'s bin folder in your `$PATH`.

You'll also need to install the template styles `thqm` with:

```console
thqm --install-styles
```

This will install the styles in the user data folder.

### Arch linux (AUR)

Using your favourite AUR helper:

```console
paru -S thqm
```

The installation process will install the styles system wide in the `/usr/share/thqm` folder.

## 📋 Usage

### CLI options

`thqm` has a few command line options, when in doubt see the `--help`.

<!-- help start -->

```console
$ thqm --help
A simple HTTP server to serve a dynamic menu web page.

thqm generates a menu based on the standard input and writes selections to standard output.

See https://github.com/loiccoyle/thqm.rs/tree/main/examples for full scripts.

Basic usage:
$ echo 'Option 1\nOption 2' | thqm -U |
    while IFS= read -r sel; do
      case $sel in
      'Option 1') echo 'hello';;
      'Option 2') echo 'world';;
      *) echo "$sel";;
      esac
    done

Usage: thqm [OPTIONS]

Options:
  -p, --port <PORT>            The port to listen on [default: 8000]
  -U, --username <USERNAME>    The username to authenticate with
  -P, --password <PASSWORD>    The password to authenticate with
  -S, --separator <SEPARATOR>  The entry separator [default: "\n"]
  -t, --title <TITLE>          The page title [default: thqm]
  -s, --style <STYLE>          The page style [default: default]
  -q, --qrcode                 Show the qrcode in terminal
      --save-qrcode <PATH>     Save the qrcode image to file
  -u, --url                    Show the page url
  -o, --oneshot                Shutdown server after first selection
  -c, --custom-input           Show custom input field
      --list-styles            List available page styles
      --no-shutdown            Don't allow the server to be shutdown from the page
      --no-qrcode              Don't allow the qrcode to be shown in the page
      --install-styles         Download and install styles to the user data directory
  -h, --help                   Print help
  -V, --version                Print version
```

<!-- help end -->

### Scripting

`thqm` will generate a web page based on the provided `stdin`, the selected entry will be printed to `stdout`.

For this behaviour to actually be useful, we'll need to do a bit of scripting.

A typical script will look something like this:

```bash
#!/bin/sh

# define the handler function, i.e. what each option should do.
handler() {
  while IFS= read -r event; do
    case "$event" in
    "Option 1")
      # handle Option 1
      ;;
    "Option 2")
      # handle Option 2
      ;;
    *)
      # pass through thqm's output
      echo "$event"
      ;;
    esac
  done
}

printf "Option 1\nOption 2" | thqm "$@" | handler
# ^                                 ^      ^ Pass user selections to the handler
# │                                 └ Forward script's options to thqm
# └ Provide the options to thqm through stdin
```

**See the [examples](./examples) folder for some example scripts.**

## 🎨 Styling

`thqm` has a few pre-made menu styles, see the [`thqm-styles`](https://github.com/loiccoyle/thqm-styles) repository, which can be installed to your system's user data directory with the `--install-styles` flag.

To create your own styles, follow the same file structure as the included styles.

| Path                                               | Usage                                                                                                          |
| -------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `{DATA_DIR}/thqm/{style_name}`                     | The name of the style is determined by the name of the style's root folder in the user data directory.         |
| `{DATA_DIR}/thqm/{style_name}/template/index.html` | This file is the [`tera`](https://docs.rs/tera/latest/tera/) template which will be used to generate the menu. |
| `{DATA_DIR}/thqm/{style_name}/static/`             | This directory holds static resources such as `css`, `js` and image files.                                     |

> The `{DATA_DIR}` directory depends on the OS:
>
> - Linux: `${XDG_DATA_HOME}`
> - MacOS: `$HOME/Library/Application Support`
> - Windows: `C:\Users\{USER}\AppData\Roaming`

User provided styles take priority over system wide styles.

If you want to contribute your own styles, please feel free to submit them to the [`thqm-styles`](https://github.com/loiccoyle/thqm-styles) repository.
