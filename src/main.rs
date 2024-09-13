use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use log::debug;
use qrcode::QrCode;

use std::process::exit;
use std::vec::Vec;

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

    let args = cli::Arguments::parse();
    if args.list_styles {
        for style in &all_styles {
            println!("{}", style)
        }
        exit(0)
    }

    if all_styles.contains(&args.style) {
        debug!("Using style: {}", args.style);
    } else {
        debug!("No such style: {}", args.style);
        return Err(anyhow!("No such style: {}", args.style));
    }

    // #[cfg(feature = "completions")]
    // if args.is_present("completions") {
    //     let shell = args
    //         .value_of_t::<Shell>("completions")
    //         .context("Failed to generate shell completions")?;
    //     let mut app = cli::build_cli(&possible_styles);
    //     cli::print_completions(shell, &mut app);
    //     exit(0)
    // }

    let stdin = utils::read_stdin().context("Failed to read stdin")?;
    debug!("stdin: {:?}", stdin);

    // Split stdin into vec.
    let entries: Vec<String> = if args.separator == r"\n" {
        stdin.lines().map(|s| s.to_string()).collect()
    } else {
        stdin
            .split(&args.separator)
            .map(|s| s.to_string())
            .collect()
    };
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

    let style = styles::Style::from_style_name(
        data_dir,
        args.style,
        Some(styles::TemplateOptions::new(
            args.title,
            !args.no_qrcode,
            !args.no_shutdown,
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
    .context(format!(
        "Failed to start web server at: {:?}",
        server_address
    ))?;
    Ok(())
}
