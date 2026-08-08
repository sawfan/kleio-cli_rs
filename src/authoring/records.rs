use super::*;

pub(crate) fn create_entity_record(
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

pub(crate) fn create_view_record(
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
