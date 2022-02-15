use std::process::exit;
use std::vec::Vec;

use anyhow::Context;
use anyhow::{anyhow, Result};
#[cfg(feature = "completions")]
use clap_complete::Shell;
use log::debug;
use qrcode::QrCode;

use thqm::cli;
use thqm::server;
use thqm::styles;
use thqm::utils;

fn main() -> Result<()> {
    env_logger::init();

    let data_dir = utils::get_data_dir()?;
    // let config_dir = utils::get_config_dir()?;

    // Initialize styles
    styles::init(&data_dir).with_context(|| format!("Failed to init data dir: {:?}", data_dir))?;
    // Fetch the available styles
    let all_styles = styles::fetch(&data_dir).context("Failed to fetch available styles")?;
    debug!("all_styles: {:?}", all_styles);

    let possible_styles = all_styles
        .iter()
        .map(String::as_str)
        .collect::<Vec<&str>>()
        .as_slice()
        .to_owned();
    let args = cli::build_cli(&possible_styles).get_matches();
    if args.is_present("list_styles") {
        for style in &all_styles {
            println!("{}", style)
        }
        exit(0)
    }

    #[cfg(feature = "completions")]
    if args.is_present("completions") {
        let shell = args
            .value_of_t::<Shell>("completions")
            .context("Failed to generate shell completions")?;
        let mut app = cli::build_cli(&possible_styles);
        cli::print_completions(shell, &mut app);
        exit(0)
    }

    let stdin = utils::read_stdin().context("Failed to read stdin")?;
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

    let ip = utils::get_ip(args.value_of("interface")).context("Failed to determine ip")?;
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
    let code = QrCode::new(&qrcode_address)
        .with_context(|| format!("Failed to generate qrcode with data: {:?}", qrcode_address))?;
    if args.is_present("show_qrcode") {
        utils::print_qrcode(&code);
    }
    if let Some(code_save_path) = args.value_of("save_qrcode") {
        utils::save_qrcode(&code, args.value_of("save_qrcode").unwrap())
            .with_context(|| format!("Failed to save qrcode to {:?}", code_save_path))?;
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
            Some(utils::create_qrcode_svg_string(&code)),
            args.is_present("custom_input"),
        )),
    )?;

    server::start(
        &style,
        server_address.as_str(),
        args.is_present("oneshot"),
        args.value_of("username").map(|s| s.to_string()),
        args.value_of("password").map(|s| s.to_string()),
    )
    .context(format!(
        "Failed to start web server at: {:?}",
        server_address
    ))?;
    Ok(())
}
