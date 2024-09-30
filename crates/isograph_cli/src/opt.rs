use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Opt {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[command(flatten)]
    pub compile: CompileCommand,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Compile(CompileCommand),
    Lsp(LspCommand),
    CreateConfig(CreateConfigFileCommand),
}

/// Compile
#[derive(Debug, Args)]
pub(crate) struct CompileCommand {
    #[arg(long)]
    pub watch: bool,

    /// Compile using this config file. If not provided, searches for a config in
    /// package.json under the `isograph` key.
    #[arg(long)]
    pub config: Option<PathBuf>,
}

/// LSP
#[derive(Debug, Args)]
pub(crate) struct LspCommand {
    /// Compile using this config file. If not provided, searches for a config in
    /// package.json under the `isograph` key.
    #[arg(long)]
    pub config: Option<PathBuf>,
}

/// Command to create the isograph.config.json file
#[derive(Debug, Args)]
pub(crate) struct CreateConfigFileCommand {
    /// The location to place the config file that will be created.
    #[arg(long, default_value = "./")]
    pub config: PathBuf,

    /// The value to use in the `project_root` field.
    #[arg(long, default_value = "./src/components")]
    pub project_root: PathBuf,

    /// The value to use in the `artifact_directory` field.
    /// If not provided, defaults to the project root.
    #[arg(long)]
    pub artifact_directory: Option<PathBuf>,

    /// The absolute path to the GraphQL schema file.
    #[arg(long)]
    pub schema: PathBuf,

    /// The absolute path to the schema extensions files.
    #[arg(long)]
    pub schema_extensions: Option<Vec<PathBuf>>,
    // TODO: Support `options`?
}
