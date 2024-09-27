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
    let args = cli::Arguments::parse();
    env_logger::init();

    let data_dir = utils::get_data_dir()?;
    if args.install_styles {
        utils::download_styles_to_dir(&data_dir)?;
        return Ok(());
    }
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

    let mut style_paths = [sys_styles, user_styles].concat();
    let chosen_style = match args.style_dir {
        Some(style_dir) if styles::is_style(&style_dir) => {
            style_paths.push(style_dir.clone());
            style_dir
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| anyhow!("Invalid style dir: {:?}", style_dir))?
                .to_string()
        }
        Some(_) => return Err(anyhow!("Invalid style dir: {:?}", args.style_dir)),
        None => args.style,
    };

    // deduplicate
    let mut all_styles = HashMap::new();
    style_paths.into_iter().for_each(|style| {
        if let Some(name) = style.file_name().and_then(|name| name.to_str()) {
            all_styles.insert(name.to_string(), style);
        };
    });
    debug!("all_styles: {:?}", all_styles);
    if all_styles.is_empty() {
        return Err(anyhow!("No styles found."));
    }

    let mut style_names: Vec<&String> = all_styles.keys().collect::<Vec<_>>();
    style_names.sort();
    if args.list_styles {
        for style in style_names {
            println!("{}", style)
        }
        exit(0)
    }

    if !all_styles.contains_key(&chosen_style) {
        debug!("No such style: {}", chosen_style);
        return Err(anyhow!("No such style: {}", chosen_style));
    }

    let style_path = &all_styles[&chosen_style];
    debug!("Using style: {:?} at {:?}", chosen_style, style_path);

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
        style_path.to_owned(),
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
