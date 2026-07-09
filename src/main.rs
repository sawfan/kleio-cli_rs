use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use kleio::{
    DEFAULT_WORLD_SLUG, LocalAssertionOptions, LocalBirthEventOptions, LocalEntityKind,
    LocalEntityOptions, LocalEventOptions, LocalImportKind, LocalImportReportOptions,
    LocalPersonOptions, LocalSchemaKind, LocalSchemaOptions, LocalSkeletonOptions,
    LocalSourceOptions, LocalViewKind, LocalViewOptions, LocalWorldBuildOptions,
    PrimaryGedcomImportOptions, WorkspaceConfig, WorkspacePaths, build_local_world_with_options,
    create_local_assertion, create_local_birth_event, create_local_entity, create_local_event,
    create_local_import_report, create_local_person, create_local_schema, create_local_source,
    create_local_view, create_workspace_skeleton, create_world_layout, create_world_skeleton,
    list_local_views, read_workspace_config, resolve_workspace_world_root,
    resolve_world_build_paths, set_primary_gedcom_import, validate_local_world,
    write_local_data_json, write_local_ecs_json, write_local_timeline_json,
    write_local_tree_json_with_view, write_workspace_config,
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

        /// Starter person display name.
        #[arg(long, default_value = "Example Person")]
        person_name: String,

        /// Optional starter birth date, such as 1900-01-01.
        #[arg(long)]
        birth_date: Option<String>,

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

        /// Starter person display name.
        #[arg(long, default_value = "Example Person")]
        person_name: String,

        /// Optional starter birth date, such as 1900-01-01.
        #[arg(long)]
        birth_date: Option<String>,

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

        /// Starter person display name used when --starter is set.
        #[arg(long, default_value = "Example Person")]
        person_name: String,

        /// Optional starter birth date used when --starter is set.
        #[arg(long)]
        birth_date: Option<String>,

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

        /// Person display name.
        #[arg(long)]
        person_name: String,

        /// Optional birth date for the starter birth event.
        #[arg(long)]
        birth_date: Option<String>,

        /// Skip creating the starter birth event.
        #[arg(long)]
        no_birth_event: bool,

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

    /// Create a semantic event record.
    NewEvent {
        event_slug: String,

        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        #[arg(long)]
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,

        /// Event kind, such as birth, residence, observation, or moment.
        #[arg(long, default_value = "observation")]
        kind: String,

        /// Event title.
        #[arg(long)]
        title: String,

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

        /// Subject id, such as person:example-person.
        #[arg(long)]
        subject: String,

        /// Predicate, such as born_on or has_name.
        #[arg(long)]
        predicate: String,

        /// Claimed value.
        #[arg(long)]
        value: String,

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

    /// Validate world files without writing build outputs.
    Validate {
        /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
        root: Option<PathBuf>,

        /// World slug. Defaults to the workspace default world.
        #[arg(long)]
        world: Option<String>,
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
            force,
        } => {
            let root = resolve_data_root(root);
            let options = LocalSkeletonOptions {
                project_id: world,
                title,
                person_slug,
                person_name,
                birth_date,
                force,
            };
            create_workspace_skeleton(&root, &options)?;
            let world_root = WorkspacePaths::new(&root)
                .world(options.world_slug())
                .root()
                .to_path_buf();
            println!("created Kleio workspace at {}", root.display());
            println!("created default world at {}", world_root.display());
            println!(
                "next: edit world files, then run `kleio-cli compile --world {}`",
                options.world_slug()
            );
        }
        Command::Init {
            root,
            project_id,
            title,
            person_slug,
            person_name,
            birth_date,
            force,
        } => {
            let root = resolve_data_root(root);
            let options = LocalSkeletonOptions {
                project_id,
                title,
                person_slug,
                person_name,
                birth_date,
                force,
            };
            create_workspace_skeleton(&root, &options)?;
            println!("created Kleio workspace at {}", root.display());
            println!(
                "next: edit world files, then run `kleio-cli compile --world {}`",
                options.world_slug()
            );
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
        Command::NewPerson {
            root,
            world,
            person_slug,
            person_name,
            birth_date,
            no_birth_event,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            create_local_person(
                &world_root,
                &LocalPersonOptions {
                    person_slug,
                    person_name,
                    birth_date,
                    create_birth_event: !no_birth_event,
                    force,
                },
            )?;
            println!("created person record under {}", world_root.display());
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
            kind,
            title,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_event(
                &world_root,
                &LocalEventOptions {
                    event_slug,
                    event_kind: kind,
                    title,
                    force,
                },
            )?;
            println!("created event record at {}", path.display());
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
            subject,
            predicate,
            value,
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_assertion(
                &world_root,
                &LocalAssertionOptions {
                    assertion_slug,
                    assertion_kind: kind,
                    subject,
                    predicate,
                    value,
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
            force,
        } => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            create_local_birth_event(
                &world_root,
                &LocalBirthEventOptions {
                    person_slug,
                    person_name,
                    birth_date,
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
            if let (Some(path), Some(events)) = (&output.timeline_json_path, output.timeline_events)
            {
                println!(
                    "wrote timeline projection with {events} events to {}",
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
                "wrote timeline projection with {} events to {}",
                timeline.events.len(),
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
