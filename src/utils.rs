use std::{
    io::{self, Read},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use dirs::data_dir;
use image::Luma;
use qrcode::{
    render::{svg, unicode},
    QrCode,
};

static QRCODE_SIZE: u32 = 256;

/// Determine the system's data directory.
pub fn get_data_dir() -> Result<PathBuf> {
    Ok(data_dir()
        .ok_or_else(|| anyhow!("Failed to get default data directory."))?
        .join("thqm"))
}

/// Determine the system's config directory.
// pub fn get_config_dir() -> Result<PathBuf> {
//     Ok(preference_dir()
//         .ok_or_else(|| anyhow!("Failed to get default config directory."))?
//         .join("thqm"))
// }

/// Read stdin.
pub fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Get the local ip.
pub fn get_ip(interface: Option<&str>) -> Result<String> {
    // This currently fails if a "tun" interface is present.
    // There is an issue in the "local_ip_address" crate.
    if let Some(ifa_name) = interface {
        let interfaces = local_ip_address::list_afinet_netifas()?;
        local_ip_address::find_ifa(interfaces, ifa_name)
            .map(|(_, ip)| ip.to_string())
            .ok_or_else(|| anyhow!(format!("Failed to get ip for interface: {:?}", interface)))
    } else {
        Ok(local_ip_address::local_ip().map(|s| s.to_string())?)
    }
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
    if username.is_some() && password.is_some() {
        format!(
            "http://{username}:{password}@{host}:{port}",
            username = username.unwrap(),
            password = password.unwrap(),
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
pub fn save_qrcode<Q>(code: &QrCode, dest: Q) -> Result<()>
where
    Q: AsRef<Path>,
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
    use std::str::FromStr;

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
        assert!(get_ip(None).is_ok());
    }
    #[test]
    fn test_get_ip_missing() {
        assert!(!get_ip(Some("missing_interface")).is_ok());
    }
}
