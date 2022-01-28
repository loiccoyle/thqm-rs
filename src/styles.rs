use std::fs::DirEntry;
use std::{fs, path::Path, path::PathBuf};

use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use log::debug;
use tar::Archive;
use tera::{Context, Tera};

const INCLUDED_STYLES_TAR_GZ: &[u8] = include_bytes!("styles.tar.gz");

fn is_style_entry(style_folder: &DirEntry) -> bool {
    let style_folder = style_folder.path();
    is_style(&style_folder)
}

/// Check the directory to see if it has the minimum requirements to be a style
/// template.
/// Returns `true` if `style_folder`/template/index.html exists.
pub fn is_style(style_folder: &Path) -> bool {
    style_folder.is_dir() & style_folder.join("template").join("index.html").is_file()
}

/// Fetch the available styles in a directory.
pub fn fetch(data_dir: &Path) -> Result<Vec<String>> {
    if !data_dir.is_dir() {
        return Err(anyhow!(format!(
            "Data folder {} does not exist.",
            data_dir.display()
        )));
    }

    Ok(fs::read_dir(data_dir)?
        .filter(|entry| is_style_entry(entry.as_ref().unwrap()))
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(String::from))
            })
        })
        .collect::<Vec<String>>())
}

/// Extract the included styles to the data directory.
pub fn init(data_dir: &Path) -> Result<()> {
    debug!("data_dir: {}", data_dir.display());
    if !data_dir.is_dir() {
        fs::create_dir(data_dir)?;
        debug!("Created data folder {}", data_dir.display());
        // Decompress the included styles to data folder.
        let tar = GzDecoder::new(INCLUDED_STYLES_TAR_GZ);
        let mut archive = Archive::new(tar);
        archive.unpack(data_dir)?;
        debug!("Decompressed styles.");
    } else {
        debug!("data_dir '{}' already exists.", data_dir.display());
    }
    Ok(())
}

#[derive(Debug)]
pub struct Style {
    pub base_path: PathBuf,
    pub template_options: Option<TemplateOptions>,
}

impl Style {
    pub fn new(base_path: PathBuf, template_options: Option<TemplateOptions>) -> Result<Self> {
        if !is_style(&base_path) {
            return Err(anyhow!(
                "{} is not a valid style directory.",
                base_path.display()
            ));
        }
        Ok(Self {
            base_path,
            template_options,
        })
    }

    pub fn from_style_name(
        data_dir: PathBuf,
        style_name: String,
        template_options: Option<TemplateOptions>,
    ) -> Result<Self, anyhow::Error> {
        let base_path = data_dir.join(style_name);
        if !base_path.is_dir() {
            return Err(anyhow!(format!(
                "Style path '{}' not found.",
                base_path.display()
            )));
        }
        Ok(Self {
            base_path,
            template_options,
        })
    }

    /// Get the path of the style's 'index.html' template.
    pub fn template_path(&self) -> Result<PathBuf, anyhow::Error> {
        let template_path = self.base_path.join("template").join("index.html");
        if !template_path.is_file() {
            return Err(anyhow!(format!(
                "Template path '{}' not found.",
                template_path.display()
            )));
        }
        Ok(template_path)
    }

    /// Set the style's template options.
    pub fn set_options(&mut self, template_options: TemplateOptions) {
        self.template_options = Some(template_options);
    }

    /// Render a style's index.html
    pub fn render(&self) -> Result<String> {
        let template_path = self.template_path()?;
        let template_options = self
            .template_options
            .as_ref()
            .ok_or_else(|| anyhow!("No template options set."))?;
        let template_contents = fs::read_to_string(template_path)?;
        let mut context = Context::new();

        // TODO: implement a macro to do this.
        context.insert("title", &template_options.title);
        context.insert("no_qrcode", &template_options.no_qrcode);
        context.insert("no_shutdown", &template_options.no_shutdown);
        context.insert("entries", &template_options.entries);
        context.insert("qrcode_svg", &template_options.qrcode_svg);
        let result = Tera::one_off(&template_contents, &context, true)?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct TemplateOptions {
    pub title: String,
    pub no_qrcode: bool,
    pub no_shutdown: bool,
    pub entries: Vec<String>,
    pub qrcode_svg: Option<String>,
}

impl TemplateOptions {
    pub fn new(
        title: String,
        no_qrcode: bool,
        no_shutdown: bool,
        entries: Vec<String>,
        qrcode_svg: Option<String>,
    ) -> Self {
        Self {
            title,
            no_qrcode,
            no_shutdown,
            entries,
            qrcode_svg,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;
    use std::str::FromStr;

    use super::*;
    use crate::utils::get_data_dir;
    use ctor;

    static TEST_DIR: &str = "./test_data_dir/";

    #[cfg(test)]
    #[ctor::ctor]
    fn setup() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        init(&test_dir).unwrap();
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
    fn test_init() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        init(&test_dir).unwrap();
    }

    #[test]
    fn test_fetch() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        let available_styles = fetch(&test_dir).unwrap();
        assert!(available_styles.len() >= 3);
    }

    #[test]
    #[should_panic]
    fn test_fetch_bad_dir() {
        let test_dir = PathBuf::from_str("/some/dir/that/doesnt/exist/").unwrap();
        fetch(&test_dir).unwrap();
    }


    #[test]
    fn test_style_from_name() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        let style = Style::from_style_name(test_dir.clone(), "default".to_string(), None).unwrap();
        assert_eq!(style.base_path, test_dir.join("default"));
    }

    #[test]
    fn test_style_template_path() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        let style = Style::from_style_name(test_dir.clone(), "default".to_string(), None).unwrap();
        let template_path = style.template_path().unwrap();
        assert_eq!(
            template_path,
            test_dir.join("default").join("template").join("index.html")
        );
    }

    #[test]
    fn style_set_option() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        let mut style =
            Style::from_style_name(test_dir.clone(), "default".to_string(), None).unwrap();
        let entries = vec!["a".to_string(), "b".to_string()];
        let options = TemplateOptions::new("test".to_string(), false, true, entries, None);
        style.set_options(options);
    }

    #[test]
    #[should_panic]
    fn test_style_render_missing_options() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        let style = Style::from_style_name(test_dir.clone(), "default".to_string(), None).unwrap();
        style.render().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_style_panic() {
        let test_dir = get_data_dir().unwrap();
        Style::from_style_name(test_dir, "missing_style".to_string(), None).unwrap();
    }
}
