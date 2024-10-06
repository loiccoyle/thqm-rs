use clap::Parser;
use clap_verbosity_flag::Verbosity;

use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, author = "Loic Coyle")]
#[command(about = "A simple HTTP server to serve a dynamic menu web page.

thqm generates a menu based on the standard input and writes selections to standard output.

See https://github.com/loiccoyle/thqm-rs/tree/main/examples for script examples.

Basic usage:
$ echo 'Option 1\\nOption 2' | thqm -u |
    while IFS= read -r sel; do
      case $sel in
      'Option 1') echo 'hello';;
      'Option 2') echo 'world';;
      *) echo \"$sel\";;
      esac
    done")]
pub struct Arguments {
    /// The port to listen on.
    #[arg(short = 'p', long = "port", default_value = "8000")]
    pub port: u64,
    /// The username to authenticate with.
    #[arg(short = 'U', long)]
    pub username: Option<String>,
    /// The password to authenticate with.
    #[arg(short = 'P', long = "password")]
    pub password: Option<String>,
    /// The entry separator.
    #[arg(short = 'S', long = "separator", default_value = "\n")]
    pub separator: String,
    /// The page title.
    #[arg(short = 't', long = "title", default_value = "thqm")]
    pub title: String,
    /// The page style.
    #[arg(short = 's', long = "style", default_value = "default")]
    pub style: String,
    /// Specify style with its root directory.
    #[arg(long, value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    pub style_dir: Option<PathBuf>,
    /// Show the qrcode in terminal.
    #[arg(short = 'Q', long = "qrcode")]
    pub show_qrcode: bool,
    /// Save the qrcode image to file.
    #[arg(long, value_name = "PATH", value_hint = clap::ValueHint::FilePath)]
    pub save_qrcode: Option<PathBuf>,
    /// Show the page url.
    #[arg(short = 'u', long = "url")]
    pub show_url: bool,
    /// Shutdown server after first selection.
    #[arg(short = 'o', long)]
    pub oneshot: bool,
    /// Show custom input field.
    #[arg(short = 'c', long)]
    pub custom_input: bool,
    /// List available page styles.
    #[arg(long)]
    pub list_styles: bool,
    /// Don't allow the server to be shutdown from the page.
    #[arg(long, default_value_t = false)]
    pub no_shutdown: bool,
    /// Don't allow the qrcode to be shown in the page.
    #[arg(long, default_value_t = false)]
    pub no_qrcode: bool,
    /// Download and install styles to the user data directory.
    #[arg(long)]
    pub install_styles: bool,
    #[command(flatten)]
    pub verbose: Verbosity,
}
