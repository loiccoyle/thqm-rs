<h3 align="center">thqm</h1>
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
In a sense, it functions similarly to [`dmenu`](https://tools.suckless.org/dmenu/)/[`rofi`](https://github.com/davatorium/rofi) but as a HTTP servers.

This makes it very flexible and script friendly.

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

### Cargo

```console
cargo install thqm
```

### Arch linux (AUR)

Using your favourite AUR helper:

```console
paru -S thqm
```

## 📋 Usage

### CLI options

`thqm` has a few command line options, when in doubt see the `--help`.

<!-- help start -->

```
$ thqm --help
A simple HTTP server to serve a dynamic menu for your scripts.

thqm serves a menu from standard input and writes selections to standard output.

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
      # pass through
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

`thqm` comes with a few included menu styles, see the [styles](./styles) folder, they will be extracted to `$XDG_DATA_DIR/thqm` when `thqm` is first run.

You can add your own by following the same style structure as those already included.

Note: `thqm` uses [`tera`](https://docs.rs/tera/latest/tera/) templates to generate the menu.
