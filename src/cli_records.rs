use super::*;
use crate::cli_values::*;

/// Create a place entity record.
#[derive(Debug, clap::Args)]
pub(crate) struct NewPlaceArgs {
    pub(crate) place_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Place display name.
    #[arg(long)]
    pub(crate) title: String,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create an organization entity record.
#[derive(Debug, clap::Args)]
pub(crate) struct NewOrganizationArgs {
    pub(crate) organization_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Organization display name.
    #[arg(long)]
    pub(crate) title: String,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create an object entity record.
#[derive(Debug, clap::Args)]
pub(crate) struct NewObjectArgs {
    pub(crate) object_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Object display name.
    #[arg(long)]
    pub(crate) title: String,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a concept entity record.
#[derive(Debug, clap::Args)]
pub(crate) struct NewConceptArgs {
    pub(crate) concept_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Concept display name.
    #[arg(long)]
    pub(crate) title: String,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create an event collection or sequence.
#[derive(Debug, clap::Args)]
pub(crate) struct NewCollectionArgs {
    pub(crate) collection_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Collection title.
    #[arg(long)]
    pub(crate) title: String,

    /// Treat the collection as an ordered sequence.
    #[arg(long)]
    pub(crate) sequence: bool,

    /// Sequence order strategy.
    #[arg(long, value_enum, default_value_t = CollectionOrderArg::ManualThenChronological)]
    pub(crate) order: CollectionOrderArg,

    /// Event ids to include as initial members. May be repeated.
    #[arg(long = "member")]
    pub(crate) members: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a relationship between two person entities.
#[derive(Debug, clap::Args)]
pub(crate) struct NewRelationshipArgs {
    pub(crate) relationship_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Optional relationship display title. Omitted relationships use inferred display labels.
    #[arg(long)]
    pub(crate) title: Option<String>,

    /// Relationship kind, such as biological-parent-child, spouse, sibling, or associate.
    #[arg(long, default_value = "associate")]
    pub(crate) kind: String,

    /// Explicit parent role override for parent/child relationships.
    #[arg(long, value_enum)]
    pub(crate) parent_role: Option<ParentRoleArg>,

    /// Source person id, such as person:example-parent.
    #[arg(long)]
    pub(crate) source: String,

    /// Target person id, such as person:example-child.
    #[arg(long)]
    pub(crate) target: String,

    /// Optional source record ids supporting this relationship. May be repeated.
    #[arg(long = "source-record")]
    pub(crate) source_records: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a source record.
#[derive(Debug, clap::Args)]
pub(crate) struct NewSourceArgs {
    pub(crate) source_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Source title.
    #[arg(long)]
    pub(crate) title: String,

    /// Source kind.
    #[arg(long, default_value = "note")]
    pub(crate) kind: String,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create an assertion record.
#[derive(Debug, clap::Args)]
pub(crate) struct NewAssertionArgs {
    pub(crate) assertion_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Assertion kind.
    #[arg(long, default_value = "claim")]
    pub(crate) kind: String,

    /// Assertion target id, such as event:birth-jon#time or person:jon#name.
    #[arg(long)]
    pub(crate) target: String,

    /// Claimed value. Optional for support assertions that target a specific event field.
    #[arg(long)]
    pub(crate) value: Option<String>,

    /// Assertion source ids. May be repeated.
    #[arg(long = "source")]
    pub(crate) sources: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a timeline view.
#[derive(Debug, clap::Args)]
pub(crate) struct NewTimelineArgs {
    pub(crate) timeline_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// View title.
    #[arg(long)]
    pub(crate) title: String,

    /// Optional subject entity id, such as person:example-person.
    #[arg(long)]
    pub(crate) subject: Option<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a tree view.
#[derive(Debug, clap::Args)]
pub(crate) struct NewTreeArgs {
    pub(crate) tree_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// View title.
    #[arg(long)]
    pub(crate) title: String,

    /// Optional root entity id, such as person:example-person.
    #[arg(long)]
    pub(crate) subject: Option<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a map view.
#[derive(Debug, clap::Args)]
pub(crate) struct NewMapArgs {
    pub(crate) map_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// View title.
    #[arg(long)]
    pub(crate) title: String,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a calendar view.
#[derive(Debug, clap::Args)]
pub(crate) struct NewCalendarArgs {
    pub(crate) calendar_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// View title.
    #[arg(long)]
    pub(crate) title: String,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a visualization view.
#[derive(Debug, clap::Args)]
pub(crate) struct NewVisualizationArgs {
    pub(crate) visualization_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// View title.
    #[arg(long)]
    pub(crate) title: String,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a schema definition record.
#[derive(Debug, clap::Args)]
pub(crate) struct NewSchemaArgs {
    pub(crate) schema_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Schema kind: component, bundle, event, view, or vocab.
    #[arg(long, default_value = "component")]
    pub(crate) kind: SchemaKindArg,

    /// Schema title.
    #[arg(long)]
    pub(crate) title: String,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create an import report TOML file under imports/<kind>/.
#[derive(Debug, clap::Args)]
pub(crate) struct NewImportReportArgs {
    pub(crate) import_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Import kind: gedcom, wikidata, or csv.
    #[arg(long, default_value = "gedcom")]
    pub(crate) kind: ImportKindArg,

    /// Import report title.
    #[arg(long)]
    pub(crate) title: String,

    /// Optional source path under the world root.
    #[arg(long)]
    pub(crate) source_path: Option<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create an assertion connecting a source to a target field or record.
#[derive(Debug, clap::Args)]
pub(crate) struct AttachSourceArgs {
    /// Assertion target, such as event:birth-example-person#time or person:example-person#name.
    pub(crate) target: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Source id/slug supporting the target.
    #[arg(long)]
    pub(crate) source: String,

    /// Assertion kind.
    #[arg(long, default_value = "event-support")]
    pub(crate) kind: String,

    /// Optional claimed value.
    #[arg(long)]
    pub(crate) value: Option<String>,

    /// Assertion confidence.
    #[arg(long, default_value = "medium")]
    pub(crate) confidence: String,

    /// Optional note stored in assertion frontmatter.
    #[arg(long)]
    pub(crate) note: Option<String>,

    /// Assertion slug. Defaults to a slug derived from target and source.
    #[arg(long)]
    pub(crate) slug: Option<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}
