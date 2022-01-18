use std::{
    io::{self, Read},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use dirs::{data_dir, preference_dir};
use qrcode_generator;

static QRCODE_SIZE: usize = 512;

pub fn get_data_dir() -> Result<PathBuf> {
    Ok(data_dir()
        .ok_or_else(|| anyhow!("Failed to get default data directory."))?
        .join("thqm"))
}

pub fn get_config_dir() -> Result<PathBuf> {
    Ok(preference_dir()
        .ok_or_else(|| anyhow!("Failed to get default config directory."))?
        .join("thqm"))
}

pub fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;
    return Ok(buffer);
}

pub fn get_ip() -> Result<String> {
    Ok(local_ip_address::local_ip()?.to_string())
}

pub fn create_url(host: &str, port: u64) -> String {
    format!("{host}:{port}", host = host, port = port)
}

pub fn create_full_url(
    host: &str,
    port: u64,
    username: Option<&str>,
    password: Option<&str>,
) -> String {
    if username.is_some() && password.is_some() {
        format!(
            "http://{username}:{password}@{host}:{port}",
            username = username.unwrap(),
            password = password.unwrap(),
            host = host,
            port = port
        )
    } else {
        format!("http://{host}:{port}", host = "localhost", port = port)
    }
}

pub fn create_qrcode_svg_string(address: &str) -> Result<String> {
    Ok(qrcode_generator::to_svg_to_string(
        address,
        qrcode_generator::QrCodeEcc::Low,
        QRCODE_SIZE,
        None::<&str>,
    )?)
}

pub fn save_qrcode(address: &str, dest: PathBuf) -> Result<()> {
    Ok(qrcode_generator::to_png_to_file(
        address,
        qrcode_generator::QrCodeEcc::Low,
        QRCODE_SIZE,
        dest,
    )?)
}
