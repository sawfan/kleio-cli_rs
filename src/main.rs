use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use kleio::{
    LocalBirthEventOptions, LocalPersonOptions, LocalSkeletonOptions, PrimaryGedcomImportOptions,
    create_local_birth_event, create_local_person, create_local_skeleton,
    set_primary_gedcom_import, write_local_data_json, write_local_tree_json,
};

#[derive(Debug, Parser)]
#[command(name = "kleio-cli")]
#[command(about = "Kleio timeline/tree local authoring tools")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a starter local-data/kleio skeleton.
    Init {
        /// Directory to create. Defaults to local-data/kleio.
        #[arg(default_value = "local-data/kleio")]
        root: PathBuf,

        /// Internal project id written to kleio.toml.
        #[arg(long, default_value = "private-timeline")]
        project_id: String,

        /// Human-readable project title.
        #[arg(long, default_value = "Private timeline")]
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

    /// Create a person record, optionally with a starter birth event.
    NewPerson {
        /// Person slug used in filename and id, e.g. alex-example.
        person_slug: String,

        /// Local data root.
        #[arg(long, default_value = "local-data/kleio")]
        root: PathBuf,

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

    /// Create a birth event for an existing person.
    NewBirth {
        /// Person slug used in the person id, e.g. alex-example.
        person_slug: String,

        /// Local data root.
        #[arg(long, default_value = "local-data/kleio")]
        root: PathBuf,

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

    /// Point kleio.toml at the active primary GEDCOM file.
    SetGedcom {
        /// GEDCOM path under the local data root, e.g. imports/gedcom/family.ged.
        path: String,

        /// Local data root.
        #[arg(long, default_value = "local-data/kleio")]
        root: PathBuf,

        /// Import strategy metadata. Currently link/import/merge are planning labels.
        #[arg(long, default_value = "link")]
        strategy: String,

        /// Update kleio.toml even if the GEDCOM file does not exist yet.
        #[arg(long)]
        allow_missing: bool,
    },

    /// Validate and compile local files into a general JSON bundle.
    Compile {
        /// Local data root.
        #[arg(default_value = "local-data/kleio")]
        root: PathBuf,

        /// Output JSON path.
        #[arg(long, default_value = "local-data/kleio/build/kleio.compiled.json")]
        out: PathBuf,
    },

    /// Compile local person records into the current tree JSON projection.
    CompileTree {
        /// Local data root.
        #[arg(default_value = "local-data/kleio")]
        root: PathBuf,

        /// Output JSON path.
        #[arg(long, default_value = "local-data/kleio/build/kleio-tree.json")]
        out: PathBuf,
    },
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
        Command::Init {
            root,
            project_id,
            title,
            person_slug,
            person_name,
            birth_date,
            force,
        } => {
            let options = LocalSkeletonOptions {
                project_id,
                title,
                person_slug,
                person_name,
                birth_date,
                force,
            };
            create_local_skeleton(&root, &options)?;
            println!("created Kleio local skeleton at {}", root.display());
            println!(
                "next: edit people/events, then run `kleio-cli compile {}`",
                root.display()
            );
        }
        Command::NewPerson {
            root,
            person_slug,
            person_name,
            birth_date,
            no_birth_event,
            force,
        } => {
            create_local_person(
                &root,
                &LocalPersonOptions {
                    person_slug,
                    person_name,
                    birth_date,
                    create_birth_event: !no_birth_event,
                    force,
                },
            )?;
            println!("created person record under {}", root.display());
        }
        Command::NewBirth {
            root,
            person_slug,
            person_name,
            birth_date,
            force,
        } => {
            create_local_birth_event(
                &root,
                &LocalBirthEventOptions {
                    person_slug,
                    person_name,
                    birth_date,
                    force,
                },
            )?;
            println!("created birth event under {}", root.display());
        }
        Command::SetGedcom {
            root,
            path,
            strategy,
            allow_missing,
        } => {
            set_primary_gedcom_import(
                &root,
                &PrimaryGedcomImportOptions {
                    path,
                    strategy,
                    allow_missing,
                },
            )?;
            println!(
                "updated primary GEDCOM import in {}/kleio.toml",
                root.display()
            );
        }
        Command::Compile { root, out } => {
            let bundle = write_local_data_json(&root, &out)?;
            println!(
                "wrote {} records and {} TOML documents to {}",
                bundle.markdown_records.len(),
                bundle.toml_documents.len(),
                out.display()
            );
        }
        Command::CompileTree { root, out } => {
            let tree = write_local_tree_json(&root, &out)?;
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
