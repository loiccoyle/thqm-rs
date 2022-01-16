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

pub fn create_qrcode_svg_string(address: &str) -> Result<String> {
    Ok(qrcode_generator::to_svg_to_string(
        address,
        qrcode_generator::QrCodeEcc::Low,
        QRCODE_SIZE,
        None::<&str>,
    )?)
}
