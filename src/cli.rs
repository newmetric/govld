use clap::Parser;
use std::{
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

use color_eyre::eyre;

/// `govld` is a code tool that adds/replaces code 'Go' according to a pre-written manifest files.
///
/// - based `tree-sitter`
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    sub_commands: Commands,
}

impl Cli {
    pub fn run() -> eyre::Result<()> {
        let args = Self::parse();

        match args.sub_commands {
            Commands::Patch { patch_type, paths } => {
                todo!()
            }
            Commands::Generate => Err(eyre::eyre!("Not implemented yet")),
        }
    }
}

#[non_exhaustive]
#[derive(clap::Subcommand, Debug)]
enum Commands {
    ///
    #[clap(name = "patch")]
    Patch {
        /// - "": no vendoring
        /// - "vendor": automatically creates(run) `go mod vendor`
        /// - custom-dir(path: e.g. "./vendor"): applies govld on the spcified directory
        #[arg(long, short, default_value_t = PatchType::CreateVendor)]
        patch_type: PatchType,

        #[arg(long, short)]
        paths: Vec<PathBuf>,
    },

    #[clap(name = "generate")]
    Generate,
}

#[derive(Clone, Debug)]
pub enum PatchType {
    /// no-vendoring, but assuming it already exists.
    NoVendor,
    /// automatically creates(run) `go mod vendor`
    CreateVendor,
    /// applies govld on the spcified directory
    CustomDir(PathBuf),
}

impl FromStr for PatchType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(PatchType::NoVendor),
            "vendor" => Ok(PatchType::CreateVendor),
            s => {
                let path = Path::new(s);
                if path.is_dir() {
                    Ok(PatchType::CustomDir(path.to_path_buf()))
                } else {
                    Err(format!("{} is not a directory", s))
                }
            }
        }
    }
}

impl fmt::Display for PatchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PatchType::CreateVendor => write!(f, "vendor"),
            PatchType::NoVendor => write!(f, ""),
            PatchType::CustomDir(dir) => write!(f, "{}", dir.display()),
        }
    }
}
