use super::*;
use crate::authoring::*;
use crate::cli::Command;

pub(crate) fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::NewPerson(crate::cli_people::NewPersonArgs {
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
        }) => {
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
        Command::AddRelative(crate::cli_people::AddRelativeArgs {
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
        }) => {
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
        Command::ConnectRelative(crate::cli_people::ConnectRelativeArgs {
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
        }) => {
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
        Command::AddSpouse(crate::cli_people::AddSpouseArgs {
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
        }) => create_spouse_and_marriage(
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
        _ => unreachable!("command routed to the wrong handler"),
    }
    Ok(())
}
