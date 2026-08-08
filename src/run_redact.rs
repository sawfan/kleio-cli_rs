use crate::authoring::resolve_world_root;
use crate::cli::Command;
use crate::redact::*;

pub(crate) fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::RedactFile(crate::cli_redact::RedactFileArgs { path, redact_body }) => {
            print!("{}", redact_file(&path, redact_body)?);
        }
        Command::RedactWorld(crate::cli_redact::RedactWorldArgs {
            root,
            world,
            out,
            redact_body,
            force,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let files = redact_world(&world_root, &out, redact_body, force)?;
            println!("wrote {files} redacted authored files to {}", out.display());
        }
        Command::RedactWorldDump(crate::cli_redact::RedactWorldDumpArgs {
            root,
            world,
            redact_body,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            print!("{}", redact_world_dump(&world_root, redact_body)?);
        }
        Command::RedactWorldTree(crate::cli_redact::RedactWorldTreeArgs { root, world }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            for path in redact_world_tree(&world_root)? {
                println!("{}", path.display());
            }
        }
        _ => unreachable!("command routed to the wrong handler"),
    }
    Ok(())
}
