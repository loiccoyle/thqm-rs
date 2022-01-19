use std::fs::DirEntry;
use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use log::debug;
use tar::Archive;
use tera::{Context, Tera};

const INCLUDED_STYLES_TAR_GZ: &'static [u8] = include_bytes!("styles.tar.gz");

// Checks the folder to see if it has the minimum requirements to be a style
// template.
fn is_style(style_folder: &DirEntry) -> bool {
    let style_folder = style_folder.path();
    return style_folder.is_dir() & style_folder.join("template").join("index.html").is_file();
}

///Fetches the available styles.
pub fn fetch(data_dir: &PathBuf) -> Result<Vec<String>> {
    if !data_dir.is_dir() {
        return Err(anyhow!(format!(
            "Data folder {} does not exist.",
            data_dir.display()
        )));
    }

    Ok(fs::read_dir(data_dir)?
        .filter(|entry| is_style(entry.as_ref().unwrap()))
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>())
}

///Extract the styles to the data folder.
pub fn init(data_dir: &PathBuf) -> Result<()> {
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
    pub fn new(base_path: PathBuf, template_options: Option<TemplateOptions>) -> Self {
        Self {
            base_path,
            template_options,
        }
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

    ///Set the style's template options.
    pub fn set_options(&mut self, template_options: TemplateOptions) {
        self.template_options = Some(template_options);
    }

    ///Render a style's index.html
    ///
    ///# Arguments
    ///* `style_path` - the path of the base directory of the style.
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
    use std::fs::create_dir;
    use std::fs::remove_dir_all;
    use std::panic;
    use std::str::FromStr;

    use super::*;
    use crate::utils::get_data_dir;

    static TEST_DIR: &str = "./test_data_dir/";

    fn setup() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        init(&test_dir).unwrap();
    }

    fn teardown() {
        let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
        if test_dir.is_dir() {
            remove_dir_all(test_dir).unwrap();
        }
    }

    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        setup();
        let result = panic::catch_unwind(|| test());
        teardown();

        if let Err(err) = result {
            panic::resume_unwind(err);
        }
    }

    // TODO: fix these tests, currently there is some sort of io race happening
    // between the setup, test and teardown
    #[test]
    fn test_init() {
        run_test(|| {})
    }

    #[test]
    fn test_fetch() {
        run_test(|| {
            let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
            let available_types = fetch(&test_dir).unwrap();
            assert!(available_types.len() >= 3);
        })
    }

    #[test]
    #[should_panic]
    fn test_fetch_bad_dir() {
        let test_dir = PathBuf::from_str("/some/dir/that/doesnt/exist/").unwrap();
        fetch(&test_dir).unwrap();
    }

    #[test]
    fn test_style() {
        Style::new(PathBuf::from_str("/some/style/path").unwrap(), None);

        let test_dir = get_data_dir().unwrap();
        Style::from_style_name(test_dir, "default".to_string(), None).unwrap();
    }

    #[test]
    fn test_style_from_name() {
        run_test(|| {
            let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
            Style::from_style_name(test_dir, "default".to_string(), None).unwrap();
        })
    }

    #[test]
    fn test_style_template_path() {
        run_test(|| {
            let test_dir = PathBuf::from_str(TEST_DIR).unwrap();
            let style = Style::from_style_name(test_dir, "default".to_string(), None).unwrap();
            style.template_path().unwrap();
        })
    }

    #[test]
    #[should_panic]
    fn test_style_panic() {
        let test_dir = get_data_dir().unwrap();
        Style::from_style_name(test_dir, "missing_style".to_string(), None).unwrap();
    }
}
