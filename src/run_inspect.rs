use super::*;
use crate::authoring::*;
use crate::cli::Command;
use crate::error::cli_error;
use crate::listing::*;
use crate::reports::*;
use crate::tree::*;

pub(crate) fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::ListPeople(crate::cli_inspect::ListPeopleArgs {
            root,
            world,
            filter,
            redact,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_people(&world_root, filter.as_deref(), redact)?;
        }
        Command::ListEvents(crate::cli_inspect::ListEventsArgs {
            root,
            world,
            filter,
            redact,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_events(&world_root, filter.as_deref(), redact)?;
        }
        Command::ListSources(crate::cli_inspect::ListSourcesArgs {
            root,
            world,
            filter,
            redact,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_sources(&world_root, filter.as_deref(), redact)?;
        }
        Command::ListViews(crate::cli_inspect::ListViewsArgs { root, world, kind }) => {
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
        Command::TreeSketch(crate::cli_inspect::TreeSketchArgs {
            root,
            world,
            person,
            depth,
            redact,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_tree_sketch(&world_root, person.as_deref(), depth, redact)?;
        }
        Command::InspectTreeView(crate::cli_inspect::InspectTreeViewArgs {
            root,
            world,
            view,
            person,
            redact,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_tree_view_inspection(&world_root, view.as_deref(), person.as_deref(), redact)?;
        }
        Command::Validate(crate::cli_inspect::ValidateArgs { root, world }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let report = validate_local_world(&world_root)?;
            println!(
                "validated world at {}: {} Markdown records, {} TOML documents",
                world_root.display(),
                report.markdown_records,
                report.toml_documents
            );
        }
        Command::CheckMedia(crate::cli_inspect::CheckMediaArgs {
            root,
            world,
            all,
            redact,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let report = check_local_media(&world_root)?;
            print_media_check_report(&world_root, &report, all, redact);
        }
        Command::Doctor(crate::cli_inspect::DoctorArgs {
            root,
            world,
            strict,
            level,
            redact,
        }) => {
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
        Command::Summary(crate::cli_inspect::SummaryArgs {
            root,
            world,
            redact,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let summary = summarize_local_world(&world_root)?;
            print_world_summary(&world_root, &summary, redact);
        }
        _ => unreachable!("command routed to the wrong handler"),
    }
    Ok(())
}
