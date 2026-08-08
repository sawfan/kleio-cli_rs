use super::*;

/// Print a redacted copy of an authored Markdown/TOML file for sharing diagnostics.
#[derive(Debug, clap::Args)]
pub(crate) struct RedactFileArgs {
    /// File to read and redact.
    pub(crate) path: PathBuf,

    /// Also redact Markdown body text. By default only structured frontmatter/config values are redacted.
    #[arg(long)]
    pub(crate) redact_body: bool,
}

/// Write a redacted copy of a world for private diagnostics sharing.
#[derive(Debug, clap::Args)]
pub(crate) struct RedactWorldArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Output directory for redacted files.
    #[arg(long)]
    pub(crate) out: PathBuf,

    /// Also redact Markdown body text. By default only structured frontmatter/config values are redacted.
    #[arg(long)]
    pub(crate) redact_body: bool,

    /// Replace existing output directory if it exists.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Print a redacted, pasteable dump of the selected authored world.
#[derive(Debug, clap::Args)]
pub(crate) struct RedactWorldDumpArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Also redact Markdown body text. By default only structured frontmatter/config values are redacted.
    #[arg(long)]
    pub(crate) redact_body: bool,
}

/// Print a redacted authored-file tree for the selected world.
#[derive(Debug, clap::Args)]
pub(crate) struct RedactWorldTreeArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,
}
