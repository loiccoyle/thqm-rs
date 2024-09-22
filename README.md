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

You'll also need to provide `thqm` with some style templates, so copy the included styles to either the system data folder `/usr/share/thqm` or your user data folder `${XDG_DATA_DIR}/thqm`.

```console
# To install system wide
mkdir -p /usr/share/thqm
cp -r styles/. /usr/share/thqm
# To install per user
mkdir -p ~/.local/share/thqm
cp -r styles/. ~/.local/share/thqm
```

### Cargo

```console
cargo install thqm
```

You'll also need to provide `thqm` with some style templates, so copy the included styles to either the system data folder `/usr/share/thqm` or your user data folder `${XDG_DATA_DIR}/thqm`.

```console
git clone https://github.com/loiccoyle/thqm-rs
cd thqm-rs
# To install system wide
mkdir -p /usr/share/thqm
cp -r styles/. /usr/share/thqm
# To install per user
mkdir -p ~/.local/share/thqm
cp -r styles/. ~/.local/share/thqm
```

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

```
$ thqm --help
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

`thqm` comes with a few included menu styles, see the [styles](./styles) folder. You can also place custom styles in your `${XDG_DATA_DIR}/thqm` folder. User provided styles take priority over system wide styles.

You can add your own by following the same style structure as those already included.

Note: `thqm` uses [`tera`](https://docs.rs/tera/latest/tera/) templates to generate the menu.
