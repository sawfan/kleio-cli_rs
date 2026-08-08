use super::*;
use crate::cli_values::*;

/// Create a person record, optionally with a starter birth event.
#[derive(Debug, clap::Args)]
pub(crate) struct NewPersonArgs {
    /// Person slug used in filename and id, e.g. alex-example.
    pub(crate) person_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Person preferred/display name. Defaults to first + family inferred from the person slug.
    #[arg(long = "preferred-name", alias = "person-name")]
    pub(crate) person_name: Option<String>,

    /// Optional sex marker used for genealogy display labels.
    #[arg(long, value_enum)]
    pub(crate) sex: Option<SexArg>,

    /// Optional birth date for the starter birth event.
    #[arg(long)]
    pub(crate) birth_date: Option<String>,

    /// Optional birth location for the starter birth event.
    #[arg(long)]
    pub(crate) birth_location: Option<String>,

    /// Optional birth latitude for the starter birth event.
    #[arg(long)]
    pub(crate) birth_latitude: Option<f64>,

    /// Optional birth longitude for the starter birth event.
    #[arg(long)]
    pub(crate) birth_longitude: Option<f64>,

    /// Skip creating the starter birth event.
    #[arg(long)]
    pub(crate) no_birth_event: bool,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Add a relative to an existing person with one command.
#[derive(Debug, clap::Args)]
pub(crate) struct AddRelativeArgs {
    /// New relative person slug used in filename and id.
    pub(crate) relative_slug: String,

    /// Relationship of the new person to the existing person.
    #[arg(long, value_enum)]
    pub(crate) relation: RelativeArg,

    /// Existing person slug/id the new relative should connect to.
    #[arg(long = "of")]
    pub(crate) person: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// New relative preferred/display name. Defaults to first + family inferred from the relative slug.
    #[arg(long = "preferred-name", alias = "person-name")]
    pub(crate) person_name: Option<String>,

    /// Optional sex marker used for genealogy display labels.
    #[arg(long, value_enum)]
    pub(crate) sex: Option<SexArg>,

    /// Optional birth date for the starter birth event.
    #[arg(long)]
    pub(crate) birth_date: Option<String>,

    /// Optional birth location for the starter birth event.
    #[arg(long)]
    pub(crate) birth_location: Option<String>,

    /// Optional birth latitude for the starter birth event.
    #[arg(long)]
    pub(crate) birth_latitude: Option<f64>,

    /// Optional birth longitude for the starter birth event.
    #[arg(long)]
    pub(crate) birth_longitude: Option<f64>,

    /// Skip creating the starter birth event.
    #[arg(long)]
    pub(crate) no_birth_event: bool,

    /// Override the default relationship kind for the selected relation.
    #[arg(long)]
    pub(crate) kind: Option<String>,

    /// Explicit parent role override for parent/child relationships.
    #[arg(long, value_enum)]
    pub(crate) parent_role: Option<ParentRoleArg>,

    /// Optional source record ids supporting this relationship. May be repeated.
    #[arg(long = "source-record")]
    pub(crate) source_records: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Connect two existing people with a relationship.
#[derive(Debug, clap::Args)]
pub(crate) struct ConnectRelativeArgs {
    /// Existing relative slug/id to connect.
    pub(crate) relative: String,

    /// Relationship of the relative to the existing person.
    #[arg(long, value_enum)]
    pub(crate) relation: RelativeArg,

    /// Existing person slug/id the relative should connect to.
    #[arg(long = "of")]
    pub(crate) person: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Override the default relationship kind for the selected relation.
    #[arg(long)]
    pub(crate) kind: Option<String>,

    /// Optional relationship title. Defaults to a generic title based on relationship kind.
    #[arg(long)]
    pub(crate) title: Option<String>,

    /// Explicit parent role override for parent/child relationships.
    #[arg(long, value_enum)]
    pub(crate) parent_role: Option<ParentRoleArg>,

    /// Relationship slug. Defaults to one derived from source, target, and relation.
    #[arg(long)]
    pub(crate) slug: Option<String>,

    /// Optional source record ids supporting this relationship. May be repeated.
    #[arg(long = "source-record")]
    pub(crate) source_records: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Add a new spouse for an existing person, creating the person, marriage event, and spouse relationship.
#[derive(Debug, clap::Args)]
pub(crate) struct AddSpouseArgs {
    /// New spouse person slug used in filename and id.
    pub(crate) spouse_slug: String,

    /// Existing spouse person slug/id.
    #[arg(long = "of")]
    pub(crate) person: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// New spouse preferred/display name. Defaults to first + family inferred from the spouse slug.
    #[arg(long = "preferred-name", alias = "person-name")]
    pub(crate) person_name: Option<String>,

    /// Optional sex marker used for genealogy display labels.
    #[arg(long, value_enum)]
    pub(crate) sex: Option<SexArg>,

    /// Optional birth date for the starter birth event.
    #[arg(long)]
    pub(crate) birth_date: Option<String>,

    /// Optional birth location for the starter birth event.
    #[arg(long)]
    pub(crate) birth_location: Option<String>,

    /// Optional birth latitude for the starter birth event.
    #[arg(long)]
    pub(crate) birth_latitude: Option<f64>,

    /// Optional birth longitude for the starter birth event.
    #[arg(long)]
    pub(crate) birth_longitude: Option<f64>,

    /// Skip creating the starter birth event.
    #[arg(long)]
    pub(crate) no_birth_event: bool,

    /// Marriage event slug. Defaults to <existing-spouse-slug>-<new-spouse-slug>-marriage.
    #[arg(long)]
    pub(crate) marriage_slug: Option<String>,

    /// Optional marriage event title. When omitted, the marriage label is derived from participants.
    #[arg(long)]
    pub(crate) title: Option<String>,

    /// Marriage place entity ids. May be repeated.
    #[arg(long = "place")]
    pub(crate) places: Vec<String>,

    /// Inline marriage location label.
    #[arg(long)]
    pub(crate) location: Option<String>,

    /// Inline marriage location latitude.
    #[arg(long)]
    pub(crate) latitude: Option<f64>,

    /// Inline marriage location longitude.
    #[arg(long)]
    pub(crate) longitude: Option<f64>,

    /// Marriage time/date text.
    #[arg(long)]
    pub(crate) time: Option<String>,

    /// Marriage date precision. Inferred from --time when omitted.
    #[arg(long)]
    pub(crate) date_precision: Option<String>,

    /// Event source ids. May be repeated.
    #[arg(long = "source")]
    pub(crate) sources: Vec<String>,

    /// Optional source record ids for the spouse relationship. May be repeated.
    #[arg(long = "relationship-source")]
    pub(crate) relationship_sources: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}
