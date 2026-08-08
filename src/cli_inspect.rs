use super::*;
use crate::cli_values::*;

/// List person records in a world.
#[derive(Debug, clap::Args)]
pub(crate) struct ListPeopleArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Case-insensitive filter matched against id, name, or path.
    #[arg(long)]
    pub(crate) filter: Option<String>,

    /// Redact ids, names, and paths in output.
    #[arg(long)]
    pub(crate) redact: bool,
}

/// List event records in a world.
#[derive(Debug, clap::Args)]
pub(crate) struct ListEventsArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Case-insensitive filter matched against id, title, event type, or path.
    #[arg(long)]
    pub(crate) filter: Option<String>,

    /// Redact ids, titles, and paths in output.
    #[arg(long)]
    pub(crate) redact: bool,
}

/// List source records in a world.
#[derive(Debug, clap::Args)]
pub(crate) struct ListSourcesArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Case-insensitive filter matched against id, title, source kind, or path.
    #[arg(long)]
    pub(crate) filter: Option<String>,

    /// Redact ids, titles, and paths in output.
    #[arg(long)]
    pub(crate) redact: bool,
}

/// List saved views in a world.
#[derive(Debug, clap::Args)]
pub(crate) struct ListViewsArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Optional view kind filter: timeline, tree, map, calendar, or visualization.
    #[arg(long)]
    pub(crate) kind: Option<ViewKindArg>,
}

/// Print a small text visualization of nearby family relationships.
#[derive(Debug, clap::Args)]
pub(crate) struct TreeSketchArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Person slug/id to focus. Defaults to the tree root or first person.
    #[arg(long)]
    pub(crate) person: Option<String>,

    /// Relationship depth to include for ancestor/descendant expansion.
    #[arg(long, default_value_t = 1)]
    pub(crate) depth: usize,

    /// Redact person names in output.
    #[arg(long)]
    pub(crate) redact: bool,
}

/// Validate world files without writing build outputs.
#[derive(Debug, clap::Args)]
pub(crate) struct ValidateArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,
}

/// Check local media/source file references.
#[derive(Debug, clap::Args)]
pub(crate) struct CheckMediaArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// List present references as well as missing references.
    #[arg(long)]
    pub(crate) all: bool,

    /// Redact paths and file-reference details in output.
    #[arg(long)]
    pub(crate) redact: bool,
}

/// Check authored world records and report actionable warnings.
#[derive(Debug, clap::Args)]
pub(crate) struct DoctorArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Exit with failure when warnings are found.
    #[arg(long)]
    pub(crate) strict: bool,

    /// Diagnostic level to show.
    #[arg(long, value_enum, default_value_t = DoctorLevelArg::Complete)]
    pub(crate) level: DoctorLevelArg,

    /// Redact ids, paths, and diagnostic details in output.
    #[arg(long)]
    pub(crate) redact: bool,
}

/// Summarize authored world records and common attention items.
#[derive(Debug, clap::Args)]
pub(crate) struct SummaryArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Redact names, paths, and warning details in output.
    #[arg(long)]
    pub(crate) redact: bool,
}
