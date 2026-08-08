use super::*;

pub(crate) fn assertion_slug_from_target_source(target: &str, source: &str) -> String {
    let target_slug = target
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let source_slug = source
        .strip_prefix("source:")
        .unwrap_or(source)
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    format!("{target_slug}-from-{source_slug}")
}

pub(crate) fn resolve_data_root(root: Option<PathBuf>) -> PathBuf {
    root.unwrap_or_else(default_data_root)
}

pub(crate) fn resolve_world_root(
    root: Option<PathBuf>,
    world: Option<&str>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let workspace_root = resolve_data_root(root);
    resolve_workspace_world_root(&workspace_root, world)
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
}

pub(crate) fn ensure_people_exist(
    world_root: &Path,
    people: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let person_ids = bundle
        .markdown_records
        .iter()
        .filter(|record| record.kind == "person")
        .map(|record| record.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    for person in people {
        let id = person_id(person);
        if !person_ids.contains(id.as_str()) {
            return Err(cli_error(format!(
                "person `{id}` does not exist in {}; create it first or check the slug",
                world_root.display()
            )));
        }
    }

    Ok(())
}

pub(crate) fn inferred_preferred_name(slug: &str) -> String {
    let slug = slug.strip_prefix("person:").unwrap_or(slug);
    let words = slug
        .split(['-', '_'])
        .filter(|part| !part.trim().is_empty())
        .map(title_case_slug_word)
        .collect::<Vec<_>>();

    match words.as_slice() {
        [] => slug.to_string(),
        [only] => only.clone(),
        [given, family] => format!("{given} {family}"),
        [given, .., family] => format!("{given} {family}"),
    }
}

pub(crate) fn title_case_slug_word(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().chain(chars).collect()
}
pub(crate) fn person_id(value: &str) -> String {
    if value.contains(':') {
        value.to_string()
    } else {
        format!("person:{value}")
    }
}

pub(crate) fn relationship_slug(source_slug: &str, target: &str, suffix: &str) -> String {
    let source_slug = person_slug_for_filename(source_slug);
    let target_slug = person_slug_for_filename(target);
    format!("{source_slug}-{target_slug}-{suffix}")
}

pub(crate) fn person_slug_for_filename(value: &str) -> String {
    value
        .strip_prefix("person:")
        .unwrap_or(value)
        .replace(':', "-")
}

pub(crate) fn relative_relationship_endpoints(
    relation: RelativeArg,
    existing_person: &str,
    relative_slug: &str,
) -> (String, String) {
    let existing_person_id = person_id(existing_person);
    let relative_id = person_id(relative_slug);
    match relation {
        RelativeArg::Parent | RelativeArg::StepParent => (relative_id, existing_person_id),
        RelativeArg::Child => (existing_person_id, relative_id),
        RelativeArg::Sibling | RelativeArg::Partner | RelativeArg::Spouse => {
            (existing_person_id, relative_id)
        }
    }
}
pub(crate) fn default_data_root() -> PathBuf {
    if let Some(path) = std::env::var_os("KLEIO_DATA_DIR").filter(|value| !value.is_empty()) {
        return PathBuf::from(path);
    }

    if let Some(path) = std::env::var_os("XDG_DATA_HOME").filter(|value| !value.is_empty()) {
        return Path::new(&path).join("kleio");
    }

    if let Some(home) = std::env::var_os("HOME").filter(|value| !value.is_empty()) {
        return Path::new(&home).join(".local/share/kleio");
    }

    PathBuf::from(".kleio-data")
}
