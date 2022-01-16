use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, AppSettings,
    Arg,
};

pub fn build_cli<'a, 'b>(possible_styles: &[&'a str]) -> App<'a, 'b> {
    app_from_crate!()
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::UnifiedHelpMessage,
            AppSettings::ArgRequiredElseHelp,
            AppSettings::ValidArgFound,
        ])
        .arg(
            Arg::with_name("port")
                .help("Set the server's port.")
                .short("p")
                .long("port")
                .default_value("8000")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("username")
                .help("Authentication username.")
                .short("u")
                .long("usename")
                .default_value("thqm")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .help("Authentication password.")
                .short("w")
                .long("password"),
        )
        .arg(
            Arg::with_name("separator")
                .help("Entry separator.")
                .short("s")
                .long("separator")
                .default_value(r"\n")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("title")
                .help("Path title.")
                .short("t")
                .long("title")
                .default_value("thqm")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("style")
                .help("Path style.")
                .short("y")
                .long("style")
                .default_value("pure_html")
                .takes_value(true)
                .possible_values(possible_styles),
        )
        .arg(
            Arg::with_name("show_qrcode")
                .help("Show the qrcode in terminal.")
                .short("q")
                .long("show-qrcode"),
        )
        .arg(
            Arg::with_name("save_qrcode")
                .help("Save the qrcode to file.")
                .long("save-qrcode"),
        )
        .arg(
            Arg::with_name("show_url")
                .help("Show the page url.")
                .long("show-url"),
        )
        .arg(
            Arg::with_name("oneshot")
                .help("Shutdown server after first click.")
                .long("oneshot")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("no_shutdown")
                .help("Shutdown server after first click.")
                .long("no-shutdown"),
        )
        .arg(
            Arg::with_name("no_qrcode")
                .help("Remove qrcode button.")
                .long("no-qrcode"),
        )
}
