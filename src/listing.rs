use super::*;

pub(crate) fn local_record_name(record: &kleio::LocalMarkdownRecord) -> String {
    record
        .title
        .clone()
        .or_else(|| local_name_table(record, "preferred"))
        .or_else(|| local_name_table(record, "legal"))
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn local_name_table(record: &kleio::LocalMarkdownRecord, usage: &str) -> Option<String> {
    let table = record.attributes.get("names")?.get(usage)?;
    table
        .get("display")
        .and_then(|value| value.as_str())
        .or_else(|| table.get("full").and_then(|value| value.as_str()))
        .map(ToOwned::to_owned)
        .or_else(|| {
            let given = table.get("given").and_then(|given| given.as_str())?;
            let family = table.get("family").and_then(|family| family.as_str())?;
            Some(format!("{given} {family}"))
        })
}

pub(crate) fn local_record_type(record: &kleio::LocalMarkdownRecord) -> String {
    record
        .attributes
        .get("type")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| record.kind.clone())
}

pub(crate) fn local_record_matches_filter(
    record: &kleio::LocalMarkdownRecord,
    fields: &[&str],
    filter: Option<&str>,
) -> bool {
    let Some(filter) = filter else {
        return true;
    };
    let filter = filter.to_lowercase();
    let haystack = std::iter::once(record.id.as_str())
        .chain(std::iter::once(record.path.as_str()))
        .chain(fields.iter().copied())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    haystack.contains(&filter)
}

pub(crate) fn redacted_id(prefix: &str, index: usize) -> String {
    format!("{prefix}:<redacted-{index}>")
}

pub(crate) fn print_people(
    world_root: &Path,
    filter: Option<&str>,
    redact: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let mut index = 1;
    for record in bundle
        .markdown_records
        .iter()
        .filter(|record| record.kind == "person")
    {
        let name = local_record_name(record);
        if !local_record_matches_filter(record, &[&name], filter) {
            continue;
        }
        if redact {
            println!("{}	[redacted]	[redacted]", redacted_id("person", index));
            index += 1;
        } else {
            println!("{}	{}	{}", record.id, name, record.path);
        }
    }
    Ok(())
}

pub(crate) fn print_events(
    world_root: &Path,
    filter: Option<&str>,
    redact: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let mut index = 1;
    for record in bundle
        .markdown_records
        .iter()
        .filter(|record| record.kind == "event")
    {
        let title = local_record_name(record);
        let event_type = local_record_type(record);
        if !local_record_matches_filter(record, &[&title, &event_type], filter) {
            continue;
        }
        if redact {
            println!(
                "{}	{}	[redacted]	[redacted]",
                redacted_id("event", index),
                event_type
            );
            index += 1;
        } else {
            println!("{}	{}	{}	{}", record.id, event_type, title, record.path);
        }
    }
    Ok(())
}

pub(crate) fn print_sources(
    world_root: &Path,
    filter: Option<&str>,
    redact: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let mut index = 1;
    for record in bundle
        .markdown_records
        .iter()
        .filter(|record| record.path.starts_with("sources/"))
    {
        let title = local_record_name(record);
        if !local_record_matches_filter(record, &[&title, &record.kind], filter) {
            continue;
        }
        if redact {
            println!(
                "{}\t{}\t[redacted]\t[redacted]",
                redacted_id("source", index),
                record.kind
            );
            index += 1;
        } else {
            println!("{}\t{}\t{}\t{}", record.id, record.kind, title, record.path);
        }
    }
    Ok(())
}
