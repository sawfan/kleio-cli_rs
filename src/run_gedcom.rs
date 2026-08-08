use super::*;
use crate::authoring::resolve_world_root;
use crate::cli::Command;

pub(crate) fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::SetGedcom(crate::cli_gedcom::SetGedcomArgs {
            root,
            world,
            path,
            strategy,
            allow_missing,
        }) => {
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
        Command::IngestGedcom(crate::cli_gedcom::IngestGedcomArgs {
            root,
            world,
            path,
            force,
        }) => {
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
        _ => unreachable!("command routed to the wrong handler"),
    }
    Ok(())
}
