use super::*;

pub(crate) fn person_has_birth_event(bundle: &kleio::LocalDataBundle, person_id: &str) -> bool {
    bundle.markdown_records.iter().any(|record| {
        record.kind == "event"
            && record
                .attributes
                .get("type")
                .and_then(|value| value.as_str())
                == Some("birth")
            && event_mentions_person(record, person_id)
    })
}

pub(crate) fn event_mentions_person(record: &kleio::LocalMarkdownRecord, person_id: &str) -> bool {
    record
        .attributes
        .get("subject")
        .and_then(|value| value.as_str())
        .is_some_and(|subject| person_id_matches(subject, person_id))
        || record
            .attributes
            .get("participants")
            .and_then(|value| value.as_array())
            .is_some_and(|participants| {
                participants.iter().any(|participant| {
                    participant
                        .as_str()
                        .is_some_and(|value| person_id_matches(value, person_id))
                        || participant
                            .as_object()
                            .and_then(|values| values.get("entity"))
                            .and_then(|value| value.as_str())
                            .is_some_and(|value| person_id_matches(value, person_id))
                })
            })
}

pub(crate) fn person_id_matches(value: &str, person_id: &str) -> bool {
    value == person_id || person_id.strip_prefix("person:") == Some(value)
}

pub(crate) fn relationship_kind(document: &kleio::LocalTomlDocument) -> Option<&str> {
    document
        .data
        .get("relationship")
        .and_then(|value| value.as_str())
        .or_else(|| {
            document
                .data
                .get("relationship_kind")
                .and_then(|value| value.as_str())
        })
        .or_else(|| {
            document
                .data
                .get("relation")
                .and_then(|value| value.as_str())
        })
}

pub(crate) fn person_parent_count(bundle: &kleio::LocalDataBundle, person_id: &str) -> usize {
    bundle
        .toml_documents
        .iter()
        .filter(|document| document.kind.as_deref() == Some("relationship"))
        .filter(|document| {
            relationship_kind(document).is_some_and(|kind| {
                matches!(
                    kind,
                    "biological-parent-child"
                        | "adoptive-parent-child"
                        | "foster-parent-child"
                        | "step-parent-child"
                        | "guardian-child"
                )
            })
        })
        .filter(|document| {
            document
                .data
                .get("target")
                .and_then(|value| value.as_str())
                .is_some_and(|target| person_id_matches(target, person_id))
        })
        .count()
}

pub(crate) fn person_has_partner(bundle: &kleio::LocalDataBundle, person_id: &str) -> bool {
    bundle
        .toml_documents
        .iter()
        .filter(|document| document.kind.as_deref() == Some("relationship"))
        .any(|document| {
            let partner_kind = relationship_kind(document)
                .is_some_and(|kind| matches!(kind, "partner" | "spouse" | "former-spouse"));
            if !partner_kind {
                return false;
            }
            let source_matches = document
                .data
                .get("source")
                .and_then(|value| value.as_str())
                .is_some_and(|source| person_id_matches(source, person_id));
            let target_matches = document
                .data
                .get("target")
                .and_then(|value| value.as_str())
                .is_some_and(|target| person_id_matches(target, person_id));
            source_matches || target_matches
        })
}

pub(crate) fn guide_subject<'a>(
    bundle: &'a kleio::LocalDataBundle,
    person: Option<&str>,
) -> Option<&'a kleio::LocalMarkdownRecord> {
    if let Some(person) = person {
        let id = person_id(person);
        return bundle
            .markdown_records
            .iter()
            .find(|record| record.kind == "person" && record.id == id);
    }

    let tree_root = bundle
        .toml_documents
        .iter()
        .find(|document| document.kind.as_deref() == Some("tree-view"))
        .and_then(|document| document.data.get("root"))
        .and_then(|root| root.get("entity"))
        .and_then(|value| value.as_str())
        .map(person_id);

    tree_root
        .as_deref()
        .and_then(|id| {
            bundle
                .markdown_records
                .iter()
                .find(|record| record.kind == "person" && record.id == id)
        })
        .or_else(|| {
            bundle
                .markdown_records
                .iter()
                .find(|record| record.kind == "person")
        })
}

pub(crate) fn print_authoring_guide(
    world_root: &Path,
    person: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let Some(subject) = guide_subject(&bundle, person) else {
        println!("No people found yet.");
        println!(
            "Start with `kleio-cli new-person <slug> --person-name \"Name\"` or `kleio-cli init-workspace --person-slug <slug> --person-name \"Name\"`."
        );
        return Ok(());
    };

    let subject_slug = subject.id.strip_prefix("person:").unwrap_or(&subject.id);
    println!("Family tree guide for {}", subject.id);
    println!("World: {}", world_root.display());
    println!();

    if !person_has_birth_event(&bundle, &subject.id) {
        println!("1. Add birth data for this person:");
        println!(
            "   kleio-cli add-event birth-{subject_slug} --type birth --person {subject_slug} --time YYYY-MM-DD --location \"Birthplace\""
        );
        println!();
    }

    let parent_count = person_parent_count(&bundle, &subject.id);
    if parent_count < 2 {
        println!("2. Add parent records:");
        println!(
            "   kleio-cli add-relative mother-slug --relation parent --of {subject_slug} --person-name \"Mother Name\""
        );
        println!(
            "   kleio-cli add-relative father-slug --relation parent --of {subject_slug} --person-name \"Father Name\""
        );
        println!(
            "   Tip: add grandparents by changing --of to a parent slug, for example `--of mother-slug`."
        );
        println!();
    }

    if !person_has_partner(&bundle, &subject.id) {
        println!("3. Optionally add a partner or spouse:");
        println!(
            "   kleio-cli add-relative partner-slug --relation partner --of {subject_slug} --person-name \"Partner Name\" --no-birth-event"
        );
        println!();
    }

    println!("4. Add other life events when ready:");
    println!(
        "   kleio-cli add-event {subject_slug}-residence --type residence --person {subject_slug} --time YYYY --location \"Place\""
    );
    println!(
        "   kleio-cli add-event {subject_slug}-moment --type moment --person {subject_slug} --title \"Short label\" --time YYYY-MM-DD"
    );
    println!();
    println!("5. Check progress:");
    println!("   kleio-cli summary --redact");
    println!("   kleio-cli doctor --redact");
    Ok(())
}
