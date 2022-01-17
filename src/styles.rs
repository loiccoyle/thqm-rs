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
        context.insert("qrcode_button", &template_options.qrcode_button);
        context.insert("shutdown_button", &template_options.shutdown_button);
        context.insert("entries", &template_options.entries);
        context.insert("qrcode", &template_options.qrcode);
        // add stuff to context
        let result = Tera::one_off(&template_contents, &context, true)?;
        debug!("{}", result);
        Ok(result)
    }
}

#[derive(Debug)]
pub struct TemplateOptions {
    pub title: String,
    pub qrcode_button: bool,
    pub shutdown_button: bool,
    pub entries: Vec<String>,
    pub qrcode: Option<String>,
}

impl TemplateOptions {
    pub fn new(
        title: String,
        qrcode_button: bool,
        shutdown_button: bool,
        entries: Vec<String>,
        qrcode: Option<String>,
    ) -> Self {
        Self {
            title,
            qrcode_button,
            shutdown_button,
            entries,
            qrcode,
        }
    }
}
