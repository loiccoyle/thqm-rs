use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    if std::env::var("DOCS_RS").is_ok() {
        // exit if running in docs.rs
        return Ok(());
    }

    let completions_dir = env::current_dir()?.join("completions");
    let mut cmd = Arguments::command();

    for &shell in Shell::value_variants() {
        let path = generate_to(shell, &mut cmd, "thqm", &completions_dir)?;
        println!("cargo:warning=completion file is generated: {path:?}");
    }

    Ok(())
}
