use std::vec::Vec;

use anyhow::{anyhow, Result};
use env_logger;
use log::debug;

use thqm::cli;
use thqm::server;
use thqm::styles;
use thqm::utils;

fn main() -> Result<()> {
    env_logger::init();

    let data_dir = utils::get_data_dir()?;
    let config_dir = utils::get_config_dir()?;
    // Initialize styles
    styles::init(&data_dir)?;
    // Fetch the available styles
    let all_styles = styles::fetch(&data_dir)?;
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

    let address = format!("{address}:{port}", address = "localhost", port = port);

    let style_name = args.value_of("style").ok_or_else(|| anyhow!("No style."))?;
    let style = styles::Style::from_style_name(
        data_dir,
        style_name.to_string(),
        Some(styles::TemplateOptions::new(
            "thqm".to_string(),
            true,
            false,
            entries,
            utils::create_qrcode_svg_string(
                format!("http://{address}", address = address).as_str(),
            )
            .ok(),
        )),
    )?;

    server::start(&style, address.as_str())?;

    /* let mut thqm_server = server::ThqmServer::new(
        "0.0.0.0",
        port,
        args.value_of("username"),
        args.value_of("password"),
        args.is_present("oneshot"),
        "test",
    );

    thqm_server.init();
    thqm_server.start()?; */

    Ok(())
}
