use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use kleio::{
    DEFAULT_WORLD_SLUG, LocalAssertionOptions, LocalBirthEventOptions, LocalCollectionKind,
    LocalCollectionOptions, LocalCollectionOrder, LocalEntityKind, LocalEntityOptions,
    LocalEventOptions, LocalImportKind, LocalImportReportOptions, LocalPersonOptions,
    LocalRelationshipOptions, LocalSchemaKind, LocalSchemaOptions, LocalSkeletonOptions,
    LocalSourceOptions, LocalViewKind, LocalViewOptions, LocalWorldBuildOptions, WorkspaceConfig,
    WorkspacePaths, build_local_world_with_options, check_local_media, compile_local_data,
    compile_local_ecs, compile_local_tree, create_local_assertion, create_local_birth_event,
    create_local_collection, create_local_entity, create_local_event, create_local_import_report,
    create_local_person, create_local_relationship, create_local_schema, create_local_source,
    create_local_view, create_workspace_skeleton, create_world_layout, create_world_skeleton,
    doctor_local_world, list_local_views, read_local_data_unvalidated, read_workspace_config,
    resolve_workspace_world_root, resolve_world_build_paths, summarize_local_world,
    validate_local_world, write_local_data_json, write_local_ecs_json, write_local_timeline_json,
    write_local_tree_json_with_view, write_workspace_config,
};
use kleio_gedcom::{
    LocalGedcomIngestOptions, PrimaryGedcomImportOptions, ingest_primary_gedcom_to_world,
    set_primary_gedcom_import,
};

#[derive(Debug)]
struct CliError(String);

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}

fn cli_error(message: impl Into<String>) -> Box<dyn std::error::Error> {
    Box::new(CliError(message.into()))
}

#[derive(Debug, Parser)]
#[command(name = "kleio-cli")]
#[command(about = "Kleio world/workspace local authoring tools")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a starter Kleio workspace with a default world.
    InitWorkspace {
        /// Workspace directory to create. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug used under worlds/<slug> and in ids.
        #[arg(long, default_value = DEFAULT_WORLD_SLUG)]
        world: String,

        /// Human-readable world title.
        #[arg(long, default_value = "Default world")]
        title: String,

        /// Starter person slug used in filenames and ids.
        #[arg(long, default_value = "example-person")]
        person_slug: String,

        /// Starter person preferred/display name.
        #[arg(
            long = "preferred-name",
            alias = "person-name",
            default_value = "Example Person"
        )]
        person_name: String,

        /// Optional starter birth date, such as 1900-01-01.
        #[arg(long)]
        birth_date: Option<String>,

        /// Optional starter birth location for the starter birth event.
        #[arg(long)]
        birth_location: Option<String>,

        /// Optional starter birth latitude.
        #[arg(long)]
        birth_latitude: Option<f64>,

        /// Optional starter birth longitude.
        #[arg(long)]
        birth_longitude: Option<f64>,

        /// Overwrite existing starter files if present.
        #[arg(long)]
        force: bool,
    },

    /// Alias for init-workspace while older local scripts migrate.
    Init {
        /// Workspace directory to create. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug used under worlds/<slug> and in ids.
        #[arg(long, default_value = DEFAULT_WORLD_SLUG)]
        project_id: String,

        /// Human-readable world title.
        #[arg(long, default_value = "Default world")]
        title: String,

        /// Starter person slug used in filenames and ids.
        #[arg(long, default_value = "example-person")]
        person_slug: String,

        /// Starter person preferred/display name.
        #[arg(
            long = "preferred-name",
            alias = "person-name",
            default_value = "Example Person"
        )]
        person_name: String,

        /// Optional starter birth date, such as 1900-01-01.
        #[arg(long)]
        birth_date: Option<String>,

        /// Optional starter birth location for the starter birth event.
        #[arg(long)]
        birth_location: Option<String>,

        /// Optional starter birth latitude.
        #[arg(long)]
        birth_latitude: Option<f64>,

        /// Optional starter birth longitude.
        #[arg(long)]
        birth_longitude: Option<f64>,

        /// Overwrite existing starter files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create an empty world under worlds/<world>.
    NewWorld {
        /// World slug used under worlds/<slug> and in ids.
        world: String,

        /// Workspace directory. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// Human-readable world title.
        #[arg(long)]
        title: Option<String>,

        /// Make this world the workspace default after creating/registering it.
        #[arg(long)]
        set_default: bool,

        /// Include the same starter person/event/views used by init-workspace.
        #[arg(long)]
        starter: bool,

        /// Starter person slug used when --starter is set.
        #[arg(long, default_value = "example-person")]
        person_slug: String,

        /// Starter person preferred/display name used when --starter is set.
        #[arg(
            long = "preferred-name",
            alias = "person-name",
            default_value = "Example Person"
        )]
        person_name: String,

        /// Optional starter birth date used when --starter is set.
        #[arg(long)]
        birth_date: Option<String>,

        /// Optional starter birth location used when --starter is set.
        #[arg(long)]
        birth_location: Option<String>,

        /// Optional starter birth latitude used when --starter is set.
        #[arg(long)]
        birth_latitude: Option<f64>,

        /// Optional starter birth longitude used when --starter is set.
        #[arg(long)]
        birth_longitude: Option<f64>,

        /// Overwrite existing starter files if present.
        #[arg(long)]
        force: bool,
    },

    /// List worlds registered in workspace kleio.toml.
    ListWorlds {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,
    },

    /// Set the workspace default world.
    SetDefaultWorld {
        /// World slug to make the default.
        world: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,
    },

    /// Create a person record, optionally with a starter birth event.
    NewPerson {
        /// Person slug used in filename and id, e.g. alex-example.
        person_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Person preferred/display name. Defaults to first + family inferred from the person slug.
        #[arg(long = "preferred-name", alias = "person-name")]
        person_name: Option<String>,

        /// Optional sex marker used for genealogy display labels.
        #[arg(long, value_enum)]
        sex: Option<SexArg>,

        /// Optional birth date for the starter birth event.
        #[arg(long)]
        birth_date: Option<String>,

        /// Optional birth location for the starter birth event.
        #[arg(long)]
        birth_location: Option<String>,

        /// Optional birth latitude for the starter birth event.
        #[arg(long)]
        birth_latitude: Option<f64>,

        /// Optional birth longitude for the starter birth event.
        #[arg(long)]
        birth_longitude: Option<f64>,

        /// Skip creating the starter birth event.
        #[arg(long)]
        no_birth_event: bool,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Add a relative to an existing person with one command.
    AddRelative {
        /// New relative person slug used in filename and id.
        relative_slug: String,

        /// Relationship of the new person to the existing person.
        #[arg(long, value_enum)]
        relation: RelativeArg,

        /// Existing person slug/id the new relative should connect to.
        #[arg(long = "of")]
        person: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// New relative preferred/display name. Defaults to first + family inferred from the relative slug.
        #[arg(long = "preferred-name", alias = "person-name")]
        person_name: Option<String>,

        /// Optional sex marker used for genealogy display labels.
        #[arg(long, value_enum)]
        sex: Option<SexArg>,

        /// Optional birth date for the starter birth event.
        #[arg(long)]
        birth_date: Option<String>,

        /// Optional birth location for the starter birth event.
        #[arg(long)]
        birth_location: Option<String>,

        /// Optional birth latitude for the starter birth event.
        #[arg(long)]
        birth_latitude: Option<f64>,

        /// Optional birth longitude for the starter birth event.
        #[arg(long)]
        birth_longitude: Option<f64>,

        /// Skip creating the starter birth event.
        #[arg(long)]
        no_birth_event: bool,

        /// Override the default relationship kind for the selected relation.
        #[arg(long)]
        kind: Option<String>,

        /// Explicit parent role override for parent/child relationships.
        #[arg(long, value_enum)]
        parent_role: Option<ParentRoleArg>,

        /// Optional source record ids supporting this relationship. May be repeated.
        #[arg(long = "source-record")]
        source_records: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Connect two existing people with a relationship.
    ConnectRelative {
        /// Existing relative slug/id to connect.
        relative: String,

        /// Relationship of the relative to the existing person.
        #[arg(long, value_enum)]
        relation: RelativeArg,

        /// Existing person slug/id the relative should connect to.
        #[arg(long = "of")]
        person: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Override the default relationship kind for the selected relation.
        #[arg(long)]
        kind: Option<String>,

        /// Optional relationship title. Defaults to a generic title based on relationship kind.
        #[arg(long)]
        title: Option<String>,

        /// Explicit parent role override for parent/child relationships.
        #[arg(long, value_enum)]
        parent_role: Option<ParentRoleArg>,

        /// Relationship slug. Defaults to one derived from source, target, and relation.
        #[arg(long)]
        slug: Option<String>,

        /// Optional source record ids supporting this relationship. May be repeated.
        #[arg(long = "source-record")]
        source_records: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a place entity record.
    NewPlace {
        place_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Place display name.
        #[arg(long)]
        title: String,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create an organization entity record.
    NewOrganization {
        organization_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Organization display name.
        #[arg(long)]
        title: String,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create an object entity record.
    NewObject {
        object_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Object display name.
        #[arg(long)]
        title: String,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a concept entity record.
    NewConcept {
        concept_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Concept display name.
        #[arg(long)]
        title: String,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Add a new spouse for an existing person, creating the person, marriage event, and spouse relationship.
    AddSpouse {
        /// New spouse person slug used in filename and id.
        spouse_slug: String,

        /// Existing spouse person slug/id.
        #[arg(long = "of")]
        person: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// New spouse preferred/display name. Defaults to first + family inferred from the spouse slug.
        #[arg(long = "preferred-name", alias = "person-name")]
        person_name: Option<String>,

        /// Optional sex marker used for genealogy display labels.
        #[arg(long, value_enum)]
        sex: Option<SexArg>,

        /// Optional birth date for the starter birth event.
        #[arg(long)]
        birth_date: Option<String>,

        /// Optional birth location for the starter birth event.
        #[arg(long)]
        birth_location: Option<String>,

        /// Optional birth latitude for the starter birth event.
        #[arg(long)]
        birth_latitude: Option<f64>,

        /// Optional birth longitude for the starter birth event.
        #[arg(long)]
        birth_longitude: Option<f64>,

        /// Skip creating the starter birth event.
        #[arg(long)]
        no_birth_event: bool,

        /// Marriage event slug. Defaults to <existing-spouse-slug>-<new-spouse-slug>-marriage.
        #[arg(long)]
        marriage_slug: Option<String>,

        /// Optional marriage event title. When omitted, the marriage label is derived from participants.
        #[arg(long)]
        title: Option<String>,

        /// Marriage place entity ids. May be repeated.
        #[arg(long = "place")]
        places: Vec<String>,

        /// Inline marriage location label.
        #[arg(long)]
        location: Option<String>,

        /// Inline marriage location latitude.
        #[arg(long)]
        latitude: Option<f64>,

        /// Inline marriage location longitude.
        #[arg(long)]
        longitude: Option<f64>,

        /// Marriage time/date text.
        #[arg(long)]
        time: Option<String>,

        /// Marriage date precision. Inferred from --time when omitted.
        #[arg(long)]
        date_precision: Option<String>,

        /// Event source ids. May be repeated.
        #[arg(long = "source")]
        sources: Vec<String>,

        /// Optional source record ids for the spouse relationship. May be repeated.
        #[arg(long = "relationship-source")]
        relationship_sources: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Add a marriage event and spouse relationship between two people.
    AddMarriage {
        /// First spouse person slug/id.
        first_person: String,

        /// Second spouse person slug/id.
        second_person: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Event slug. Defaults to <first>-<second>-marriage.
        #[arg(long)]
        slug: Option<String>,

        /// Optional event title. When omitted, the marriage label is derived from participants.
        #[arg(long)]
        title: Option<String>,

        /// Place entity ids. May be repeated.
        #[arg(long = "place")]
        places: Vec<String>,

        /// Inline event location label.
        #[arg(long)]
        location: Option<String>,

        /// Inline event location latitude.
        #[arg(long)]
        latitude: Option<f64>,

        /// Inline event location longitude.
        #[arg(long)]
        longitude: Option<f64>,

        /// Event time/date text.
        #[arg(long)]
        time: Option<String>,

        /// Event date precision. Inferred from --time when omitted.
        #[arg(long)]
        date_precision: Option<String>,

        /// Event source ids. May be repeated.
        #[arg(long = "source")]
        sources: Vec<String>,

        /// Optional source record ids for the spouse relationship. May be repeated.
        #[arg(long = "relationship-source")]
        relationship_sources: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Add a divorce event and former-spouse relationship between two people.
    AddDivorce {
        /// First former spouse person slug/id.
        first_person: String,

        /// Second former spouse person slug/id.
        second_person: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Event slug. Defaults to <first>-<second>-divorce.
        #[arg(long)]
        slug: Option<String>,

        /// Optional event title. When omitted, the divorce label is derived from participants.
        #[arg(long)]
        title: Option<String>,

        /// Place entity ids. May be repeated.
        #[arg(long = "place")]
        places: Vec<String>,

        /// Inline event location label.
        #[arg(long)]
        location: Option<String>,

        /// Inline event location latitude.
        #[arg(long)]
        latitude: Option<f64>,

        /// Inline event location longitude.
        #[arg(long)]
        longitude: Option<f64>,

        /// Event time/date text.
        #[arg(long)]
        time: Option<String>,

        /// Event date precision. Inferred from --time when omitted.
        #[arg(long)]
        date_precision: Option<String>,

        /// Event source ids. May be repeated.
        #[arg(long = "source")]
        sources: Vec<String>,

        /// Optional source record ids for the former-spouse relationship. May be repeated.
        #[arg(long = "relationship-source")]
        relationship_sources: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Add a death event for one person.
    AddDeath {
        /// Person slug/id.
        person: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Event slug. Defaults to <person-slug>-death.
        #[arg(long)]
        slug: Option<String>,

        /// Optional event title. When omitted, the death label is derived from the person.
        #[arg(long)]
        title: Option<String>,

        /// Place entity ids. May be repeated.
        #[arg(long = "place")]
        places: Vec<String>,

        /// Inline event location label.
        #[arg(long)]
        location: Option<String>,

        /// Inline event location latitude.
        #[arg(long)]
        latitude: Option<f64>,

        /// Inline event location longitude.
        #[arg(long)]
        longitude: Option<f64>,

        /// Event time/date text.
        #[arg(long)]
        time: Option<String>,

        /// Event date precision. Inferred from --time when omitted.
        #[arg(long)]
        date_precision: Option<String>,

        /// Event source ids. May be repeated.
        #[arg(long = "source")]
        sources: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Add a life event for one or more people.
    AddEvent {
        event_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Event type, such as birth, death, residence, marriage, migration, observation, or moment.
        #[arg(long = "type")]
        event_type: String,

        /// Person participant ids/slugs. May be repeated.
        #[arg(long = "person")]
        people: Vec<String>,

        /// Partner participant id/slug for relationship-like events such as marriage.
        #[arg(long)]
        partner: Option<String>,

        /// Optional event title. When omitted, common event types derive labels from participants.
        #[arg(long)]
        title: Option<String>,

        /// Place entity ids. May be repeated.
        #[arg(long = "place")]
        places: Vec<String>,

        /// Inline event location label.
        #[arg(long)]
        location: Option<String>,

        /// Inline event location latitude.
        #[arg(long)]
        latitude: Option<f64>,

        /// Inline event location longitude.
        #[arg(long)]
        longitude: Option<f64>,

        /// Event time/date text.
        #[arg(long)]
        time: Option<String>,

        /// Event date precision. Inferred from --time when omitted.
        #[arg(long)]
        date_precision: Option<String>,

        /// Event source ids. May be repeated.
        #[arg(long = "source")]
        sources: Vec<String>,

        /// Also create a spouse/partner relationship between the first person and --partner.
        #[arg(long)]
        create_relationship: bool,

        /// Relationship slug to use with --create-relationship.
        #[arg(long)]
        relationship_slug: Option<String>,

        /// Relationship kind to use with --create-relationship.
        #[arg(long, default_value = "spouse")]
        relationship_kind: String,

        /// Optional source record ids for the relationship created by --create-relationship. May be repeated.
        #[arg(long = "relationship-source")]
        relationship_sources: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a semantic event record.
    NewEvent {
        event_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Event type, such as birth, residence, observation, or moment.
        #[arg(long = "type", default_value = "observation")]
        event_type: String,

        /// Optional event title. When omitted, common event types derive labels from participants.
        #[arg(long)]
        title: Option<String>,

        /// Event participant entity ids. May be repeated.
        #[arg(long = "participant")]
        participants: Vec<String>,

        /// Event place entity ids. May be repeated.
        #[arg(long = "place")]
        places: Vec<String>,

        /// Inline event location label.
        #[arg(long)]
        location: Option<String>,

        /// Inline event location latitude.
        #[arg(long)]
        latitude: Option<f64>,

        /// Inline event location longitude.
        #[arg(long)]
        longitude: Option<f64>,

        /// Event time/date text.
        #[arg(long)]
        time: Option<String>,

        /// Event date precision. Inferred from --time when omitted.
        #[arg(long)]
        date_precision: Option<String>,

        /// Event source ids. May be repeated.
        #[arg(long = "source")]
        sources: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create an event collection or sequence.
    NewCollection {
        collection_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Collection title.
        #[arg(long)]
        title: String,

        /// Treat the collection as an ordered sequence.
        #[arg(long)]
        sequence: bool,

        /// Sequence order strategy.
        #[arg(long, value_enum, default_value_t = CollectionOrderArg::ManualThenChronological)]
        order: CollectionOrderArg,

        /// Event ids to include as initial members. May be repeated.
        #[arg(long = "member")]
        members: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a relationship between two person entities.
    NewRelationship {
        relationship_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Optional relationship display title. Omitted relationships use inferred display labels.
        #[arg(long)]
        title: Option<String>,

        /// Relationship kind, such as biological-parent-child, spouse, sibling, or associate.
        #[arg(long, default_value = "associate")]
        kind: String,

        /// Explicit parent role override for parent/child relationships.
        #[arg(long, value_enum)]
        parent_role: Option<ParentRoleArg>,

        /// Source person id, such as person:example-parent.
        #[arg(long)]
        source: String,

        /// Target person id, such as person:example-child.
        #[arg(long)]
        target: String,

        /// Optional source record ids supporting this relationship. May be repeated.
        #[arg(long = "source-record")]
        source_records: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a source record.
    NewSource {
        source_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Source title.
        #[arg(long)]
        title: String,

        /// Source kind.
        #[arg(long, default_value = "note")]
        kind: String,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create an assertion record.
    NewAssertion {
        assertion_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Assertion kind.
        #[arg(long, default_value = "claim")]
        kind: String,

        /// Assertion target id, such as event:birth-jon#time or person:jon#name.
        #[arg(long)]
        target: String,

        /// Claimed value. Optional for support assertions that target a specific event field.
        #[arg(long)]
        value: Option<String>,

        /// Assertion source ids. May be repeated.
        #[arg(long = "source")]
        sources: Vec<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a birth event for an existing person.
    NewBirth {
        /// Person slug used in the person id, e.g. alex-example.
        person_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Person display name used in the event title.
        #[arg(long)]
        person_name: String,

        /// Optional birth date, such as 1900-01-01.
        #[arg(long)]
        birth_date: Option<String>,

        /// Optional birth location, stored inline on the birth event.
        #[arg(long)]
        birth_location: Option<String>,

        /// Optional birth latitude, stored inline on the birth event.
        #[arg(long)]
        birth_latitude: Option<f64>,

        /// Optional birth longitude, stored inline on the birth event.
        #[arg(long)]
        birth_longitude: Option<f64>,

        /// Overwrite existing generated file if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a timeline view.
    NewTimeline {
        timeline_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// View title.
        #[arg(long)]
        title: String,

        /// Optional subject entity id, such as person:example-person.
        #[arg(long)]
        subject: Option<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a tree view.
    NewTree {
        tree_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// View title.
        #[arg(long)]
        title: String,

        /// Optional root entity id, such as person:example-person.
        #[arg(long)]
        subject: Option<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a map view.
    NewMap {
        map_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// View title.
        #[arg(long)]
        title: String,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a calendar view.
    NewCalendar {
        calendar_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// View title.
        #[arg(long)]
        title: String,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a visualization view.
    NewVisualization {
        visualization_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// View title.
        #[arg(long)]
        title: String,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create a schema definition record.
    NewSchema {
        schema_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Schema kind: component, bundle, event, view, or vocab.
        #[arg(long, default_value = "component")]
        kind: SchemaKindArg,

        /// Schema title.
        #[arg(long)]
        title: String,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create an import report TOML file under imports/<kind>/.
    NewImportReport {
        import_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Import kind: gedcom, wikidata, or csv.
        #[arg(long, default_value = "gedcom")]
        kind: ImportKindArg,

        /// Import report title.
        #[arg(long)]
        title: String,

        /// Optional source path under the world root.
        #[arg(long)]
        source_path: Option<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Show progressive self-authoring suggestions for building a family tree.
    Guide {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Person slug/id to focus suggestions around. Defaults to tree root or first person.
        #[arg(long)]
        person: Option<String>,
    },

    /// List person records in a world.
    ListPeople {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Case-insensitive filter matched against id, name, or path.
        #[arg(long)]
        filter: Option<String>,

        /// Redact ids, names, and paths in output.
        #[arg(long)]
        redact: bool,
    },

    /// List event records in a world.
    ListEvents {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Case-insensitive filter matched against id, title, event type, or path.
        #[arg(long)]
        filter: Option<String>,

        /// Redact ids, titles, and paths in output.
        #[arg(long)]
        redact: bool,
    },

    /// List source records in a world.
    ListSources {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Case-insensitive filter matched against id, title, source kind, or path.
        #[arg(long)]
        filter: Option<String>,

        /// Redact ids, titles, and paths in output.
        #[arg(long)]
        redact: bool,
    },

    /// List saved views in a world.
    ListViews {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Optional view kind filter: timeline, tree, map, calendar, or visualization.
        #[arg(long)]
        kind: Option<ViewKindArg>,
    },

    /// Print a small text visualization of nearby family relationships.
    TreeSketch {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Person slug/id to focus. Defaults to the tree root or first person.
        #[arg(long)]
        person: Option<String>,

        /// Relationship depth to include for ancestor/descendant expansion.
        #[arg(long, default_value_t = 1)]
        depth: usize,

        /// Redact person names in output.
        #[arg(long)]
        redact: bool,
    },

    /// Point a world's world.toml at the active primary GEDCOM file.
    SetGedcom {
        /// GEDCOM path under the world root, e.g. imports/gedcom/family.ged.
        path: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Import strategy metadata. Currently link/import/merge are planning labels.
        #[arg(long, default_value = "link")]
        strategy: String,

        /// Update world.toml even if the GEDCOM file does not exist yet.
        #[arg(long)]
        allow_missing: bool,
    },

    /// Ingest a GEDCOM file into first-pass world records.
    IngestGedcom {
        /// GEDCOM path under the world root, e.g. imports/gedcom/family.ged.
        path: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Overwrite generated people/events/places/relationships/source files if present.
        #[arg(long)]
        force: bool,
    },

    /// Create an assertion connecting a source to a target field or record.
    AttachSource {
        /// Assertion target, such as event:birth-example-person#time or person:example-person#name.
        target: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Source id/slug supporting the target.
        #[arg(long)]
        source: String,

        /// Assertion kind.
        #[arg(long, default_value = "event-support")]
        kind: String,

        /// Optional claimed value.
        #[arg(long)]
        value: Option<String>,

        /// Assertion confidence.
        #[arg(long, default_value = "medium")]
        confidence: String,

        /// Optional note stored in assertion frontmatter.
        #[arg(long)]
        note: Option<String>,

        /// Assertion slug. Defaults to a slug derived from target and source.
        #[arg(long)]
        slug: Option<String>,

        /// Overwrite existing generated files if present.
        #[arg(long)]
        force: bool,
    },

    /// Print a redacted copy of an authored Markdown/TOML file for sharing diagnostics.
    RedactFile {
        /// File to read and redact.
        path: PathBuf,

        /// Also redact Markdown body text. By default only structured frontmatter/config values are redacted.
        #[arg(long)]
        redact_body: bool,
    },

    /// Write a redacted copy of a world for private diagnostics sharing.
    RedactWorld {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Output directory for redacted files.
        #[arg(long)]
        out: PathBuf,

        /// Also redact Markdown body text. By default only structured frontmatter/config values are redacted.
        #[arg(long)]
        redact_body: bool,

        /// Replace existing output directory if it exists.
        #[arg(long)]
        force: bool,
    },

    /// Print a redacted authored-file tree for the selected world.
    RedactWorldTree {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,
    },

    /// Print a redacted, pasteable dump of the selected authored world.
    RedactWorldDump {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Also redact Markdown body text. By default only structured frontmatter/config values are redacted.
        #[arg(long)]
        redact_body: bool,
    },

    /// Validate world files without writing build outputs.
    Validate {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,
    },

    /// Check local media/source file references.
    CheckMedia {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// List present references as well as missing references.
        #[arg(long)]
        all: bool,

        /// Redact paths and file-reference details in output.
        #[arg(long)]
        redact: bool,
    },

    /// Check authored world records and report actionable warnings.
    Doctor {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Exit with failure when warnings are found.
        #[arg(long)]
        strict: bool,

        /// Diagnostic level to show.
        #[arg(long, value_enum, default_value_t = DoctorLevelArg::Complete)]
        level: DoctorLevelArg,

        /// Redact ids, paths, and diagnostic details in output.
        #[arg(long)]
        redact: bool,
    },

    /// Summarize authored world records and common attention items.
    Summary {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Redact names, paths, and warning details in output.
        #[arg(long)]
        redact: bool,
    },

    /// Validate and compile world files into a semantic JSON bundle.
    Compile {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Output JSON path. Defaults to <world-root>/build/kleio.compiled.json.
        #[arg(long)]
        out: Option<PathBuf>,
    },

    /// Compile world files into a minimal ECS-friendly JSON bundle.
    CompileEcs {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Output JSON path. Defaults to <world-root>/build/kleio.ecs.json.
        #[arg(long)]
        out: Option<PathBuf>,
    },

    /// Compile both semantic and ECS bundles for a world.
    Build {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Optional timeline view slug to compile during build.
        #[arg(long)]
        timeline_view: Option<String>,

        /// Optional tree view slug to compile during build.
        #[arg(long)]
        tree_view: Option<String>,
    },

    /// Compile world events into a timeline view JSON projection.
    CompileTimeline {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Timeline view slug. Defaults to the first timeline view in the world.
        #[arg(long)]
        view: Option<String>,

        /// Output JSON path. Defaults to <world-root>/build/<view-or-timeline>.timeline.json.
        #[arg(long)]
        out: Option<PathBuf>,
    },

    /// Compile world person records into the current tree view JSON projection.
    CompileTree {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Tree view slug. Defaults to the first tree view in the world.
        #[arg(long)]
        view: Option<String>,

        /// Output JSON path. Defaults to <world-root>/build/<view-or-kleio-tree>.json.
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum DoctorLevelArg {
    Complete,
    Structure,
}

impl DoctorLevelArg {
    fn includes(self, diagnostic: &kleio::LocalWorldDiagnostic) -> bool {
        match self {
            Self::Complete => true,
            Self::Structure => !matches!(
                diagnostic.kind,
                kleio::LocalWorldDiagnosticKind::EventMissingSource
                    | kleio::LocalWorldDiagnosticKind::RelationshipMissingSource
                    | kleio::LocalWorldDiagnosticKind::ReferencedFileMissing
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum SexArg {
    Female,
    Male,
    Unknown,
    Other,
}

impl SexArg {
    fn as_str(self) -> &'static str {
        match self {
            Self::Female => "female",
            Self::Male => "male",
            Self::Unknown => "unknown",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum RelativeArg {
    Parent,
    StepParent,
    Child,
    Sibling,
    Partner,
    Spouse,
}

impl RelativeArg {
    fn default_relationship_kind(self) -> &'static str {
        match self {
            Self::Parent | Self::Child => "biological-parent-child",
            Self::StepParent => "step-parent-child",
            Self::Sibling => "sibling",
            Self::Partner => "partner",
            Self::Spouse => "spouse",
        }
    }

    fn slug_suffix(self) -> &'static str {
        match self {
            Self::Parent | Self::Child => "parent-child",
            Self::StepParent => "step-parent-child",
            Self::Sibling => "sibling",
            Self::Partner => "partner",
            Self::Spouse => "spouse",
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum ParentRoleArg {
    Father,
    Mother,
    Parent,
    Unknown,
}

impl ParentRoleArg {
    fn as_str(self) -> &'static str {
        match self {
            Self::Father => "father",
            Self::Mother => "mother",
            Self::Parent => "parent",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum CollectionOrderArg {
    Chronological,
    Manual,
    ManualThenChronological,
}

impl From<CollectionOrderArg> for LocalCollectionOrder {
    fn from(value: CollectionOrderArg) -> Self {
        match value {
            CollectionOrderArg::Chronological => Self::Chronological,
            CollectionOrderArg::Manual => Self::Manual,
            CollectionOrderArg::ManualThenChronological => Self::ManualThenChronological,
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum ViewKindArg {
    Timeline,
    Tree,
    Map,
    Calendar,
    Visualization,
}

impl From<ViewKindArg> for LocalViewKind {
    fn from(value: ViewKindArg) -> Self {
        match value {
            ViewKindArg::Timeline => Self::Timeline,
            ViewKindArg::Tree => Self::Tree,
            ViewKindArg::Map => Self::Map,
            ViewKindArg::Calendar => Self::Calendar,
            ViewKindArg::Visualization => Self::Visualization,
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum ImportKindArg {
    Gedcom,
    Wikidata,
    Csv,
}

impl From<ImportKindArg> for LocalImportKind {
    fn from(value: ImportKindArg) -> Self {
        match value {
            ImportKindArg::Gedcom => Self::Gedcom,
            ImportKindArg::Wikidata => Self::Wikidata,
            ImportKindArg::Csv => Self::Csv,
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum SchemaKindArg {
    Component,
    Bundle,
    Event,
    View,
    Vocab,
}

impl From<SchemaKindArg> for LocalSchemaKind {
    fn from(value: SchemaKindArg) -> Self {
        match value {
            SchemaKindArg::Component => Self::Component,
            SchemaKindArg::Bundle => Self::Bundle,
            SchemaKindArg::Event => Self::Event,
            SchemaKindArg::View => Self::View,
            SchemaKindArg::Vocab => Self::Vocab,
        }
    }
}

fn assertion_slug_from_target_source(target: &str, source: &str) -> String {
    let target_slug = target
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let source_slug = source
        .strip_prefix("source:")
        .unwrap_or(source)
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    format!("{target_slug}-from-{source_slug}")
}

fn resolve_data_root(root: Option<PathBuf>) -> PathBuf {
    root.unwrap_or_else(default_data_root)
}

fn resolve_world_root(
    root: Option<PathBuf>,
    world: Option<&str>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let workspace_root = resolve_data_root(root);
    resolve_workspace_world_root(&workspace_root, world)
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
}

fn ensure_people_exist(
    world_root: &Path,
    people: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let person_ids = bundle
        .markdown_records
        .iter()
        .filter(|record| record.kind == "person")
        .map(|record| record.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    for person in people {
        let id = person_id(person);
        if !person_ids.contains(id.as_str()) {
            return Err(cli_error(format!(
                "person `{id}` does not exist in {}; create it first or check the slug",
                world_root.display()
            )));
        }
    }

    Ok(())
}

fn inferred_preferred_name(slug: &str) -> String {
    let slug = slug.strip_prefix("person:").unwrap_or(slug);
    let words = slug
        .split(['-', '_'])
        .filter(|part| !part.trim().is_empty())
        .map(title_case_slug_word)
        .collect::<Vec<_>>();

    match words.as_slice() {
        [] => slug.to_string(),
        [only] => only.clone(),
        [given, family] => format!("{given} {family}"),
        [given, .., family] => format!("{given} {family}"),
    }
}

fn title_case_slug_word(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().chain(chars).collect()
}

fn create_spouse_and_marriage(
    root: Option<PathBuf>,
    world: Option<&str>,
    spouse_slug: String,
    existing_person: String,
    person_name: String,
    sex: Option<String>,
    birth_date: Option<String>,
    birth_location: Option<String>,
    birth_latitude: Option<f64>,
    birth_longitude: Option<f64>,
    create_birth_event: bool,
    marriage_slug: Option<String>,
    title: Option<String>,
    places: Vec<String>,
    location: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    time: Option<String>,
    date_precision: Option<String>,
    sources: Vec<String>,
    relationship_sources: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = resolve_data_root(root);
    let world_root = resolve_workspace_world_root(&workspace_root, world)?;
    let existing_files = collect_existing_files(&world_root)?;
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        ensure_people_exist(&world_root, &[existing_person.clone()])?;
        create_local_person(
            &world_root,
            &LocalPersonOptions {
                person_slug: spouse_slug.clone(),
                person_name,
                sex,
                birth_date,
                birth_location,
                birth_latitude,
                birth_longitude,
                create_birth_event,
                force,
            },
        )?;

        create_marriage_event(
            Some(workspace_root),
            world,
            existing_person,
            spouse_slug,
            marriage_slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            relationship_sources,
            force,
        )
    })();

    if result.is_err() && !force {
        rollback_new_files(&world_root, &existing_files)?;
    }

    result
}

fn collect_existing_files(root: &Path) -> Result<BTreeSet<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = BTreeSet::new();
    collect_existing_files_inner(root, root, &mut files)?;
    Ok(files)
}

fn collect_existing_files_inner(
    root: &Path,
    path: &Path,
    files: &mut BTreeSet<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_existing_files_inner(root, &path, files)?;
        } else {
            files.insert(path.strip_prefix(root).unwrap_or(&path).to_path_buf());
        }
    }

    Ok(())
}

fn rollback_new_files(
    root: &Path,
    existing_files: &BTreeSet<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let current_files = collect_existing_files(root)?;
    for path in current_files.difference(existing_files) {
        let full_path = root.join(path);
        if let Err(source) = fs::remove_file(&full_path) {
            return Err(cli_error(format!(
                "failed to roll back partially-created file {}: {source}",
                full_path.display()
            )));
        }
    }

    Ok(())
}

fn create_marriage_event(
    root: Option<PathBuf>,
    world: Option<&str>,
    first_person: String,
    second_person: String,
    slug: Option<String>,
    title: Option<String>,
    places: Vec<String>,
    location: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    time: Option<String>,
    date_precision: Option<String>,
    sources: Vec<String>,
    relationship_sources: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_slug =
        slug.unwrap_or_else(|| relationship_slug(&first_person, &second_person, "marriage"));
    create_event_with_optional_relationship(
        root,
        world,
        event_slug,
        "marriage".to_string(),
        title,
        vec![first_person],
        Some(second_person),
        places,
        location,
        latitude,
        longitude,
        time,
        date_precision,
        sources,
        true,
        None,
        "spouse".to_string(),
        relationship_sources,
        force,
    )
}

fn create_death_event(
    root: Option<PathBuf>,
    world: Option<&str>,
    person: String,
    slug: Option<String>,
    title: Option<String>,
    places: Vec<String>,
    location: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    time: Option<String>,
    date_precision: Option<String>,
    sources: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_slug = slug.unwrap_or_else(|| format!("{}-death", person_slug_for_filename(&person)));
    create_event_with_optional_relationship(
        root,
        world,
        event_slug,
        "death".to_string(),
        title,
        vec![person],
        None,
        places,
        location,
        latitude,
        longitude,
        time,
        date_precision,
        sources,
        false,
        None,
        "associate".to_string(),
        Vec::new(),
        force,
    )
}

fn create_divorce_event(
    root: Option<PathBuf>,
    world: Option<&str>,
    first_person: String,
    second_person: String,
    slug: Option<String>,
    title: Option<String>,
    places: Vec<String>,
    location: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    time: Option<String>,
    date_precision: Option<String>,
    sources: Vec<String>,
    relationship_sources: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_slug =
        slug.unwrap_or_else(|| relationship_slug(&first_person, &second_person, "divorce"));
    create_event_with_optional_relationship(
        root,
        world,
        event_slug,
        "divorce".to_string(),
        title,
        vec![first_person],
        Some(second_person),
        places,
        location,
        latitude,
        longitude,
        time,
        date_precision,
        sources,
        true,
        None,
        "former-spouse".to_string(),
        relationship_sources,
        force,
    )
}

fn create_event_with_optional_relationship(
    root: Option<PathBuf>,
    world: Option<&str>,
    event_slug: String,
    event_type: String,
    title: Option<String>,
    mut participants: Vec<String>,
    partner: Option<String>,
    places: Vec<String>,
    location: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    time: Option<String>,
    date_precision: Option<String>,
    sources: Vec<String>,
    create_relationship: bool,
    relationship_slug_option: Option<String>,
    relationship_kind: String,
    relationship_sources: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(partner) = partner.clone() {
        participants.push(partner);
    }
    if participants.is_empty() {
        return Err(cli_error(
            "add-event requires at least one --person or --partner",
        ));
    }

    let world_root = resolve_world_root(root, world)?;
    if !participants.is_empty() {
        let mut referenced_people = participants.clone();
        if let Some(partner) = &partner {
            referenced_people.push(partner.clone());
        }
        ensure_people_exist(&world_root, &referenced_people)?;
    }
    let event_subject = (participants.len() == 1).then(|| participants[0].clone());
    let event_participants = if event_subject.is_some() {
        Vec::new()
    } else {
        participants.clone()
    };
    let event_path = create_local_event(
        &world_root,
        &LocalEventOptions {
            event_slug,
            event_type: event_type.clone(),
            title,
            subject: event_subject,
            participants: event_participants,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            force,
        },
    )?;
    println!("created event record at {}", event_path.display());

    let should_create_relationship = create_relationship
        || (event_type == "marriage" && partner.is_some())
        || (event_type == "divorce" && partner.is_some());
    if should_create_relationship {
        let Some(first_person) = participants.first() else {
            return Err(cli_error(
                "add-event --create-relationship requires --person",
            ));
        };
        let Some(second_person) = participants.get(1) else {
            return Err(cli_error(
                "add-event --create-relationship requires --partner or a second --person",
            ));
        };
        let first_person_id = person_id(first_person);
        let second_person_id = person_id(second_person);
        let relationship_slug = relationship_slug_option.unwrap_or_else(|| {
            let suffix = if event_type == "marriage" && relationship_kind == "spouse" {
                "spouse"
            } else if event_type == "divorce" && relationship_kind == "former-spouse" {
                "former-spouse"
            } else {
                "relationship"
            };
            relationship_slug(first_person, second_person, suffix)
        });
        let relationship_path = create_local_relationship(
            &world_root,
            &LocalRelationshipOptions {
                relationship_slug,
                title: None,
                relationship_kind,
                parent_role: None,
                source: first_person_id,
                target: second_person_id,
                sources: relationship_sources,
                force,
            },
        )?;
        println!(
            "created related relationship at {}",
            relationship_path.display()
        );
    }

    Ok(())
}

fn person_id(value: &str) -> String {
    if value.contains(':') {
        value.to_string()
    } else {
        format!("person:{value}")
    }
}

fn relationship_slug(source_slug: &str, target: &str, suffix: &str) -> String {
    let source_slug = person_slug_for_filename(source_slug);
    let target_slug = person_slug_for_filename(target);
    format!("{source_slug}-{target_slug}-{suffix}")
}

fn person_slug_for_filename(value: &str) -> String {
    value
        .strip_prefix("person:")
        .unwrap_or(value)
        .replace(':', "-")
}

fn relative_relationship_endpoints(
    relation: RelativeArg,
    existing_person: &str,
    relative_slug: &str,
) -> (String, String) {
    let existing_person_id = person_id(existing_person);
    let relative_id = person_id(relative_slug);
    match relation {
        RelativeArg::Parent | RelativeArg::StepParent => (relative_id, existing_person_id),
        RelativeArg::Child => (existing_person_id, relative_id),
        RelativeArg::Sibling | RelativeArg::Partner | RelativeArg::Spouse => {
            (existing_person_id, relative_id)
        }
    }
}

fn create_family_person_relationship(
    root: Option<PathBuf>,
    world: Option<&str>,
    new_person_slug: String,
    new_person_name: String,
    birth_date: Option<String>,
    birth_location: Option<String>,
    birth_latitude: Option<f64>,
    birth_longitude: Option<f64>,
    sex: Option<String>,
    create_birth_event: bool,
    relationship_slug: String,
    relationship_title: Option<String>,
    relationship_kind: String,
    parent_role: Option<String>,
    source: String,
    target: String,
    existing_people: Vec<String>,
    source_records: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let world_root = resolve_world_root(root, world)?;
    ensure_people_exist(&world_root, &existing_people)?;
    create_local_person(
        &world_root,
        &LocalPersonOptions {
            person_slug: new_person_slug,
            person_name: new_person_name,
            sex,
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            create_birth_event,
            force,
        },
    )?;
    let relationship_path = create_local_relationship(
        &world_root,
        &LocalRelationshipOptions {
            relationship_slug,
            title: relationship_title,
            relationship_kind,
            parent_role,
            source,
            target,
            sources: source_records,
            force,
        },
    )?;
    println!(
        "created person and relationship under {}; relationship at {}",
        world_root.display(),
        relationship_path.display()
    );
    Ok(())
}

fn create_entity_record(
    root: Option<PathBuf>,
    world: Option<&str>,
    slug: String,
    title: String,
    kind: LocalEntityKind,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let world_root = resolve_world_root(root, world)?;
    let path = create_local_entity(
        &world_root,
        &LocalEntityOptions {
            slug,
            title,
            kind,
            force,
        },
    )?;
    println!("created {} record at {}", kind.as_str(), path.display());
    Ok(())
}

fn create_view_record(
    root: Option<PathBuf>,
    world: Option<&str>,
    slug: String,
    title: String,
    kind: LocalViewKind,
    subject: Option<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let world_root = resolve_world_root(root, world)?;
    let path = create_local_view(
        &world_root,
        &LocalViewOptions {
            view_slug: slug,
            title,
            kind,
            subject,
            force,
        },
    )?;
    println!("created {} at {}", kind.kind_value(), path.display());
    Ok(())
}

fn person_has_birth_event(bundle: &kleio::LocalDataBundle, person_id: &str) -> bool {
    bundle.markdown_records.iter().any(|record| {
        record.kind == "event"
            && record
                .attributes
                .get("type")
                .and_then(|value| value.as_str())
                == Some("birth")
            && event_mentions_person(record, person_id)
    })
}

fn event_mentions_person(record: &kleio::LocalMarkdownRecord, person_id: &str) -> bool {
    record
        .attributes
        .get("subject")
        .and_then(|value| value.as_str())
        .is_some_and(|subject| person_id_matches(subject, person_id))
        || record
            .attributes
            .get("participants")
            .and_then(|value| value.as_array())
            .is_some_and(|participants| {
                participants.iter().any(|participant| {
                    participant
                        .as_str()
                        .is_some_and(|value| person_id_matches(value, person_id))
                        || participant
                            .as_object()
                            .and_then(|values| values.get("entity"))
                            .and_then(|value| value.as_str())
                            .is_some_and(|value| person_id_matches(value, person_id))
                })
            })
}

fn person_id_matches(value: &str, person_id: &str) -> bool {
    value == person_id || person_id.strip_prefix("person:") == Some(value)
}

fn relationship_kind(document: &kleio::LocalTomlDocument) -> Option<&str> {
    document
        .data
        .get("relationship")
        .and_then(|value| value.as_str())
        .or_else(|| {
            document
                .data
                .get("relationship_kind")
                .and_then(|value| value.as_str())
        })
        .or_else(|| {
            document
                .data
                .get("relation")
                .and_then(|value| value.as_str())
        })
}

fn person_parent_count(bundle: &kleio::LocalDataBundle, person_id: &str) -> usize {
    bundle
        .toml_documents
        .iter()
        .filter(|document| document.kind.as_deref() == Some("relationship"))
        .filter(|document| {
            relationship_kind(document).is_some_and(|kind| {
                matches!(
                    kind,
                    "biological-parent-child"
                        | "adoptive-parent-child"
                        | "foster-parent-child"
                        | "step-parent-child"
                        | "guardian-child"
                )
            })
        })
        .filter(|document| {
            document
                .data
                .get("target")
                .and_then(|value| value.as_str())
                .is_some_and(|target| person_id_matches(target, person_id))
        })
        .count()
}

fn person_has_partner(bundle: &kleio::LocalDataBundle, person_id: &str) -> bool {
    bundle
        .toml_documents
        .iter()
        .filter(|document| document.kind.as_deref() == Some("relationship"))
        .any(|document| {
            let partner_kind = relationship_kind(document)
                .is_some_and(|kind| matches!(kind, "partner" | "spouse" | "former-spouse"));
            if !partner_kind {
                return false;
            }
            let source_matches = document
                .data
                .get("source")
                .and_then(|value| value.as_str())
                .is_some_and(|source| person_id_matches(source, person_id));
            let target_matches = document
                .data
                .get("target")
                .and_then(|value| value.as_str())
                .is_some_and(|target| person_id_matches(target, person_id));
            source_matches || target_matches
        })
}

fn guide_subject<'a>(
    bundle: &'a kleio::LocalDataBundle,
    person: Option<&str>,
) -> Option<&'a kleio::LocalMarkdownRecord> {
    if let Some(person) = person {
        let id = person_id(person);
        return bundle
            .markdown_records
            .iter()
            .find(|record| record.kind == "person" && record.id == id);
    }

    let tree_root = bundle
        .toml_documents
        .iter()
        .find(|document| document.kind.as_deref() == Some("tree-view"))
        .and_then(|document| document.data.get("root"))
        .and_then(|root| root.get("entity"))
        .and_then(|value| value.as_str())
        .map(person_id);

    tree_root
        .as_deref()
        .and_then(|id| {
            bundle
                .markdown_records
                .iter()
                .find(|record| record.kind == "person" && record.id == id)
        })
        .or_else(|| {
            bundle
                .markdown_records
                .iter()
                .find(|record| record.kind == "person")
        })
}

fn print_authoring_guide(
    world_root: &Path,
    person: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let Some(subject) = guide_subject(&bundle, person) else {
        println!("No people found yet.");
        println!(
            "Start with `kleio-cli new-person <slug> --person-name \"Name\"` or `kleio-cli init-workspace --person-slug <slug> --person-name \"Name\"`."
        );
        return Ok(());
    };

    let subject_slug = subject.id.strip_prefix("person:").unwrap_or(&subject.id);
    println!("Family tree guide for {}", subject.id);
    println!("World: {}", world_root.display());
    println!();

    if !person_has_birth_event(&bundle, &subject.id) {
        println!("1. Add birth data for this person:");
        println!(
            "   kleio-cli add-event birth-{subject_slug} --type birth --person {subject_slug} --time YYYY-MM-DD --location \"Birthplace\""
        );
        println!();
    }

    let parent_count = person_parent_count(&bundle, &subject.id);
    if parent_count < 2 {
        println!("2. Add parent records:");
        println!(
            "   kleio-cli add-relative mother-slug --relation parent --of {subject_slug} --person-name \"Mother Name\""
        );
        println!(
            "   kleio-cli add-relative father-slug --relation parent --of {subject_slug} --person-name \"Father Name\""
        );
        println!(
            "   Tip: add grandparents by changing --of to a parent slug, for example `--of mother-slug`."
        );
        println!();
    }

    if !person_has_partner(&bundle, &subject.id) {
        println!("3. Optionally add a partner or spouse:");
        println!(
            "   kleio-cli add-relative partner-slug --relation partner --of {subject_slug} --person-name \"Partner Name\" --no-birth-event"
        );
        println!();
    }

    println!("4. Add other life events when ready:");
    println!(
        "   kleio-cli add-event {subject_slug}-residence --type residence --person {subject_slug} --time YYYY --location \"Place\""
    );
    println!(
        "   kleio-cli add-event {subject_slug}-moment --type moment --person {subject_slug} --title \"Short label\" --time YYYY-MM-DD"
    );
    println!();
    println!("5. Check progress:");
    println!("   kleio-cli summary --redact");
    println!("   kleio-cli doctor --redact");
    Ok(())
}

fn local_record_name(record: &kleio::LocalMarkdownRecord) -> String {
    record
        .title
        .clone()
        .or_else(|| local_name_table(record, "preferred"))
        .or_else(|| local_name_table(record, "legal"))
        .unwrap_or_else(|| "-".to_string())
}

fn local_name_table(record: &kleio::LocalMarkdownRecord, usage: &str) -> Option<String> {
    let table = record.attributes.get("names")?.get(usage)?;
    table
        .get("display")
        .and_then(|value| value.as_str())
        .or_else(|| table.get("full").and_then(|value| value.as_str()))
        .map(ToOwned::to_owned)
        .or_else(|| {
            let given = table.get("given").and_then(|given| given.as_str())?;
            let family = table.get("family").and_then(|family| family.as_str())?;
            Some(format!("{given} {family}"))
        })
}

fn local_record_type(record: &kleio::LocalMarkdownRecord) -> String {
    record
        .attributes
        .get("type")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| record.kind.clone())
}

fn local_record_matches_filter(
    record: &kleio::LocalMarkdownRecord,
    fields: &[&str],
    filter: Option<&str>,
) -> bool {
    let Some(filter) = filter else {
        return true;
    };
    let filter = filter.to_lowercase();
    let haystack = std::iter::once(record.id.as_str())
        .chain(std::iter::once(record.path.as_str()))
        .chain(fields.iter().copied())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    haystack.contains(&filter)
}

fn redacted_id(prefix: &str, index: usize) -> String {
    format!("{prefix}:<redacted-{index}>")
}

fn print_people(
    world_root: &Path,
    filter: Option<&str>,
    redact: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let mut index = 1;
    for record in bundle
        .markdown_records
        .iter()
        .filter(|record| record.kind == "person")
    {
        let name = local_record_name(record);
        if !local_record_matches_filter(record, &[&name], filter) {
            continue;
        }
        if redact {
            println!("{}	[redacted]	[redacted]", redacted_id("person", index));
            index += 1;
        } else {
            println!("{}	{}	{}", record.id, name, record.path);
        }
    }
    Ok(())
}

fn print_events(
    world_root: &Path,
    filter: Option<&str>,
    redact: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let mut index = 1;
    for record in bundle
        .markdown_records
        .iter()
        .filter(|record| record.kind == "event")
    {
        let title = local_record_name(record);
        let event_type = local_record_type(record);
        if !local_record_matches_filter(record, &[&title, &event_type], filter) {
            continue;
        }
        if redact {
            println!(
                "{}	{}	[redacted]	[redacted]",
                redacted_id("event", index),
                event_type
            );
            index += 1;
        } else {
            println!("{}	{}	{}	{}", record.id, event_type, title, record.path);
        }
    }
    Ok(())
}

fn print_sources(
    world_root: &Path,
    filter: Option<&str>,
    redact: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let mut index = 1;
    for record in bundle
        .markdown_records
        .iter()
        .filter(|record| record.path.starts_with("sources/"))
    {
        let title = local_record_name(record);
        if !local_record_matches_filter(record, &[&title, &record.kind], filter) {
            continue;
        }
        if redact {
            println!(
                "{}	{}	[redacted]	[redacted]",
                redacted_id("source", index),
                record.kind
            );
            index += 1;
        } else {
            println!("{}	{}	{}	{}", record.id, record.kind, title, record.path);
        }
    }
    Ok(())
}

fn print_tree_sketch(
    world_root: &Path,
    person: Option<&str>,
    depth: usize,
    redact: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let tree = compile_local_tree(world_root)?;
    let derived_kinship = compile_local_ecs(world_root)?
        .resources
        .relationships
        .derived_kinship;
    if tree.people.is_empty() {
        println!("Tree sketch: no people found.");
        return Ok(());
    }

    let focus = if let Some(person) = person {
        resolve_tree_person_id(&tree, person)
    } else {
        configured_tree_sketch_person(world_root)?
            .as_deref()
            .and_then(|person| resolve_tree_person_id(&tree, person))
    }
    .ok_or_else(|| {
        cli_error(match person {
            Some(person) => format!("person `{person}` was not found in the compiled tree"),
            None => "tree-sketch needs a focus person; pass `--person <person-slug>` or set `[root].entity` in a tree view / `[tree].main_person` in the world config".to_string(),
        })
    })?;
    let depth = depth.min(5);
    let redacted_names = redacted_person_names(&tree);

    println!(
        "Tree sketch for {}",
        tree_sketch_person_label(&tree, focus, redact, &redacted_names)
    );
    println!(
        "Focus: {}",
        tree_sketch_person_label(&tree, focus, redact, &redacted_names)
    );

    let mut parent_lines = ancestor_lines(&tree, focus, depth, redact, &redacted_names);
    parent_lines.extend(derived_step_parent_lines(
        &tree,
        &derived_kinship,
        focus,
        redact,
        &redacted_names,
    ));
    print_tree_group("Parents / ancestors", parent_lines);
    print_tree_group(
        "Siblings",
        sibling_lines(&tree, focus, redact, &redacted_names),
    );
    print_tree_group(
        "Spouses / partners",
        partner_lines(&tree, focus, redact, &redacted_names),
    );
    let mut child_lines = descendant_lines(&tree, focus, depth, redact, &redacted_names);
    child_lines.extend(derived_step_child_lines(
        &tree,
        &derived_kinship,
        focus,
        redact,
        &redacted_names,
    ));
    print_tree_group("Children / descendants", child_lines);

    Ok(())
}

fn resolve_tree_person_id(tree: &kleio::TreeDocument, person: &str) -> Option<kleio::PersonId> {
    let source_ids = tree
        .people
        .iter()
        .filter_map(|candidate| {
            let source_id = candidate.source_record.as_ref()?.0.strip_prefix("local:")?;
            Some((source_id.to_string(), candidate.id))
        })
        .collect::<BTreeMap<_, _>>();

    source_ids
        .get(person)
        .copied()
        .or_else(|| source_ids.get(&person_id(person)).copied())
}

fn configured_tree_sketch_person(
    world_root: &Path,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let tree_view_root = bundle
        .toml_documents
        .iter()
        .filter(|document| document.kind.as_deref() == Some("tree-view"))
        .find_map(|document| {
            document
                .data
                .get("root")?
                .get("entity")?
                .as_str()
                .map(ToOwned::to_owned)
        });
    if tree_view_root.is_some() {
        return Ok(tree_view_root);
    }

    Ok(bundle
        .toml_documents
        .iter()
        .find(|document| {
            document.kind.as_deref() == Some("registry") || document.path == "world.toml"
        })
        .and_then(|document| document.data.get("tree"))
        .and_then(|tree| tree.get("main_person"))
        .and_then(|main_person| main_person.as_str())
        .map(ToOwned::to_owned))
}

fn redacted_person_names(tree: &kleio::TreeDocument) -> BTreeMap<kleio::PersonId, String> {
    tree.people
        .iter()
        .enumerate()
        .map(|(index, person)| (person.id, redacted_id("person", index + 1)))
        .collect()
}

fn tree_sketch_person_label(
    tree: &kleio::TreeDocument,
    person: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> String {
    if redact {
        redacted_names
            .get(&person)
            .cloned()
            .unwrap_or_else(|| "person:<redacted>".to_string())
    } else {
        tree.person_display_name(person)
            .map(str::to_string)
            .unwrap_or_else(|| format!("Person {}", person.0))
    }
}

fn relationship_line_label(relationship: &kleio::TreeRelationship) -> String {
    relationship.label.clone().unwrap_or_else(|| {
        relationship
            .kind
            .label_with_parent_role(relationship.parent_role)
    })
}

fn ancestor_lines(
    tree: &kleio::TreeDocument,
    focus: kleio::PersonId,
    depth: usize,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut visited = BTreeSet::from([focus]);
    collect_ancestor_lines(
        tree,
        focus,
        depth,
        1,
        redact,
        redacted_names,
        &mut visited,
        &mut lines,
    );
    lines
}

fn collect_ancestor_lines(
    tree: &kleio::TreeDocument,
    person: kleio::PersonId,
    remaining_depth: usize,
    level: usize,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
    visited: &mut BTreeSet<kleio::PersonId>,
    lines: &mut Vec<String>,
) {
    if remaining_depth == 0 {
        return;
    }

    for relationship in tree
        .relationships
        .iter()
        .filter(|relationship| relationship.kind.is_parent_child() && relationship.target == person)
    {
        let parent = relationship.source;
        let indent = "  ".repeat(level.saturating_sub(1));
        let label = tree_sketch_person_label(tree, parent, redact, redacted_names);
        lines.push(format!(
            "{indent}- {label} ({})",
            relationship_line_label(relationship)
        ));
        if visited.insert(parent) {
            collect_ancestor_lines(
                tree,
                parent,
                remaining_depth - 1,
                level + 1,
                redact,
                redacted_names,
                visited,
                lines,
            );
        }
    }
}

fn descendant_lines(
    tree: &kleio::TreeDocument,
    focus: kleio::PersonId,
    depth: usize,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut visited = BTreeSet::from([focus]);
    collect_descendant_lines(
        tree,
        focus,
        depth,
        1,
        redact,
        redacted_names,
        &mut visited,
        &mut lines,
    );
    lines
}

fn collect_descendant_lines(
    tree: &kleio::TreeDocument,
    person: kleio::PersonId,
    remaining_depth: usize,
    level: usize,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
    visited: &mut BTreeSet<kleio::PersonId>,
    lines: &mut Vec<String>,
) {
    if remaining_depth == 0 {
        return;
    }

    for relationship in tree
        .relationships
        .iter()
        .filter(|relationship| relationship.kind.is_parent_child() && relationship.source == person)
    {
        let child = relationship.target;
        let indent = "  ".repeat(level.saturating_sub(1));
        let label = tree_sketch_person_label(tree, child, redact, redacted_names);
        lines.push(format!(
            "{indent}- {label} ({})",
            relationship_line_label(relationship)
        ));
        if visited.insert(child) {
            collect_descendant_lines(
                tree,
                child,
                remaining_depth - 1,
                level + 1,
                redact,
                redacted_names,
                visited,
                lines,
            );
        }
    }
}

fn sibling_lines(
    tree: &kleio::TreeDocument,
    focus: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let parents = tree
        .relationships
        .iter()
        .filter(|relationship| relationship.kind.is_parent_child() && relationship.target == focus)
        .map(|relationship| relationship.source)
        .collect::<BTreeSet<_>>();
    let mut siblings = BTreeSet::new();
    for parent in parents {
        siblings.extend(
            tree.relationships
                .iter()
                .filter(|relationship| {
                    relationship.kind.is_parent_child()
                        && relationship.source == parent
                        && relationship.target != focus
                })
                .map(|relationship| relationship.target),
        );
    }

    for relationship in tree
        .relationships
        .iter()
        .filter(|relationship| relationship.kind == kleio::RelationshipKind::Sibling)
    {
        if relationship.source == focus {
            siblings.insert(relationship.target);
        } else if relationship.target == focus {
            siblings.insert(relationship.source);
        }
    }

    siblings
        .into_iter()
        .map(|sibling| {
            format!(
                "- {}",
                tree_sketch_person_label(tree, sibling, redact, redacted_names)
            )
        })
        .collect()
}

fn partner_lines(
    tree: &kleio::TreeDocument,
    focus: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    tree.relationships
        .iter()
        .filter_map(|relationship| {
            let kind = relationship.kind.as_value();
            if !matches!(kind, "spouse" | "partner" | "former-spouse") {
                return None;
            }
            let other = if relationship.source == focus {
                relationship.target
            } else if relationship.target == focus {
                relationship.source
            } else {
                return None;
            };
            Some(format!(
                "- {} ({})",
                tree_sketch_person_label(tree, other, redact, redacted_names),
                relationship_line_label(relationship)
            ))
        })
        .collect()
}

fn derived_step_parent_lines(
    tree: &kleio::TreeDocument,
    derived_kinship: &[kleio::LocalDerivedKinshipRelationship],
    focus: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let Some(focus_source_id) = tree_source_id(tree, focus) else {
        return Vec::new();
    };

    derived_kinship
        .iter()
        .filter(|relationship| {
            relationship.relationship_kind == "step-parent-child"
                && relationship.target == focus_source_id
        })
        .filter_map(|relationship| {
            let source = tree_person_by_source_id(tree, &relationship.source)?;
            Some(format!(
                "- {} ({})",
                tree_sketch_person_label(tree, source, redact, redacted_names),
                derived_step_parent_label(relationship)
            ))
        })
        .collect()
}

fn derived_step_child_lines(
    tree: &kleio::TreeDocument,
    derived_kinship: &[kleio::LocalDerivedKinshipRelationship],
    focus: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let Some(focus_source_id) = tree_source_id(tree, focus) else {
        return Vec::new();
    };

    derived_kinship
        .iter()
        .filter(|relationship| {
            relationship.relationship_kind == "step-parent-child"
                && relationship.source == focus_source_id
        })
        .filter_map(|relationship| {
            let target = tree_person_by_source_id(tree, &relationship.target)?;
            Some(format!(
                "- {} ({})",
                tree_sketch_person_label(tree, target, redact, redacted_names),
                derived_step_child_label(relationship)
            ))
        })
        .collect()
}

fn derived_step_parent_label(relationship: &kleio::LocalDerivedKinshipRelationship) -> String {
    derived_period_label("inferred step-parent", relationship)
}

fn derived_step_child_label(relationship: &kleio::LocalDerivedKinshipRelationship) -> String {
    derived_period_label("inferred step-child", relationship)
}

fn derived_period_label(
    prefix: &str,
    relationship: &kleio::LocalDerivedKinshipRelationship,
) -> String {
    match (&relationship.valid_from, &relationship.valid_until) {
        (Some(from), Some(until)) => format!("{prefix}, {from} to {until}"),
        (Some(from), None) => format!("{prefix}, since {from}"),
        (None, Some(until)) => format!("{prefix}, until {until}"),
        (None, None) => prefix.to_string(),
    }
}

fn tree_source_id(tree: &kleio::TreeDocument, person: kleio::PersonId) -> Option<String> {
    tree.people
        .iter()
        .find(|candidate| candidate.id == person)?
        .source_record
        .as_ref()?
        .0
        .strip_prefix("local:")
        .map(ToOwned::to_owned)
}

fn tree_person_by_source_id(
    tree: &kleio::TreeDocument,
    source_id: &str,
) -> Option<kleio::PersonId> {
    tree.people.iter().find_map(|candidate| {
        let candidate_source_id = candidate.source_record.as_ref()?.0.strip_prefix("local:")?;
        (candidate_source_id == source_id).then_some(candidate.id)
    })
}

fn print_tree_group(label: &str, lines: Vec<String>) {
    println!("\n{label}:");
    if lines.is_empty() {
        println!("- none found");
    } else {
        for line in lines {
            println!("{line}");
        }
    }
}

fn print_workspace_next_steps(root: &Path, world_slug: &str, person_slug: &str) {
    println!(
        "next: grow the tree with `kleio-cli add-relative <slug> --relation parent|child|sibling|partner|spouse --of {person_slug} --person-name \"Name\" --root {}`",
        root.display()
    );
    println!(
        "add life events with `kleio-cli add-event <slug> --type residence|marriage|death|moment --person {person_slug} --root {}`",
        root.display()
    );
    println!(
        "then check your work with `kleio-cli summary --world {world_slug}` and `kleio-cli doctor --world {world_slug}`"
    );
    println!("build outputs with `kleio-cli build --world {world_slug}`");
}

fn print_media_check_report(
    world_root: &Path,
    report: &kleio::LocalMediaCheckReport,
    show_all: bool,
    redact: bool,
) {
    if redact {
        println!("Media/source file references for [redacted]");
    } else {
        println!("Media/source file references for {}", world_root.display());
    }
    println!("Referenced files: {}", report.referenced_files());
    println!("Present: {}", report.present_files());
    println!("Missing: {}", report.missing_files());

    if redact {
        println!(
            "\nDetails redacted. Run without --redact locally to inspect file paths and references."
        );
        return;
    }

    let references = report
        .references
        .iter()
        .filter(|reference| show_all || !reference.exists)
        .collect::<Vec<_>>();
    if references.is_empty() {
        if show_all {
            println!("\nReferences: none found");
        } else {
            println!("\nMissing: none found");
        }
    } else {
        println!("\n{}:", if show_all { "References" } else { "Missing" });
        for reference in references {
            let status = if reference.exists {
                "present"
            } else {
                "missing"
            };
            if show_all {
                println!("- {} [{status}]", reference.path);
            } else {
                println!("- {}", reference.path);
            }
            for source in &reference.referenced_by {
                println!("  referenced by {source}");
            }
        }
    }
}

#[derive(Debug, Default)]
struct FileRedactor {
    tokens: BTreeMap<String, String>,
    next_person: usize,
    next_event: usize,
    next_source: usize,
    next_place: usize,
    next_relationship: usize,
    next_assertion: usize,
    next_other: usize,
}

impl FileRedactor {
    fn redact_text(&mut self, text: &str, redact_body: bool) -> String {
        let mut output = String::new();
        let mut in_frontmatter = false;
        let mut frontmatter_closed = false;
        let mut body_redacted = false;
        for line in text.lines() {
            if line.trim() == "+++" && !frontmatter_closed {
                in_frontmatter = !in_frontmatter;
                if !in_frontmatter {
                    frontmatter_closed = true;
                }
                output.push_str(line);
                output.push('\n');
                continue;
            }

            if redact_body && frontmatter_closed {
                if !body_redacted && !line.trim().is_empty() {
                    output.push('\n');
                    output.push_str("[body redacted]");
                    output.push('\n');
                    body_redacted = true;
                }
                continue;
            }

            if in_frontmatter || !frontmatter_closed {
                output.push_str(&self.redact_line(line));
            } else {
                output.push_str(line);
            }
            output.push('\n');
        }
        output
    }

    fn redact_line(&mut self, line: &str) -> String {
        if let Some(redacted) = self.redact_unquoted_sensitive_assignment(line) {
            return redacted;
        }

        let mut output = String::new();
        let mut chars = line.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '"' {
                let mut value = String::new();
                let mut escaped = false;
                for inner in chars.by_ref() {
                    if escaped {
                        value.push(inner);
                        escaped = false;
                    } else if inner == '\\' {
                        escaped = true;
                    } else if inner == '"' {
                        break;
                    } else {
                        value.push(inner);
                    }
                }
                output.push('"');
                output.push_str(&self.redact_value(&value));
                output.push('"');
            } else {
                output.push(ch);
            }
        }
        output
    }

    fn redact_unquoted_sensitive_assignment(&self, line: &str) -> Option<String> {
        let trimmed_start = line.trim_start();
        let indent_len = line.len() - trimmed_start.len();
        let (key, _) = trimmed_start.split_once('=')?;
        let key = key.trim();
        if !matches!(
            key,
            "latitude" | "longitude" | "lat" | "lng" | "birth_date" | "death_date" | "time"
        ) {
            return None;
        }
        let indent = &line[..indent_len];
        Some(format!("{indent}{key} = \"<date-or-number>\""))
    }

    fn redact_value(&mut self, value: &str) -> String {
        if value.trim().is_empty() || value.starts_with('#') {
            return value.to_string();
        }
        if matches!(
            value,
            "person"
                | "event"
                | "relationship"
                | "source"
                | "place"
                | "assertion"
                | "birth"
                | "death"
                | "residence"
                | "marriage"
                | "migration"
                | "immigration"
                | "emigration"
                | "observation"
                | "moment"
                | "event-support"
                | "medium"
                | "high"
                | "low"
                | "subject"
                | "parent"
                | "partner"
                | "spouse"
                | "sibling"
        ) {
            return value.to_string();
        }
        if looks_like_date_or_time(value) || value.parse::<f64>().is_ok() {
            return "<date-or-number>".to_string();
        }
        if let Some((prefix, _)) = value.split_once(':')
            && matches!(
                prefix,
                "person" | "event" | "source" | "place" | "relationship" | "assertion"
            )
        {
            return self.token_for(prefix, value);
        }
        if value.contains('/') || value.contains('.') {
            return self.token_for("path", value);
        }
        self.token_for("text", value)
    }

    fn token_for(&mut self, prefix: &str, value: &str) -> String {
        if let Some(token) = self.tokens.get(value) {
            return token.clone();
        }
        let (label, index) = match prefix {
            "person" => {
                self.next_person += 1;
                ("person", self.next_person)
            }
            "event" => {
                self.next_event += 1;
                ("event", self.next_event)
            }
            "source" => {
                self.next_source += 1;
                ("source", self.next_source)
            }
            "place" => {
                self.next_place += 1;
                ("place", self.next_place)
            }
            "relationship" => {
                self.next_relationship += 1;
                ("relationship", self.next_relationship)
            }
            "assertion" => {
                self.next_assertion += 1;
                ("assertion", self.next_assertion)
            }
            _ => {
                self.next_other += 1;
                ("redacted", self.next_other)
            }
        };
        let token = format!("{label}:<redacted-{index}>");
        self.tokens.insert(value.to_string(), token.clone());
        token
    }
}

fn looks_like_date_or_time(value: &str) -> bool {
    let value = value.trim();
    value.len() >= 4 && value.chars().take(4).all(|ch| ch.is_ascii_digit())
}

fn redact_file(path: &Path, redact_body: bool) -> Result<String, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    Ok(FileRedactor::default().redact_text(&text, redact_body))
}

fn redact_world_dump(
    world_root: &Path,
    redact_body: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut redactor = FileRedactor::default();
    let mut path_redactor = FileRedactor::default();
    let mut files = Vec::new();
    collect_redactable_world_files(world_root, world_root, &mut files)?;
    files.sort();

    let mut output = String::new();
    output.push_str("===== REDACTED KLEIO WORLD DUMP =====\n");
    output.push_str("Generated by `kleio-cli redact-world-dump`.\n");
    output.push_str("Skipped: build/, imports/, media/, hidden files/directories.\n");
    output.push_str(&format!("Markdown body text redacted: {redact_body}\n"));

    for relative_path in &files {
        let redacted_path = redact_relative_path(relative_path, &mut path_redactor);
        let source_path = world_root.join(relative_path);
        let text = std::fs::read_to_string(&source_path)?;
        output.push_str("\n===== ");
        output.push_str(&redacted_path.display().to_string());
        output.push_str(" =====\n");
        output.push_str(&redactor.redact_text(&text, redact_body));
    }

    Ok(output)
}

fn redact_world_tree(world_root: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    collect_redactable_world_files(world_root, world_root, &mut files)?;
    files.sort();
    let mut redactor = FileRedactor::default();
    Ok(files
        .iter()
        .map(|path| redact_relative_path(path, &mut redactor))
        .collect())
}

fn redact_world(
    world_root: &Path,
    out: &Path,
    redact_body: bool,
    force: bool,
) -> Result<usize, Box<dyn std::error::Error>> {
    if out.exists() {
        if !force {
            return Err(cli_error(format!(
                "output directory {} already exists; pass --force to replace it",
                out.display()
            )));
        }
        std::fs::remove_dir_all(out)?;
    }
    std::fs::create_dir_all(out)?;

    let mut redactor = FileRedactor::default();
    let mut files = Vec::new();
    collect_redactable_world_files(world_root, world_root, &mut files)?;
    files.sort();

    let mut path_redactor = FileRedactor::default();
    let redacted_paths = files
        .iter()
        .map(|relative_path| {
            let redacted_path = redact_relative_path(relative_path, &mut path_redactor);
            (relative_path.clone(), redacted_path)
        })
        .collect::<Vec<_>>();

    for (relative_path, redacted_path) in &redacted_paths {
        let source_path = world_root.join(relative_path);
        let output_path = out.join(redacted_path);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = std::fs::read_to_string(&source_path)?;
        std::fs::write(output_path, redactor.redact_text(&text, redact_body))?;
    }

    let readme_path = out.join("README.md");
    std::fs::write(readme_path, redacted_world_readme(files.len(), redact_body))?;

    Ok(files.len())
}

fn redacted_world_readme(file_count: usize, redact_body: bool) -> String {
    format!(
        r#"# Redacted Kleio world

This directory was generated by `kleio-cli redact-world`.

It contains redacted authored Markdown/TOML files for diagnostics and structural
review. References are redacted consistently within this output so the shape of
people, events, sources, and relationships can still be inspected.

## Included

- Authored `.md` and `.toml` files from the selected world.
- Directory categories such as `entities/`, `events/`, `relationships/`,
  `sources/`, `views/`, and `schemas/`.

## Skipped

- `build/` generated outputs.
- `imports/` raw import artifacts.
- `media/` files and blobs.
- Hidden files and directories.

## Redaction

- Redacted authored files: {file_count}
- Markdown body text redacted: {redact_body}
- `README.md` is added by the redaction command and is not counted above.
- Quoted TOML/frontmatter values are replaced with placeholders.
- Date-like and numeric string values are replaced with `<date-or-number>`.
- Output file names are redacted while directory categories are preserved.

## Review before sharing

Inspect this directory before sharing. Redaction is best-effort and intended for
diagnostics, not as a guarantee that the output is safe for public release.
"#
    )
}

fn redact_relative_path(path: &Path, redactor: &mut FileRedactor) -> PathBuf {
    let mut redacted = PathBuf::new();
    for component in path.components() {
        let value = component.as_os_str().to_string_lossy();
        if value.ends_with(".md") || value.ends_with(".toml") {
            let extension = Path::new(value.as_ref())
                .extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or("txt");
            let stem = Path::new(value.as_ref())
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("file");
            let token = redactor.token_for("path", stem);
            let safe_stem = token
                .chars()
                .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
                .collect::<String>()
                .trim_matches('-')
                .to_string();
            redacted.push(format!("{}.{}", safe_stem, extension));
        } else {
            redacted.push(value.as_ref());
        }
    }
    redacted
}

fn collect_redactable_world_files(
    world_root: &Path,
    dir: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if file_name.starts_with('.')
            || (file_type.is_dir() && matches!(file_name.as_ref(), "build" | "imports" | "media"))
        {
            continue;
        }

        if file_type.is_dir() {
            collect_redactable_world_files(world_root, &path, files)?;
        } else if file_type.is_file()
            && matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("md" | "toml")
            )
        {
            files.push(path.strip_prefix(world_root).unwrap_or(&path).to_path_buf());
        }
    }
    Ok(())
}

fn diagnostic_kind_label(kind: kleio::LocalWorldDiagnosticKind) -> &'static str {
    match kind {
        kleio::LocalWorldDiagnosticKind::PersonMissingName => "person-missing-name",
        kleio::LocalWorldDiagnosticKind::PersonMissingBirthEvent => "person-missing-birth-event",
        kleio::LocalWorldDiagnosticKind::EventMissingParticipant => "event-missing-participant",
        kleio::LocalWorldDiagnosticKind::EventMissingTime => "event-missing-time",
        kleio::LocalWorldDiagnosticKind::EventMissingSource => "event-missing-source",
        kleio::LocalWorldDiagnosticKind::RelationshipMissingSource => "relationship-missing-source",
        kleio::LocalWorldDiagnosticKind::ReferencedFileMissing => "referenced-file-missing",
        kleio::LocalWorldDiagnosticKind::RecordUnexpectedPath => "record-unexpected-path",
        kleio::LocalWorldDiagnosticKind::PossibleDuplicatePerson => "possible-duplicate-person",
        kleio::LocalWorldDiagnosticKind::SuspiciousParentChildDirection => {
            "suspicious-parent-child-direction"
        }
    }
}

fn print_world_doctor_report(
    world_root: &Path,
    report: &kleio::LocalWorldDoctorReport,
    level: DoctorLevelArg,
    redact: bool,
) {
    if redact {
        println!("Checked world at [redacted]");
    } else {
        println!("Checked world at {}", world_root.display());
    }
    let diagnostics = report
        .diagnostics
        .iter()
        .filter(|diagnostic| level.includes(diagnostic))
        .collect::<Vec<_>>();
    if diagnostics.is_empty() {
        println!("No authoring warnings found for {:?} checks.", level);
        return;
    }

    println!("Warnings: {}", diagnostics.len());
    for diagnostic in diagnostics {
        if redact {
            println!(
                "- {} warning [details redacted]",
                diagnostic_kind_label(diagnostic.kind)
            );
        } else {
            println!("- {} ({})", diagnostic.message, diagnostic.path);
        }
    }
}

fn print_world_summary(world_root: &Path, summary: &kleio::LocalWorldSummary, redact: bool) {
    if redact {
        println!("World: [redacted]");
        println!("Path: [redacted]");
    } else {
        let title = summary
            .world_title
            .as_deref()
            .or(summary.world_id.as_deref())
            .unwrap_or("unknown world");
        println!("World: {title}");
        println!("Path: {}", world_root.display());
    }
    println!("People: {}", summary.counts.people);
    println!("Places: {}", summary.counts.places);
    println!("Organizations: {}", summary.counts.organizations);
    println!("Objects: {}", summary.counts.objects);
    println!("Concepts: {}", summary.counts.concepts);
    println!("Events: {}", summary.counts.events);
    for (event_type, count) in &summary.counts.events_by_type {
        println!("  {event_type}: {count}");
    }
    println!("Relationships: {}", summary.counts.relationships);
    println!("Sources: {}", summary.counts.sources);
    println!("Assertions: {}", summary.counts.assertions);
    println!("Collections: {}", summary.counts.collections);
    println!(
        "Views: {} timelines, {} trees, {} maps, {} calendars, {} visualizations",
        summary.counts.timeline_views,
        summary.counts.tree_views,
        summary.counts.map_views,
        summary.counts.calendar_views,
        summary.counts.visualization_views
    );

    if summary.warnings.is_empty() {
        println!("\nNeeds attention: none found");
    } else {
        println!("\nNeeds attention:");
        for warning in &summary.warnings {
            if redact {
                println!(
                    "- {} warning [details redacted]",
                    summary_warning_kind_label(warning.kind)
                );
            } else {
                println!("- {} ({})", warning.message, warning.path);
            }
        }
    }
}

fn summary_warning_kind_label(kind: kleio::LocalWorldSummaryWarningKind) -> &'static str {
    match kind {
        kleio::LocalWorldSummaryWarningKind::PersonMissingBirthEvent => {
            "person-missing-birth-event"
        }
        kleio::LocalWorldSummaryWarningKind::EventMissingTime => "event-missing-time",
        kleio::LocalWorldSummaryWarningKind::EventMissingSource => "event-missing-source",
        kleio::LocalWorldSummaryWarningKind::RelationshipMissingSource => {
            "relationship-missing-source"
        }
        kleio::LocalWorldSummaryWarningKind::ReferencedFileMissing => "referenced-file-missing",
        kleio::LocalWorldSummaryWarningKind::RecordUnexpectedPath => "record-unexpected-path",
        kleio::LocalWorldSummaryWarningKind::PossibleDuplicatePerson => "possible-duplicate-person",
        kleio::LocalWorldSummaryWarningKind::SuspiciousParentChildDirection => {
            "suspicious-parent-child-direction"
        }
    }
}

fn default_data_root() -> PathBuf {
    if let Some(path) = std::env::var_os("KLEIO_DATA_DIR").filter(|value| !value.is_empty()) {
        return PathBuf::from(path);
    }

    if let Some(path) = std::env::var_os("XDG_DATA_HOME").filter(|value| !value.is_empty()) {
        return Path::new(&path).join("kleio");
    }

    if let Some(home) = std::env::var_os("HOME").filter(|value| !value.is_empty()) {
        return Path::new(&home).join(".local/share/kleio");
    }

    PathBuf::from(".kleio-data")
}

fn main() -> ExitCode {
    let args = Args::parse();
    match run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("kleio-cli: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Command::InitWorkspace {
            root,
            world,
            title,
            person_slug,
            person_name,
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            force,
        } => {
            let root = resolve_data_root(root);
            let options = LocalSkeletonOptions {
                project_id: world,
                title,
                person_slug,
                person_name,
                birth_date,
                birth_location,
                birth_latitude,
                birth_longitude,
                force,
            };
            create_workspace_skeleton(&root, &options)?;
            let world_root = WorkspacePaths::new(&root)
                .world(options.world_slug())
                .root()
                .to_path_buf();
            println!("created Kleio workspace at {}", root.display());
            println!("created default world at {}", world_root.display());
            print_workspace_next_steps(&root, options.world_slug(), &options.person_slug);
        }
        Command::Init {
            root,
            project_id,
            title,
            person_slug,
            person_name,
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            force,
        } => {
            let root = resolve_data_root(root);
            let options = LocalSkeletonOptions {
                project_id,
                title,
                person_slug,
                person_name,
                birth_date,
                birth_location,
                birth_latitude,
                birth_longitude,
                force,
            };
            create_workspace_skeleton(&root, &options)?;
            println!("created Kleio workspace at {}", root.display());
            print_workspace_next_steps(&root, options.world_slug(), &options.person_slug);
        }
        Command::NewWorld {
            root,
            world,
            title,
            set_default,
            starter,
            person_slug,
            person_name,
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            force,
        } => {
            let root = resolve_data_root(root);
            let title = title.unwrap_or_else(|| world.clone());
            let options = LocalSkeletonOptions {
                project_id: world,
                title,
                person_slug,
                person_name,
                birth_date,
                birth_location,
                birth_latitude,
                birth_longitude,
                force,
            };
            let world_root = WorkspacePaths::new(&root)
                .world(options.world_slug())
                .root()
                .to_path_buf();
            if starter {
                create_world_skeleton(&world_root, &options)?;
            } else {
                create_world_layout(&world_root, &options)?;
            }
            let config_path = WorkspacePaths::new(&root).config();
            let mut config = if config_path.exists() {
                read_workspace_config(&root)?
            } else {
                WorkspaceConfig::with_default_world(options.world_slug(), &options.title)
            };
            config.upsert_world(options.world_slug(), &options.title);
            if set_default {
                config.workspace.default_world = options.world_slug().to_string();
            }
            write_workspace_config(&root, &config)?;
            println!("created world at {}", world_root.display());
            if starter {
                print_workspace_next_steps(&root, options.world_slug(), &options.person_slug);
            } else {
                println!(
                    "next: add records with `kleio-cli add-relative`, `kleio-cli add-event`, or lower-level `new-*` commands, then check with `kleio-cli summary --world {}` and `kleio-cli doctor --world {}`",
                    options.world_slug(),
                    options.world_slug()
                );
            }
        }
        Command::ListWorlds { root } => {
            let root = resolve_data_root(root);
            let config = read_workspace_config(&root)?;
            for world in &config.worlds {
                let marker = if world.slug == config.workspace.default_world {
                    "*"
                } else {
                    " "
                };
                println!(
                    "{marker} {}\t{}\t{}",
                    world.slug,
                    world.title,
                    root.join(&world.path).display()
                );
            }
        }
        Command::SetDefaultWorld { root, world } => {
            let root = resolve_data_root(root);
            let mut config = read_workspace_config(&root)?;
            if config.world_entry(&world).is_none() {
                return Err(cli_error(format!(
                    "world `{world}` is not registered in {}",
                    WorkspacePaths::new(&root).config().display()
                )));
            }
            config.workspace.default_world = world.clone();
            write_workspace_config(&root, &config)?;
            println!("set default world to `{world}`");
        }
        Command::Guide {
            root,
            world,
            person,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_authoring_guide(&world_root, person.as_deref())?;
        }
        Command::NewPerson {
            root,
            world,
            person_slug,
            person_name,
            sex,
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            no_birth_event,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            create_local_person(
                &world_root,
                &LocalPersonOptions {
                    person_slug: person_slug.clone(),
                    person_name: person_name
                        .unwrap_or_else(|| inferred_preferred_name(&person_slug)),
                    sex: sex.map(|sex| sex.as_str().to_string()),
                    birth_date,
                    birth_location,
                    birth_latitude,
                    birth_longitude,
                    create_birth_event: !no_birth_event,
                    force,
                },
            )?;
            println!("created person record under {}", world_root.display());
        }
        Command::AddRelative {
            root,
            world,
            relative_slug,
            relation,
            person,
            person_name,
            sex,
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            no_birth_event,
            kind,
            parent_role,
            source_records,
            force,
        } => {
            let relationship_kind =
                kind.unwrap_or_else(|| relation.default_relationship_kind().to_string());
            let (source, target) =
                relative_relationship_endpoints(relation, &person, &relative_slug);
            let existing_person_id = person_id(&person);
            create_family_person_relationship(
                root,
                world.as_deref(),
                relative_slug.clone(),
                person_name.unwrap_or_else(|| inferred_preferred_name(&relative_slug)),
                birth_date,
                birth_location,
                birth_latitude,
                birth_longitude,
                sex.map(|sex| sex.as_str().to_string()),
                !no_birth_event,
                relationship_slug(&person, &relative_slug, relation.slug_suffix()),
                None,
                relationship_kind,
                parent_role.map(|role| role.as_str().to_string()),
                source,
                target,
                vec![existing_person_id],
                source_records,
                force,
            )?;
        }
        Command::ConnectRelative {
            root,
            world,
            relative,
            relation,
            person,
            kind,
            title,
            parent_role,
            slug,
            source_records,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let relative_id = person_id(&relative);
            let person_id = person_id(&person);
            ensure_people_exist(&world_root, &[relative_id.clone(), person_id.clone()])?;
            let relationship_kind =
                kind.unwrap_or_else(|| relation.default_relationship_kind().to_string());
            let (source, target) = relative_relationship_endpoints(relation, &person, &relative);
            let relationship_slug =
                slug.unwrap_or_else(|| relationship_slug(&source, &target, relation.slug_suffix()));
            let parent_role = parent_role.map(|role| role.as_str().to_string());
            let path = create_local_relationship(
                &world_root,
                &LocalRelationshipOptions {
                    relationship_slug,
                    title,
                    relationship_kind,
                    parent_role,
                    source,
                    target,
                    sources: source_records,
                    force,
                },
            )?;
            println!("created relationship record at {}", path.display());
        }
        Command::AddSpouse {
            root,
            world,
            spouse_slug,
            person,
            person_name,
            sex,
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            no_birth_event,
            marriage_slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            relationship_sources,
            force,
        } => create_spouse_and_marriage(
            root,
            world.as_deref(),
            spouse_slug.clone(),
            person,
            person_name.unwrap_or_else(|| inferred_preferred_name(&spouse_slug)),
            sex.map(|sex| sex.as_str().to_string()),
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            !no_birth_event,
            marriage_slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            relationship_sources,
            force,
        )?,
        Command::AddMarriage {
            root,
            world,
            first_person,
            second_person,
            slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            relationship_sources,
            force,
        } => create_marriage_event(
            root,
            world.as_deref(),
            first_person,
            second_person,
            slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            relationship_sources,
            force,
        )?,
        Command::AddDivorce {
            root,
            world,
            first_person,
            second_person,
            slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            relationship_sources,
            force,
        } => create_divorce_event(
            root,
            world.as_deref(),
            first_person,
            second_person,
            slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            relationship_sources,
            force,
        )?,
        Command::AddDeath {
            root,
            world,
            person,
            slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            force,
        } => create_death_event(
            root,
            world.as_deref(),
            person,
            slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            force,
        )?,
        Command::AddEvent {
            root,
            world,
            event_slug,
            event_type,
            people,
            partner,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            create_relationship,
            relationship_slug,
            relationship_kind,
            relationship_sources,
            force,
        } => {
            let relationship_kind = if event_type == "marriage" && partner.is_some() {
                "spouse".to_string()
            } else if event_type == "divorce" && partner.is_some() {
                "former-spouse".to_string()
            } else {
                relationship_kind
            };
            create_event_with_optional_relationship(
                root,
                world.as_deref(),
                event_slug,
                event_type,
                title,
                people,
                partner,
                places,
                location,
                latitude,
                longitude,
                time,
                date_precision,
                sources,
                create_relationship,
                relationship_slug,
                relationship_kind,
                relationship_sources,
                force,
            )?
        }
        Command::NewPlace {
            root,
            world,
            place_slug,
            title,
            force,
        } => {
            create_entity_record(
                root,
                world.as_deref(),
                place_slug,
                title,
                LocalEntityKind::Place,
                force,
            )?;
        }
        Command::NewOrganization {
            root,
            world,
            organization_slug,
            title,
            force,
        } => {
            create_entity_record(
                root,
                world.as_deref(),
                organization_slug,
                title,
                LocalEntityKind::Organization,
                force,
            )?;
        }
        Command::NewObject {
            root,
            world,
            object_slug,
            title,
            force,
        } => {
            create_entity_record(
                root,
                world.as_deref(),
                object_slug,
                title,
                LocalEntityKind::Object,
                force,
            )?;
        }
        Command::NewConcept {
            root,
            world,
            concept_slug,
            title,
            force,
        } => {
            create_entity_record(
                root,
                world.as_deref(),
                concept_slug,
                title,
                LocalEntityKind::Concept,
                force,
            )?;
        }
        Command::NewEvent {
            root,
            world,
            event_slug,
            event_type,
            title,
            participants,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_event(
                &world_root,
                &LocalEventOptions {
                    event_slug,
                    event_type,
                    title,
                    subject: None,
                    participants,
                    places,
                    location,
                    latitude,
                    longitude,
                    time,
                    date_precision,
                    sources,
                    force,
                },
            )?;
            println!("created event record at {}", path.display());
        }
        Command::NewCollection {
            root,
            world,
            collection_slug,
            title,
            sequence,
            order,
            members,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let kind = if sequence {
                LocalCollectionKind::Sequence
            } else {
                LocalCollectionKind::Set
            };
            let path = create_local_collection(
                &world_root,
                &LocalCollectionOptions {
                    collection_slug,
                    title,
                    kind,
                    order: order.into(),
                    members,
                    force,
                },
            )?;
            println!("created event collection at {}", path.display());
        }
        Command::NewRelationship {
            root,
            world,
            relationship_slug,
            title,
            kind,
            parent_role,
            source,
            target,
            source_records,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_relationship(
                &world_root,
                &LocalRelationshipOptions {
                    relationship_slug,
                    title,
                    relationship_kind: kind,
                    parent_role: parent_role.map(|role| role.as_str().to_string()),
                    source,
                    target,
                    sources: source_records,
                    force,
                },
            )?;
            println!("created relationship record at {}", path.display());
        }
        Command::NewSource {
            root,
            world,
            source_slug,
            title,
            kind,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_source(
                &world_root,
                &LocalSourceOptions {
                    source_slug,
                    title,
                    source_kind: kind,
                    force,
                },
            )?;
            println!("created source record at {}", path.display());
        }
        Command::NewAssertion {
            root,
            world,
            assertion_slug,
            kind,
            target,
            value,
            sources,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_assertion(
                &world_root,
                &LocalAssertionOptions {
                    assertion_slug,
                    assertion_kind: kind,
                    target,
                    value,
                    sources,
                    confidence: None,
                    note: None,
                    force,
                },
            )?;
            println!("created assertion record at {}", path.display());
        }
        Command::NewBirth {
            root,
            world,
            person_slug,
            person_name,
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            create_local_birth_event(
                &world_root,
                &LocalBirthEventOptions {
                    person_slug,
                    person_name,
                    birth_date,
                    birth_location,
                    birth_latitude,
                    birth_longitude,
                    force,
                },
            )?;
            println!("created birth event under {}", world_root.display());
        }
        Command::NewTimeline {
            root,
            world,
            timeline_slug,
            title,
            subject,
            force,
        } => create_view_record(
            root,
            world.as_deref(),
            timeline_slug,
            title,
            LocalViewKind::Timeline,
            subject,
            force,
        )?,
        Command::NewTree {
            root,
            world,
            tree_slug,
            title,
            subject,
            force,
        } => create_view_record(
            root,
            world.as_deref(),
            tree_slug,
            title,
            LocalViewKind::Tree,
            subject,
            force,
        )?,
        Command::NewMap {
            root,
            world,
            map_slug,
            title,
            force,
        } => create_view_record(
            root,
            world.as_deref(),
            map_slug,
            title,
            LocalViewKind::Map,
            None,
            force,
        )?,
        Command::NewCalendar {
            root,
            world,
            calendar_slug,
            title,
            force,
        } => create_view_record(
            root,
            world.as_deref(),
            calendar_slug,
            title,
            LocalViewKind::Calendar,
            None,
            force,
        )?,
        Command::NewVisualization {
            root,
            world,
            visualization_slug,
            title,
            force,
        } => create_view_record(
            root,
            world.as_deref(),
            visualization_slug,
            title,
            LocalViewKind::Visualization,
            None,
            force,
        )?,
        Command::NewSchema {
            root,
            world,
            schema_slug,
            title,
            kind,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_schema(
                &world_root,
                &LocalSchemaOptions {
                    schema_slug,
                    title,
                    kind: kind.into(),
                    force,
                },
            )?;
            println!("created schema record at {}", path.display());
        }
        Command::NewImportReport {
            root,
            world,
            import_slug,
            title,
            kind,
            source_path,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_import_report(
                &world_root,
                &LocalImportReportOptions {
                    import_slug,
                    kind: kind.into(),
                    title,
                    source_path,
                    force,
                },
            )?;
            println!("created import report at {}", path.display());
        }
        Command::ListPeople {
            root,
            world,
            filter,
            redact,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_people(&world_root, filter.as_deref(), redact)?;
        }
        Command::ListEvents {
            root,
            world,
            filter,
            redact,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_events(&world_root, filter.as_deref(), redact)?;
        }
        Command::ListSources {
            root,
            world,
            filter,
            redact,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_sources(&world_root, filter.as_deref(), redact)?;
        }
        Command::ListViews { root, world, kind } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let views = list_local_views(&world_root, kind.map(Into::into))?;
            for view in views {
                println!(
                    "{}\t{}\t{}",
                    view.id.unwrap_or_else(|| "-".to_string()),
                    view.kind,
                    view.path
                );
            }
        }
        Command::TreeSketch {
            root,
            world,
            person,
            depth,
            redact,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_tree_sketch(&world_root, person.as_deref(), depth, redact)?;
        }
        Command::SetGedcom {
            root,
            world,
            path,
            strategy,
            allow_missing,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            set_primary_gedcom_import(
                &world_root,
                &PrimaryGedcomImportOptions {
                    path,
                    strategy,
                    allow_missing,
                },
            )?;
            println!(
                "updated primary GEDCOM import in {}/world.toml",
                world_root.display()
            );
        }
        Command::IngestGedcom {
            root,
            world,
            path,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let report = ingest_primary_gedcom_to_world(
                &world_root,
                &LocalGedcomIngestOptions { path, force },
            )?;
            println!(
                "ingested GEDCOM into {}: {} people, {} places, {} events, {} relationships, {} assertions, {} sources ({} existing records skipped)",
                world_root.display(),
                report.people,
                report.places,
                report.events,
                report.relationships,
                report.assertions,
                report.sources,
                report.skipped_existing
            );
        }
        Command::AttachSource {
            root,
            world,
            target,
            source,
            kind,
            value,
            confidence,
            note,
            slug,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let assertion_slug =
                slug.unwrap_or_else(|| assertion_slug_from_target_source(&target, &source));
            let path = create_local_assertion(
                &world_root,
                &LocalAssertionOptions {
                    assertion_slug,
                    assertion_kind: kind,
                    target,
                    value,
                    sources: vec![source],
                    confidence: Some(confidence),
                    note,
                    force,
                },
            )?;
            println!("created source assertion at {}", path.display());
        }
        Command::RedactFile { path, redact_body } => {
            print!("{}", redact_file(&path, redact_body)?);
        }
        Command::RedactWorld {
            root,
            world,
            out,
            redact_body,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let files = redact_world(&world_root, &out, redact_body, force)?;
            println!("wrote {files} redacted authored files to {}", out.display());
        }
        Command::RedactWorldDump {
            root,
            world,
            redact_body,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print!("{}", redact_world_dump(&world_root, redact_body)?);
        }
        Command::RedactWorldTree { root, world } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            for path in redact_world_tree(&world_root)? {
                println!("{}", path.display());
            }
        }
        Command::Validate { root, world } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let report = validate_local_world(&world_root)?;
            println!(
                "validated world at {}: {} Markdown records, {} TOML documents",
                world_root.display(),
                report.markdown_records,
                report.toml_documents
            );
        }
        Command::CheckMedia {
            root,
            world,
            all,
            redact,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let report = check_local_media(&world_root)?;
            print_media_check_report(&world_root, &report, all, redact);
        }
        Command::Doctor {
            root,
            world,
            strict,
            level,
            redact,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let report = doctor_local_world(&world_root)?;
            let warning_count = report
                .diagnostics
                .iter()
                .filter(|diagnostic| level.includes(diagnostic))
                .count();
            print_world_doctor_report(&world_root, &report, level, redact);
            if strict && warning_count > 0 {
                return Err(cli_error(format!(
                    "doctor found {warning_count} authoring warnings"
                )));
            }
        }
        Command::Summary {
            root,
            world,
            redact,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let summary = summarize_local_world(&world_root)?;
            print_world_summary(&world_root, &summary, redact);
        }
        Command::Compile { root, world, out } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let out = out.unwrap_or(build_paths.compiled_json);
            let bundle = write_local_data_json(&world_root, &out)?;
            println!(
                "wrote {} records and {} TOML documents to {}",
                bundle.markdown_records.len(),
                bundle.toml_documents.len(),
                out.display()
            );
        }
        Command::CompileEcs { root, world, out } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let out = out.unwrap_or(build_paths.ecs_json);
            let bundle = write_local_ecs_json(&world_root, &out)?;
            println!(
                "wrote ECS bundle with {} entities to {}",
                bundle.entities.len(),
                out.display()
            );
        }
        Command::Build {
            root,
            world,
            timeline_view,
            tree_view,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let output = build_local_world_with_options(
                &world_root,
                &LocalWorldBuildOptions {
                    timeline_view: timeline_view.as_deref(),
                    tree_view: tree_view.as_deref(),
                },
            )?;
            println!(
                "built world at {}: {} Markdown records, {} TOML documents, {} ECS entities",
                world_root.display(),
                output.markdown_records,
                output.toml_documents,
                output.ecs_entities
            );
            if let (Some(path), Some(events), Some(collections)) = (
                &output.timeline_json_path,
                output.timeline_events,
                output.timeline_collections,
            ) {
                println!(
                    "wrote timeline projection with {events} events and {collections} collections to {}",
                    path.display()
                );
            }
            if let (Some(path), Some(people), Some(events), Some(relationships)) = (
                &output.tree_json_path,
                output.tree_people,
                output.tree_events,
                output.tree_relationships,
            ) {
                println!(
                    "wrote tree projection with {people} people, {events} events, and {relationships} relationships to {}",
                    path.display()
                );
            }
        }
        Command::CompileTimeline {
            root,
            world,
            view,
            out,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let build_dir = build_paths
                .compiled_json
                .parent()
                .unwrap_or(world_root.as_path());
            let output_slug = view.as_deref().unwrap_or("timeline");
            let out = out.unwrap_or_else(|| build_dir.join(format!("{output_slug}.timeline.json")));
            let timeline = write_local_timeline_json(&world_root, view.as_deref(), &out)?;
            println!(
                "wrote timeline projection with {} events and {} collections to {}",
                timeline.events.len(),
                timeline.collections.len(),
                out.display()
            );
        }
        Command::CompileTree {
            root,
            world,
            view,
            out,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let build_dir = build_paths
                .compiled_json
                .parent()
                .unwrap_or(world_root.as_path());
            let out = out.unwrap_or_else(|| {
                view.as_deref()
                    .map(|view| build_dir.join(format!("{view}.tree.json")))
                    .unwrap_or_else(|| build_dir.join("kleio-tree.json"))
            });
            let tree = write_local_tree_json_with_view(&world_root, view.as_deref(), &out)?;
            println!(
                "wrote tree with {} people, {} events, and {} relationships to {}",
                tree.people.len(),
                tree.events.len(),
                tree.relationships.len(),
                out.display()
            );
        }
    }
    Ok(())
}
