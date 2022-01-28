use std::path;
use std::vec::Vec;

use anyhow::Context;
use anyhow::{anyhow, Result};
use log::debug;

use thqm::cli;
use thqm::server;
use thqm::styles;
use thqm::utils;

fn main() -> Result<()> {
    env_logger::init();

    let data_dir = utils::get_data_dir()?;
    // let config_dir = utils::get_config_dir()?;

    // Initialize styles
    styles::init(&data_dir).context("Unpacking styles to data dir.")?;
    // Fetch the available styles
    let all_styles = styles::fetch(&data_dir).context("Fetching styles.")?;
    debug!("all_styles: {:?}", all_styles);

    let args = cli::build_cli(
        all_styles
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
            .as_slice(),
    )
    .get_matches();

    let stdin = utils::read_stdin().unwrap();
    debug!("stdin: {:?}", stdin);

    // Separator logic
    let sep = args
        .value_of("separator")
        .ok_or_else(|| anyhow!("No separator."))?;
    debug!("sep: {:?}", sep);

    // Split stdin into vec.
    let entries: Vec<String> = if sep == r"\n" {
        stdin.lines().map(|s| s.to_string()).collect()
    } else {
        stdin
            .split(sep)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    };
    debug!("stdin entries: {:?}", entries);

    let port = args
        .value_of("port")
        .ok_or_else(|| anyhow!("No port."))?
        .parse::<u64>()?;
    debug!("Port: {}", port);

    let ip = utils::get_ip()?;
    debug!("Local ip: {:?}", ip);

    let qrcode_address = utils::create_full_url(
        &ip,
        port,
        args.value_of("username"),
        args.value_of("password"),
    );
    if args.is_present("show_url") {
        println!("{}", qrcode_address);
    }
    if args.value_of("save_qrcode").is_some() {
        utils::save_qrcode(
            &qrcode_address,
            path::PathBuf::from(
                args.value_of("save_qrcode")
                    .ok_or_else(|| anyhow!("No qrcode save path."))?,
            ),
        )?;
    }
    let server_address = utils::create_url(&ip, port);

    let style_name = args.value_of("style").ok_or_else(|| anyhow!("No style."))?;
    let style = styles::Style::from_style_name(
        data_dir,
        style_name.to_string(),
        Some(styles::TemplateOptions::new(
            args.value_of("title")
                .ok_or_else(|| anyhow!("No title."))?
                .to_string(),
            !args.is_present("no_qrcode"),
            !args.is_present("no_shutdown"),
            entries,
            utils::create_qrcode_svg_string(&qrcode_address).ok(),
            args.is_present("custom_input"),
        )),
    )?;

    server::start(
        &style,
        server_address.as_str(),
        args.is_present("oneshot"),
        args.value_of("username").map(|s| s.to_string()),
        args.value_of("password").map(|s| s.to_string()),
    )?;
    Ok(())
}
