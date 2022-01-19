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
                .long("username")
                .default_value("thqm")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .help("Authentication password.")
                .short("P")
                .long("password")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("separator")
                .help("Entry separator.")
                .long("separator")
                .short("S")
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
                .help("Page style.")
                .short("s")
                .long("style")
                .default_value("pure_html")
                .takes_value(true)
                .possible_values(possible_styles),
        )
        /* .arg(
            Arg::with_name("show_qrcode")
                .help("Show the qrcode in terminal.")
                .short("q")
                .long("show-qrcode"),
        ) */
        .arg(
            Arg::with_name("save_qrcode")
                .help("Save the qrcode image to file.")
                .long("save-qrcode"),
        )
        .arg(
            Arg::with_name("show_url")
                .help("Show the page url.")
                .short("U")
                .long("show-url")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("oneshot")
                .help("Shutdown server after first selection.")
                .long("oneshot")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("no_shutdown")
                .help("Don't allow the server to be shutdown from the page.")
                .long("no-shutdown")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("no_qrcode")
                .help("Don't show the qrcode on the page.")
                .long("no-qrcode")
                .takes_value(false),
        )
}
