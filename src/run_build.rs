use super::*;
use crate::authoring::*;
use crate::cli::Command;

pub(crate) fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Compile(crate::cli_build::CompileArgs { root, world, out }) => {
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
        Command::CompileEcs(crate::cli_build::CompileEcsArgs { root, world, out }) => {
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
        Command::Build(crate::cli_build::BuildArgs {
            root,
            world,
            timeline_view,
            tree_view,
        }) => {
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
        Command::CompileTimeline(crate::cli_build::CompileTimelineArgs {
            root,
            world,
            view,
            out,
        }) => {
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
        Command::CompileTree(crate::cli_build::CompileTreeArgs {
            root,
            world,
            view,
            out,
        }) => {
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
        _ => unreachable!("command routed to the wrong handler"),
    }
    Ok(())
}
