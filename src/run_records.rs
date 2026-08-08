use super::*;
use crate::authoring::*;
use crate::cli::Command;

pub(crate) fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::NewPlace(crate::cli_records::NewPlaceArgs {
            root,
            world,
            place_slug,
            title,
            force,
        }) => {
            create_entity_record(
                root,
                world.as_deref(),
                place_slug,
                title,
                LocalEntityKind::Place,
                force,
            )?;
        }
        Command::NewOrganization(crate::cli_records::NewOrganizationArgs {
            root,
            world,
            organization_slug,
            title,
            force,
        }) => {
            create_entity_record(
                root,
                world.as_deref(),
                organization_slug,
                title,
                LocalEntityKind::Organization,
                force,
            )?;
        }
        Command::NewObject(crate::cli_records::NewObjectArgs {
            root,
            world,
            object_slug,
            title,
            force,
        }) => {
            create_entity_record(
                root,
                world.as_deref(),
                object_slug,
                title,
                LocalEntityKind::Object,
                force,
            )?;
        }
        Command::NewConcept(crate::cli_records::NewConceptArgs {
            root,
            world,
            concept_slug,
            title,
            force,
        }) => {
            create_entity_record(
                root,
                world.as_deref(),
                concept_slug,
                title,
                LocalEntityKind::Concept,
                force,
            )?;
        }
        Command::NewCollection(crate::cli_records::NewCollectionArgs {
            root,
            world,
            collection_slug,
            title,
            sequence,
            order,
            members,
            force,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let kind = if sequence {
                LocalCollectionKind::Sequence
            } else {
                LocalCollectionKind::Set
            };
            let path = create_local_collection(
                &world_root,
                &LocalCollectionOptions {
                    collection_slug,
                    title,
                    kind,
                    order: order.into(),
                    members,
                    force,
                },
            )?;
            println!("created event collection at {}", path.display());
        }
        Command::NewRelationship(crate::cli_records::NewRelationshipArgs {
            root,
            world,
            relationship_slug,
            title,
            kind,
            parent_role,
            source,
            target,
            source_records,
            force,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_relationship(
                &world_root,
                &LocalRelationshipOptions {
                    relationship_slug,
                    title,
                    relationship_kind: kind,
                    parent_role: parent_role.map(|role| role.as_str().to_string()),
                    source,
                    target,
                    sources: source_records,
                    force,
                },
            )?;
            println!("created relationship record at {}", path.display());
        }
        Command::NewSource(crate::cli_records::NewSourceArgs {
            root,
            world,
            source_slug,
            title,
            kind,
            force,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_source(
                &world_root,
                &LocalSourceOptions {
                    source_slug,
                    title,
                    source_kind: kind,
                    force,
                },
            )?;
            println!("created source record at {}", path.display());
        }
        Command::NewAssertion(crate::cli_records::NewAssertionArgs {
            root,
            world,
            assertion_slug,
            kind,
            target,
            value,
            sources,
            force,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_assertion(
                &world_root,
                &LocalAssertionOptions {
                    assertion_slug,
                    assertion_kind: kind,
                    target,
                    value,
                    sources,
                    confidence: None,
                    note: None,
                    force,
                },
            )?;
            println!("created assertion record at {}", path.display());
        }
        Command::NewTimeline(crate::cli_records::NewTimelineArgs {
            root,
            world,
            timeline_slug,
            title,
            subject,
            force,
        }) => create_view_record(
            root,
            world.as_deref(),
            timeline_slug,
            title,
            LocalViewKind::Timeline,
            subject,
            force,
        )?,
        Command::NewTree(crate::cli_records::NewTreeArgs {
            root,
            world,
            tree_slug,
            title,
            subject,
            force,
        }) => create_view_record(
            root,
            world.as_deref(),
            tree_slug,
            title,
            LocalViewKind::Tree,
            subject,
            force,
        )?,
        Command::NewMap(crate::cli_records::NewMapArgs {
            root,
            world,
            map_slug,
            title,
            force,
        }) => create_view_record(
            root,
            world.as_deref(),
            map_slug,
            title,
            LocalViewKind::Map,
            None,
            force,
        )?,
        Command::NewCalendar(crate::cli_records::NewCalendarArgs {
            root,
            world,
            calendar_slug,
            title,
            force,
        }) => create_view_record(
            root,
            world.as_deref(),
            calendar_slug,
            title,
            LocalViewKind::Calendar,
            None,
            force,
        )?,
        Command::NewVisualization(crate::cli_records::NewVisualizationArgs {
            root,
            world,
            visualization_slug,
            title,
            force,
        }) => create_view_record(
            root,
            world.as_deref(),
            visualization_slug,
            title,
            LocalViewKind::Visualization,
            None,
            force,
        )?,
        Command::NewSchema(crate::cli_records::NewSchemaArgs {
            root,
            world,
            schema_slug,
            title,
            kind,
            force,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_schema(
                &world_root,
                &LocalSchemaOptions {
                    schema_slug,
                    title,
                    kind: kind.into(),
                    force,
                },
            )?;
            println!("created schema record at {}", path.display());
        }
        Command::NewImportReport(crate::cli_records::NewImportReportArgs {
            root,
            world,
            import_slug,
            title,
            kind,
            source_path,
            force,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let path = create_local_import_report(
                &world_root,
                &LocalImportReportOptions {
                    import_slug,
                    kind: kind.into(),
                    title,
                    source_path,
                    force,
                },
            )?;
            println!("created import report at {}", path.display());
        }
        Command::AttachSource(crate::cli_records::AttachSourceArgs {
            root,
            world,
            target,
            source,
            kind,
            value,
            confidence,
            note,
            slug,
            force,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let assertion_slug =
                slug.unwrap_or_else(|| assertion_slug_from_target_source(&target, &source));
            let path = create_local_assertion(
                &world_root,
                &LocalAssertionOptions {
                    assertion_slug,
                    assertion_kind: kind,
                    target,
                    value,
                    sources: vec![source],
                    confidence: Some(confidence),
                    note,
                    force,
                },
            )?;
            println!("created source assertion at {}", path.display());
        }
        _ => unreachable!("command routed to the wrong handler"),
    }
    Ok(())
}
