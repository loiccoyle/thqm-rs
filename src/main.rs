use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use log::debug;
use qrcode::QrCode;

use std::collections::HashMap;
use std::process::exit;
use std::vec::Vec;

use thqm::cli;
use thqm::server;
use thqm::styles;
use thqm::utils;

fn main() -> Result<()> {
    env_logger::init();

    let data_dir = utils::get_data_dir()?;
    let sys_data_dir = utils::get_sys_data_dir()?;

    // Fetch the available styles
    let sys_styles = if sys_data_dir.exists() {
        styles::fetch(&sys_data_dir)
            .with_context(|| format!("Failed to fetch system styles at {:?}", sys_data_dir))?
    } else {
        vec![]
    };
    debug!("sys_styles: {:?}", sys_styles);
    let user_styles = if data_dir.exists() {
        styles::fetch(&data_dir)
            .with_context(|| format!("Failed to fetch user styles at {:?}", data_dir))?
    } else {
        vec![]
    };
    debug!("user_styles: {:?}", user_styles);

    if sys_styles.is_empty() && user_styles.is_empty() {
        return Err(anyhow!("No styles found."));
    }

    // deduplicate with user dir having priority
    let mut all_styles = HashMap::new();
    [sys_styles, user_styles]
        .concat()
        .into_iter()
        .for_each(|style| {
            if let Some(name) = style.file_name() {
                let name_str = name.to_str().unwrap().to_string();
                all_styles.insert(name_str, style);
            };
        });
    debug!("all_styles: {:?}", all_styles);

    let args = cli::Arguments::parse();

    let mut style_names: Vec<&String> = all_styles.keys().collect::<Vec<_>>();
    style_names.sort();
    if args.list_styles {
        for style in style_names {
            println!("{}", style)
        }
        exit(0)
    }

    if !all_styles.contains_key(&args.style) {
        debug!("No such style: {}", args.style);
        return Err(anyhow!("No such style: {}", args.style));
    }

    let style_path = &all_styles[&args.style];
    debug!("Using style: {:?} at {:?}", args.style, style_path);

    let stdin = utils::read_stdin().context("Failed to read stdin")?;
    debug!("stdin: {:?}", stdin);

    // Split stdin into vec.
    let entries: Vec<String> = stdin
        .split(&args.separator)
        .map(|s| s.trim().to_string())
        .filter(|entry| !entry.is_empty())
        .collect();

    debug!("stdin entries: {:?}", entries);

    let ip = utils::get_ip().context("Failed to determine ip")?;
    debug!("Local ip: {:?}", ip);

    let qrcode_address = utils::create_full_url(
        &ip,
        args.port,
        args.username.as_deref(),
        args.password.as_deref(),
    );
    if args.show_url {
        println!("{}", qrcode_address);
    }

    let code = QrCode::new(&qrcode_address)
        .with_context(|| format!("Failed to generate qrcode with data: {:?}", qrcode_address))?;
    if args.show_qrcode {
        utils::print_qrcode(&code);
    }

    if let Some(code_save_path) = args.save_qrcode {
        utils::save_qrcode(&code, code_save_path.clone())
            .with_context(|| format!("Failed to save qrcode to {:?}", code_save_path))?;
    }

    let server_address = utils::create_url(&ip, args.port);

    let style = styles::Style::new(
        style_path.clone(),
        Some(styles::TemplateOptions::new(
            args.title,
            args.no_qrcode,
            args.no_shutdown,
            entries,
            Some(utils::create_qrcode_svg_string(&code)),
            args.custom_input,
        )),
    )?;

    server::start(
        &style,
        server_address.as_str(),
        args.oneshot,
        args.username,
        args.password,
    )
    .with_context(|| format!("Failed to start web server at: {:?}", server_address))?;
    Ok(())
}
