use clap::{app_from_crate, App, Arg};
#[cfg(feature = "completions")]
use {
    clap_complete::{generate, Generator},
    std::io,
};

pub fn build_cli<'a>(possible_styles: &'a [&'a str]) -> App<'a> {
    let app = app_from_crate!()
        .about(
            "A simple HTTP server to serve a dynamic menu for your scripts.

thqm serves a menu from standard input and writes selections to standard output.

See https://github.com/loiccoyle/thqm.rs/tree/main/examples for full scripts.

Basic usage:
$ echo 'Option 1\\nOption 2' | thqm -U |
    while IFS= read -r sel; do
      case $sel in
      'Option 1') echo 'hello';;
      'Option 2') echo 'world';;
      *) echo \"$sel\";;
      esac
    done",
        )
        .arg(
            Arg::new("port")
                .help("Set the server's port.")
                .short('p')
                .long("port")
                .default_value("8000")
                .takes_value(true),
        )
        .arg(
            Arg::new("username")
                .help("Authentication username.")
                .short('u')
                .long("username")
                .default_value("thqm")
                .takes_value(true),
        )
        .arg(
            Arg::new("password")
                .help("Authentication password.")
                .short('P')
                .long("password")
                .takes_value(true),
        )
        .arg(
            Arg::new("separator")
                .help("Entry separator.")
                .long("separator")
                .short('S')
                .default_value(r"\n")
                .takes_value(true),
        )
        .arg(
            Arg::new("title")
                .help("Page title.")
                .short('t')
                .long("title")
                .default_value("thqm")
                .takes_value(true),
        )
        .arg(
            Arg::new("style")
                .help("Page style.")
                .short('s')
                .long("style")
                .default_value("default")
                .takes_value(true)
                .possible_values(possible_styles),
        )
        .arg(
            Arg::new("list_styles")
                .help("List available page styles.")
                .long("list-styles"),
        )
        .arg(
            Arg::new("interface")
                .help("Network interface to use to determine ip.")
                .long("interface")
                .takes_value(true),
        )
        .arg(
            Arg::new("show_qrcode")
                .help("Show the qrcode in terminal.")
                .short('q')
                .long("show-qrcode"),
        )
        .arg(
            Arg::new("save_qrcode")
                .help("Save the qrcode image to file.")
                .long("save-qrcode")
                .value_name("path")
                .takes_value(true),
        )
        .arg(
            Arg::new("show_url")
                .help("Show the page url.")
                .short('U')
                .long("show-url")
                .takes_value(false),
        )
        .arg(
            Arg::new("oneshot")
                .help("Shutdown server after first selection.")
                .long("oneshot")
                .takes_value(false),
        )
        .arg(
            Arg::new("custom_input")
                .help("Show custom input field.")
                .long("custom-input")
                .takes_value(false),
        )
        .arg(
            Arg::new("no_shutdown")
                .help("Don't allow the server to be shutdown from the page.")
                .long("no-shutdown")
                .takes_value(false),
        )
        .arg(
            Arg::new("no_qrcode")
                .help("Don't show the qrcode on the page.")
                .long("no-qrcode")
                .takes_value(false),
        );

    #[cfg(feature = "completions")]
    let app = app.arg(
        Arg::new("completions")
            .help("Generate shell completions.")
            .long("completions")
            .value_name("shell")
            .takes_value(true)
            .possible_values(["bash", "elvish", "fish", "powershell", "zsh"]),
    );

    app
}

#[cfg(feature = "completions")]
pub fn print_completions<G: Generator>(gen: G, app: &mut App) {
    // TOOD: make completion look for installed styles.
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
