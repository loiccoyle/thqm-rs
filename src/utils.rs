use anyhow::{anyhow, Context, Result};
use dirs::data_dir;
use image::Luma;
use qrcode::render::{svg, unicode};
use qrcode::QrCode;

use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;

static QRCODE_SIZE: u32 = 256;

/// Determine the system's data directory.
pub fn get_sys_data_dir() -> Result<PathBuf> {
    PathBuf::from_str("/usr/share/thqm")
        .with_context(|| anyhow!("Failed to get system data directory."))
}

/// Determine the user's data directory.
pub fn get_data_dir() -> Result<PathBuf> {
    Ok(data_dir()
        .ok_or_else(|| anyhow!("Failed to get default data directory."))?
        .join("thqm"))
}

/// Download styles and extract them to a directory.
pub fn download_styles_to_dir(dir: &PathBuf) -> Result<()> {
    let url = "https://github.com/loiccoyle/thqm-styles/releases/latest/download/styles.tar.gz";
    let resp = reqwest::blocking::get(url).with_context(|| "Failed to download styles.")?;
    let mut tar = flate2::read::GzDecoder::new(resp);
    let mut archive = tar::Archive::new(&mut tar);
    archive
        .unpack(dir)
        .with_context(|| format!("Failed to extract downloaded styles to {:?}.", dir))?;
    Ok(())
}

/// Read stdin.
pub fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Get the local ip.
pub fn get_ip() -> Result<String> {
    Ok(local_ip_address::local_ip().map(|s| s.to_string())?)
}

/// Create the url string.
///
/// ```
/// use thqm::utils::create_url;
///
/// assert_eq!("192.168.1.1:123456", create_url("192.168.1.1", 123456));
/// ```
pub fn create_url(host: &str, port: u64) -> String {
    format!("{host}:{port}", host = host, port = port)
}

/// Create a full url string, with http basic auth if logins provided.
///
/// ```
/// use thqm::utils::create_full_url;
///
/// assert_eq!("http://thqm:test@192.168.1.1:123456", create_full_url("192.168.1.1", 123456, Some("thqm"), Some("test")));
/// assert_eq!("http://192.168.1.1:123456", create_full_url("192.168.1.1", 123456, None, None));
/// ```
pub fn create_full_url(
    host: &str,
    port: u64,
    username: Option<&str>,
    password: Option<&str>,
) -> String {
    if let (Some(username), Some(password)) = (username, password) {
        format!(
            "http://{username}:{password}@{host}:{port}",
            username = username,
            password = password,
            host = host,
            port = port
        )
    } else {
        format!("http://{host}:{port}", host = host, port = port)
    }
}

/// Convert qrcode to svg string.
pub fn create_qrcode_svg_string(code: &QrCode) -> String {
    code.render::<svg::Color>()
        .min_dimensions(QRCODE_SIZE, QRCODE_SIZE)
        .build()
}

/// Print a qrcode in the terminal.
pub fn print_qrcode(code: &QrCode) {
    let unicode_img = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", unicode_img)
}

/// Save a qrcode to file as an image.
pub fn save_qrcode<P>(code: &QrCode, dest: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let image = code
        .render::<Luma<u8>>()
        .min_dimensions(QRCODE_SIZE, QRCODE_SIZE)
        .build();
    Ok(image.save(dest)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;
    use std::fs;
    use std::fs::remove_dir_all;

    static TEST_DIR: &str = "./utils_test_dir/";

    #[cfg(test)]
    #[ctor::ctor]
    fn setup() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        fs::create_dir(test_dir).unwrap();
    }

    #[cfg(test)]
    #[ctor::dtor]
    fn teardown() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        if test_dir.is_dir() {
            remove_dir_all(&test_dir).unwrap();
            debug!("Removed: {}", test_dir.display());
        }
    }

    #[test]
    fn test_create_url() {
        assert_eq!(create_url("test_host", 1234), "test_host:1234");
    }

    #[test]
    fn test_create_full_url() {
        assert_eq!(
            create_full_url("test_host", 1234, Some("user"), Some("hunter2")),
            "http://user:hunter2@test_host:1234"
        );
        assert_eq!(
            create_full_url("test_host", 1234, None, Some("hunter2")),
            "http://test_host:1234"
        );
        assert_eq!(
            create_full_url("test_host", 1234, None, None),
            "http://test_host:1234"
        );
    }

    #[test]
    fn test_create_svg_string() {
        let code = QrCode::new("test").unwrap();
        create_qrcode_svg_string(&code);
    }

    #[test]
    fn test_print_qrcode() {
        let code = QrCode::new("test").unwrap();
        print_qrcode(&code);
    }

    #[test]
    fn test_save_qrcode() {
        let code = QrCode::new("test").unwrap();
        let qrcode_dest = PathBuf::from_str(TEST_DIR).unwrap().join("qrcode.png");
        save_qrcode(&code, &qrcode_dest).unwrap();
        assert!(qrcode_dest.is_file());

        let qrcode_dest = PathBuf::from_str(TEST_DIR).unwrap().join("qrcode.jpg");
        save_qrcode(&code, &qrcode_dest).unwrap();
        assert!(qrcode_dest.is_file());
    }

    #[test]
    fn test_get_ip() {
        assert!(get_ip().is_ok());
    }

    #[test]
    fn test_download_styles_to_dir() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap().join("styles");
        assert!(download_styles_to_dir(&test_dir).is_ok());

        assert!(test_dir.is_dir());
        // make sure there are folders in there
        let styles = crate::styles::fetch(&test_dir).unwrap();
        assert!(!styles.is_empty())
    }
}
