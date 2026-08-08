use super::*;

/// Point a world's world.toml at the active primary GEDCOM file.
#[derive(Debug, clap::Args)]
pub(crate) struct SetGedcomArgs {
    /// GEDCOM path under the world root, e.g. imports/gedcom/family.ged.
    pub(crate) path: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Import strategy metadata. Currently link/import/merge are planning labels.
    #[arg(long, default_value = "link")]
    pub(crate) strategy: String,

    /// Update world.toml even if the GEDCOM file does not exist yet.
    #[arg(long)]
    pub(crate) allow_missing: bool,
}

/// Ingest a GEDCOM file into first-pass world records.
#[derive(Debug, clap::Args)]
pub(crate) struct IngestGedcomArgs {
    /// GEDCOM path under the world root, e.g. imports/gedcom/family.ged.
    pub(crate) path: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Overwrite generated people/events/places/relationships/source files if present.
    #[arg(long)]
    pub(crate) force: bool,
}
