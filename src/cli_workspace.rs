use super::*;

/// Create a starter Kleio workspace with a default world.
#[derive(Debug, clap::Args)]
pub(crate) struct InitWorkspaceArgs {
    /// Workspace directory to create. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug used under worlds/<slug> and in ids.
    #[arg(long, default_value = DEFAULT_WORLD_SLUG)]
    pub(crate) world: String,

    /// Human-readable world title.
    #[arg(long, default_value = "Default world")]
    pub(crate) title: String,

    /// Starter person slug used in filenames and ids.
    #[arg(long, default_value = "example-person")]
    pub(crate) person_slug: String,

    /// Starter person preferred/display name.
    #[arg(
        long = "preferred-name",
        alias = "person-name",
        default_value = "Example Person"
    )]
    pub(crate) person_name: String,

    /// Optional starter birth date, such as 1900-01-01.
    #[arg(long)]
    pub(crate) birth_date: Option<String>,

    /// Optional starter birth location for the starter birth event.
    #[arg(long)]
    pub(crate) birth_location: Option<String>,

    /// Optional starter birth latitude.
    #[arg(long)]
    pub(crate) birth_latitude: Option<f64>,

    /// Optional starter birth longitude.
    #[arg(long)]
    pub(crate) birth_longitude: Option<f64>,

    /// Overwrite existing starter files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Alias for init-workspace while older local scripts migrate.
#[derive(Debug, clap::Args)]
pub(crate) struct InitArgs {
    /// Workspace directory to create. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug used under worlds/<slug> and in ids.
    #[arg(long, default_value = DEFAULT_WORLD_SLUG)]
    pub(crate) project_id: String,

    /// Human-readable world title.
    #[arg(long, default_value = "Default world")]
    pub(crate) title: String,

    /// Starter person slug used in filenames and ids.
    #[arg(long, default_value = "example-person")]
    pub(crate) person_slug: String,

    /// Starter person preferred/display name.
    #[arg(
        long = "preferred-name",
        alias = "person-name",
        default_value = "Example Person"
    )]
    pub(crate) person_name: String,

    /// Optional starter birth date, such as 1900-01-01.
    #[arg(long)]
    pub(crate) birth_date: Option<String>,

    /// Optional starter birth location for the starter birth event.
    #[arg(long)]
    pub(crate) birth_location: Option<String>,

    /// Optional starter birth latitude.
    #[arg(long)]
    pub(crate) birth_latitude: Option<f64>,

    /// Optional starter birth longitude.
    #[arg(long)]
    pub(crate) birth_longitude: Option<f64>,

    /// Overwrite existing starter files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create an empty world under worlds/<world>.
#[derive(Debug, clap::Args)]
pub(crate) struct NewWorldArgs {
    /// World slug used under worlds/<slug> and in ids.
    pub(crate) world: String,

    /// Workspace directory. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// Human-readable world title.
    #[arg(long)]
    pub(crate) title: Option<String>,

    /// Make this world the workspace default after creating/registering it.
    #[arg(long)]
    pub(crate) set_default: bool,

    /// Include the same starter person/event/views used by init-workspace.
    #[arg(long)]
    pub(crate) starter: bool,

    /// Starter person slug used when --starter is set.
    #[arg(long, default_value = "example-person")]
    pub(crate) person_slug: String,

    /// Starter person preferred/display name used when --starter is set.
    #[arg(
        long = "preferred-name",
        alias = "person-name",
        default_value = "Example Person"
    )]
    pub(crate) person_name: String,

    /// Optional starter birth date used when --starter is set.
    #[arg(long)]
    pub(crate) birth_date: Option<String>,

    /// Optional starter birth location used when --starter is set.
    #[arg(long)]
    pub(crate) birth_location: Option<String>,

    /// Optional starter birth latitude used when --starter is set.
    #[arg(long)]
    pub(crate) birth_latitude: Option<f64>,

    /// Optional starter birth longitude used when --starter is set.
    #[arg(long)]
    pub(crate) birth_longitude: Option<f64>,

    /// Overwrite existing starter files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// List worlds registered in workspace kleio.toml.
#[derive(Debug, clap::Args)]
pub(crate) struct ListWorldsArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,
}

/// Set the workspace default world.
#[derive(Debug, clap::Args)]
pub(crate) struct SetDefaultWorldArgs {
    /// World slug to make the default.
    pub(crate) world: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,
}

/// Show progressive self-authoring suggestions for building a family tree.
#[derive(Debug, clap::Args)]
pub(crate) struct GuideArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Person slug/id to focus suggestions around. Defaults to tree root or first person.
    #[arg(long)]
    pub(crate) person: Option<String>,
}
