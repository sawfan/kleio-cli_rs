use super::*;
use crate::authoring::*;
use crate::cli::Command;

pub(crate) fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::AddMarriage(crate::cli_events::AddMarriageArgs {
            root,
            world,
            first_person,
            second_person,
            slug,
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
        }) => create_marriage_event(
            root,
            world.as_deref(),
            first_person,
            second_person,
            slug,
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
        Command::AddDivorce(crate::cli_events::AddDivorceArgs {
            root,
            world,
            first_person,
            second_person,
            slug,
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
        }) => create_divorce_event(
            root,
            world.as_deref(),
            first_person,
            second_person,
            slug,
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
        Command::AddDeath(crate::cli_events::AddDeathArgs {
            root,
            world,
            person,
            slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            force,
        }) => create_death_event(
            root,
            world.as_deref(),
            person,
            slug,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            force,
        )?,
        Command::AddEvent(crate::cli_events::AddEventArgs {
            root,
            world,
            event_slug,
            event_type,
            people,
            partner,
            title,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            create_relationship,
            relationship_slug,
            relationship_kind,
            relationship_sources,
            force,
        }) => {
            let relationship_kind = if event_type == "marriage" && partner.is_some() {
                "spouse".to_string()
            } else if event_type == "divorce" && partner.is_some() {
                "former-spouse".to_string()
            } else {
                relationship_kind
            };
            create_event_with_optional_relationship(
                root,
                world.as_deref(),
                event_slug,
                event_type,
                title,
                people,
                partner,
                places,
                location,
                latitude,
                longitude,
                time,
                date_precision,
                sources,
                create_relationship,
                relationship_slug,
                relationship_kind,
                relationship_sources,
                force,
            )?
        }
        Command::NewEvent(crate::cli_events::NewEventArgs {
            root,
            world,
            event_slug,
            event_type,
            title,
            participants,
            places,
            location,
            latitude,
            longitude,
            time,
            date_precision,
            sources,
            force,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_event(
                &world_root,
                &LocalEventOptions {
                    event_slug,
                    event_type,
                    title,
                    subject: None,
                    participants,
                    places,
                    location,
                    latitude,
                    longitude,
                    time,
                    date_precision,
                    sources,
                    force,
                },
            )?;
            println!("created event record at {}", path.display());
        }
        Command::NewBirth(crate::cli_events::NewBirthArgs {
            root,
            world,
            person_slug,
            person_name,
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            force,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            create_local_birth_event(
                &world_root,
                &LocalBirthEventOptions {
                    person_slug,
                    person_name,
                    birth_date,
                    birth_location,
                    birth_latitude,
                    birth_longitude,
                    force,
                },
            )?;
            println!("created birth event under {}", world_root.display());
        }
        _ => unreachable!("command routed to the wrong handler"),
    }
    Ok(())
}
