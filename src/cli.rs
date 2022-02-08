use clap::{app_from_crate, App, Arg};

pub fn build_cli<'a>(possible_styles: &'a [&'a str]) -> App<'a> {
    app_from_crate!()
        // .settings(&[AppSettings::ArgRequiredElseHelp, AppSettings::ValidArgFound])
        .about(
            "Control your scripts over the network.

thqm generates a web page menu from standard input and outputs client selections to standard output.

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
            Arg::new("interface")
                .help("Network interface to use to determine ip.")
                .long("interface")
                .takes_value(true)
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
        )
}
