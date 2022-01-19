<h1 align="center">thqm</h1>
<h3 align="center"><img src="https://i.imgur.com/gVB270Z.png" width="150"></h3>
<h5 align="center">Remote command execution made easy.</h5>

<p align="center">
  <a href="https://github.com/loiccoyle/thqm.rs/actions?query=workflow%3Atest"><img src="https://github.com/loiccoyle/thqm.rs/workflows/test/badge.svg"></a>
  <a href="https://crates.io/crates/thqm"><img src="https://img.shields.io/crates/v/thqm.svg"></a>
  <a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20macOS%20%7C%20windows-informational">
</p>

<img src="https://i.imgur.com/lYwkjzP.png" align="right" width='170px'>
<img src="https://i.imgur.com/ezJgbhX.png" align="right" width='170px'>

> `thqm` takes its name from the arabic تحكم, pronounced tahakum, meaning control.

`thqm` is a nifty little HTTP server which reads from standard input. It dynamically generates a menu based on the provided `stdin` and outputs any button the user presses to `stdout`.
In a sense its kind of like [`dmenu`](https://tools.suckless.org/dmenu/)/[`rofi`](https://github.com/davatorium/rofi) but as a HTTP servers.

This makes it very flexible and script friendly. See the [examples](./examples) folder for some scripts.

## Getting started

This project requires rust to be installed. On OS X with Homebrew you can just run `brew install rust`.

Running it then should be as simple as:

```console
$ make
$ ./bin/thqm
```

### Testing

`make test`
