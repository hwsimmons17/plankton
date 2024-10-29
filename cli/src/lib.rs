use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use clap::Parser;
use heck::{ToKebabCase, ToSnakeCase};

mod templates;

pub const VERSION: &str = "0.1.0";

#[derive(Debug, Parser)]
#[clap(version = VERSION)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Initializes a workspace.
    Init {
        /// Workspace name
        name: String,
    },
}

pub fn entry(opts: Opts) -> Result<()> {
    process_command(opts)
}

fn process_command(opts: Opts) -> Result<()> {
    match opts.command {
        Command::Init { name } => init(name),
    }
}

fn init(name: String) -> Result<()> {
    println!("Initializing workspace: {}", name);

    let rust_name = name.to_snake_case();
    let project_name = if name == rust_name {
        rust_name.clone()
    } else {
        name.to_kebab_case()
    };

    // Additional keywords that have not been added to the `syn` crate as reserved words
    // https://github.com/dtolnay/syn/pull/1098
    let extra_keywords = ["async", "await", "try"];
    // Anchor converts to snake case before writing the program name
    if syn::parse_str::<syn::Ident>(&rust_name).is_err()
        || extra_keywords.contains(&rust_name.as_str())
    {
        return Err(anyhow!(
            "Plankton workspace name must be a valid Rust identifier. It may not be a Rust reserved word, start with a digit, or include certain disallowed characters. See https://doc.rust-lang.org/reference/identifiers.html for more detail.",
        ));
    }

    fs::create_dir(&project_name)?;
    std::env::set_current_dir(&project_name)?;
    fs::create_dir_all("app")?;

    Ok(())
}

/// Array of (path, content) tuple.
pub type Files = Vec<(PathBuf, String)>;

/// Create files from the given (path, content) tuple array.
///
/// # Example
///
/// ```ignore
/// crate_files(vec![("programs/my_program/src/lib.rs".into(), "// Content".into())])?;
/// ```
pub fn create_files(files: &Files) -> Result<()> {
    for (path, content) in files {
        let path = path
            .display()
            .to_string()
            .replace('/', std::path::MAIN_SEPARATOR_STR);
        let path = Path::new(&path);
        if path.exists() {
            continue;
        }

        match path.extension() {
            Some(_) => {
                fs::create_dir_all(path.parent().unwrap())?;
                fs::write(path, content)?;
            }
            None => fs::create_dir_all(path)?,
        }
    }

    Ok(())
}
