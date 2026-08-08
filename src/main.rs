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
    WorkspacePaths, build_local_world_with_options, check_local_media, compile_local_ecs,
    compile_local_tree, create_local_assertion, create_local_birth_event, create_local_collection,
    create_local_entity, create_local_event, create_local_import_report, create_local_person,
    create_local_relationship, create_local_schema, create_local_source, create_local_view,
    create_workspace_skeleton, create_world_layout, create_world_skeleton, doctor_local_world,
    list_local_views, read_local_data_unvalidated, read_workspace_config,
    resolve_workspace_world_root, resolve_world_build_paths, summarize_local_world,
    validate_local_world, write_local_data_json, write_local_ecs_json, write_local_timeline_json,
    write_local_tree_json_with_view, write_workspace_config,
};
use kleio_gedcom::{
    LocalGedcomIngestOptions, PrimaryGedcomImportOptions, ingest_primary_gedcom_to_world,
    set_primary_gedcom_import,
};

mod authoring;
mod cli;
mod cli_build;
mod cli_events;
mod cli_gedcom;
mod cli_inspect;
mod cli_people;
mod cli_records;
mod cli_redact;
mod cli_values;
mod cli_workspace;
mod error;
mod listing;
mod redact;
mod reports;
mod run;
mod run_build;
mod run_events;
mod run_gedcom;
mod run_inspect;
mod run_people;
mod run_records;
mod run_redact;
mod run_workspace;
mod tree;

use cli::Args;
use run::run;

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
