use super::*;
use crate::authoring::*;
use crate::cli::Command;
use crate::error::cli_error;
use crate::reports::*;

pub(crate) fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::InitWorkspace(crate::cli_workspace::InitWorkspaceArgs {
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
        }) => {
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
        Command::Init(crate::cli_workspace::InitArgs {
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
        }) => {
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
        Command::NewWorld(crate::cli_workspace::NewWorldArgs {
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
        }) => {
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
        Command::ListWorlds(crate::cli_workspace::ListWorldsArgs { root }) => {
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
        Command::SetDefaultWorld(crate::cli_workspace::SetDefaultWorldArgs { root, world }) => {
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
        Command::Guide(crate::cli_workspace::GuideArgs {
            root,
            world,
            person,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print_authoring_guide(&world_root, person.as_deref())?;
        }
        _ => unreachable!("command routed to the wrong handler"),
    }
    Ok(())
}
