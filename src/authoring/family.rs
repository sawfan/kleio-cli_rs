use super::*;

pub(crate) fn create_spouse_and_marriage(
    root: Option<PathBuf>,
    world: Option<&str>,
    spouse_slug: String,
    existing_person: String,
    person_name: String,
    sex: Option<String>,
    birth_date: Option<String>,
    birth_location: Option<String>,
    birth_latitude: Option<f64>,
    birth_longitude: Option<f64>,
    create_birth_event: bool,
    marriage_slug: Option<String>,
    title: Option<String>,
    places: Vec<String>,
    location: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    time: Option<String>,
    date_precision: Option<String>,
    sources: Vec<String>,
    relationship_sources: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = resolve_data_root(root);
    let world_root = resolve_workspace_world_root(&workspace_root, world)?;
    let existing_files = collect_existing_files(&world_root)?;
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        ensure_people_exist(&world_root, &[existing_person.clone()])?;
        create_local_person(
            &world_root,
            &LocalPersonOptions {
                person_slug: spouse_slug.clone(),
                person_name,
                sex,
                birth_date,
                birth_location,
                birth_latitude,
                birth_longitude,
                create_birth_event,
                force,
            },
        )?;

        create_marriage_event(
            Some(workspace_root),
            world,
            existing_person,
            spouse_slug,
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
        )
    })();

    if result.is_err() && !force {
        rollback_new_files(&world_root, &existing_files)?;
    }

    result
}

pub(crate) fn collect_existing_files(
    root: &Path,
) -> Result<BTreeSet<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = BTreeSet::new();
    collect_existing_files_inner(root, root, &mut files)?;
    Ok(files)
}

pub(crate) fn collect_existing_files_inner(
    root: &Path,
    path: &Path,
    files: &mut BTreeSet<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_existing_files_inner(root, &path, files)?;
        } else {
            files.insert(path.strip_prefix(root).unwrap_or(&path).to_path_buf());
        }
    }

    Ok(())
}

pub(crate) fn rollback_new_files(
    root: &Path,
    existing_files: &BTreeSet<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let current_files = collect_existing_files(root)?;
    for path in current_files.difference(existing_files) {
        let full_path = root.join(path);
        if let Err(source) = fs::remove_file(&full_path) {
            return Err(cli_error(format!(
                "failed to roll back partially-created file {}: {source}",
                full_path.display()
            )));
        }
    }

    Ok(())
}

pub(crate) fn create_marriage_event(
    root: Option<PathBuf>,
    world: Option<&str>,
    first_person: String,
    second_person: String,
    slug: Option<String>,
    title: Option<String>,
    places: Vec<String>,
    location: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    time: Option<String>,
    date_precision: Option<String>,
    sources: Vec<String>,
    relationship_sources: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_slug =
        slug.unwrap_or_else(|| relationship_slug(&first_person, &second_person, "marriage"));
    create_event_with_optional_relationship(
        root,
        world,
        event_slug,
        "marriage".to_string(),
        title,
        vec![first_person],
        Some(second_person),
        places,
        location,
        latitude,
        longitude,
        time,
        date_precision,
        sources,
        true,
        None,
        "spouse".to_string(),
        relationship_sources,
        force,
    )
}

pub(crate) fn create_death_event(
    root: Option<PathBuf>,
    world: Option<&str>,
    person: String,
    slug: Option<String>,
    title: Option<String>,
    places: Vec<String>,
    location: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    time: Option<String>,
    date_precision: Option<String>,
    sources: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_slug = slug.unwrap_or_else(|| format!("{}-death", person_slug_for_filename(&person)));
    create_event_with_optional_relationship(
        root,
        world,
        event_slug,
        "death".to_string(),
        title,
        vec![person],
        None,
        places,
        location,
        latitude,
        longitude,
        time,
        date_precision,
        sources,
        false,
        None,
        "associate".to_string(),
        Vec::new(),
        force,
    )
}

pub(crate) fn create_divorce_event(
    root: Option<PathBuf>,
    world: Option<&str>,
    first_person: String,
    second_person: String,
    slug: Option<String>,
    title: Option<String>,
    places: Vec<String>,
    location: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    time: Option<String>,
    date_precision: Option<String>,
    sources: Vec<String>,
    relationship_sources: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_slug =
        slug.unwrap_or_else(|| relationship_slug(&first_person, &second_person, "divorce"));
    create_event_with_optional_relationship(
        root,
        world,
        event_slug,
        "divorce".to_string(),
        title,
        vec![first_person],
        Some(second_person),
        places,
        location,
        latitude,
        longitude,
        time,
        date_precision,
        sources,
        true,
        None,
        "former-spouse".to_string(),
        relationship_sources,
        force,
    )
}

pub(crate) fn create_event_with_optional_relationship(
    root: Option<PathBuf>,
    world: Option<&str>,
    event_slug: String,
    event_type: String,
    title: Option<String>,
    mut participants: Vec<String>,
    partner: Option<String>,
    places: Vec<String>,
    location: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    time: Option<String>,
    date_precision: Option<String>,
    sources: Vec<String>,
    create_relationship: bool,
    relationship_slug_option: Option<String>,
    relationship_kind: String,
    relationship_sources: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(partner) = partner.clone() {
        participants.push(partner);
    }
    if participants.is_empty() {
        return Err(cli_error(
            "add-event requires at least one --person or --partner",
        ));
    }

    let world_root = resolve_world_root(root, world)?;
    if !participants.is_empty() {
        let mut referenced_people = participants.clone();
        if let Some(partner) = &partner {
            referenced_people.push(partner.clone());
        }
        ensure_people_exist(&world_root, &referenced_people)?;
    }
    let event_subject = (participants.len() == 1).then(|| participants[0].clone());
    let event_participants = if event_subject.is_some() {
        Vec::new()
    } else {
        participants.clone()
    };
    let event_path = create_local_event(
        &world_root,
        &LocalEventOptions {
            event_slug,
            event_type: event_type.clone(),
            title,
            subject: event_subject,
            participants: event_participants,
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
    println!("created event record at {}", event_path.display());

    let should_create_relationship = create_relationship
        || (event_type == "marriage" && partner.is_some())
        || (event_type == "divorce" && partner.is_some());
    if should_create_relationship {
        let Some(first_person) = participants.first() else {
            return Err(cli_error(
                "add-event --create-relationship requires --person",
            ));
        };
        let Some(second_person) = participants.get(1) else {
            return Err(cli_error(
                "add-event --create-relationship requires --partner or a second --person",
            ));
        };
        let first_person_id = person_id(first_person);
        let second_person_id = person_id(second_person);
        let relationship_slug = relationship_slug_option.unwrap_or_else(|| {
            let suffix = if event_type == "marriage" && relationship_kind == "spouse" {
                "spouse"
            } else if event_type == "divorce" && relationship_kind == "former-spouse" {
                "former-spouse"
            } else {
                "relationship"
            };
            relationship_slug(first_person, second_person, suffix)
        });
        let relationship_path = create_local_relationship(
            &world_root,
            &LocalRelationshipOptions {
                relationship_slug,
                title: None,
                relationship_kind,
                parent_role: None,
                source: first_person_id,
                target: second_person_id,
                sources: relationship_sources,
                force,
            },
        )?;
        println!(
            "created related relationship at {}",
            relationship_path.display()
        );
    }

    Ok(())
}

pub(crate) fn create_family_person_relationship(
    root: Option<PathBuf>,
    world: Option<&str>,
    new_person_slug: String,
    new_person_name: String,
    birth_date: Option<String>,
    birth_location: Option<String>,
    birth_latitude: Option<f64>,
    birth_longitude: Option<f64>,
    sex: Option<String>,
    create_birth_event: bool,
    relationship_slug: String,
    relationship_title: Option<String>,
    relationship_kind: String,
    parent_role: Option<String>,
    source: String,
    target: String,
    existing_people: Vec<String>,
    source_records: Vec<String>,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let world_root = resolve_world_root(root, world)?;
    ensure_people_exist(&world_root, &existing_people)?;
    create_local_person(
        &world_root,
        &LocalPersonOptions {
            person_slug: new_person_slug,
            person_name: new_person_name,
            sex,
            birth_date,
            birth_location,
            birth_latitude,
            birth_longitude,
            create_birth_event,
            force,
        },
    )?;
    let relationship_path = create_local_relationship(
        &world_root,
        &LocalRelationshipOptions {
            relationship_slug,
            title: relationship_title,
            relationship_kind,
            parent_role,
            source,
            target,
            sources: source_records,
            force,
        },
    )?;
    println!(
        "created person and relationship under {}; relationship at {}",
        world_root.display(),
        relationship_path.display()
    );
    Ok(())
}
