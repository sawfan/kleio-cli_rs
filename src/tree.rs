use super::*;
use crate::authoring::person_id;
use crate::error::cli_error;
use crate::listing::redacted_id;

pub(crate) fn print_tree_sketch(
    world_root: &Path,
    person: Option<&str>,
    depth: usize,
    redact: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let tree = compile_local_tree(world_root)?;
    let derived_kinship = compile_local_ecs(world_root)?
        .resources
        .relationships
        .derived_kinship;
    if tree.people.is_empty() {
        println!("Tree sketch: no people found.");
        return Ok(());
    }

    let focus = if let Some(person) = person {
        resolve_tree_person_id(&tree, person)
    } else {
        configured_tree_sketch_person(world_root)?
            .as_deref()
            .and_then(|person| resolve_tree_person_id(&tree, person))
    }
    .ok_or_else(|| {
        cli_error(match person {
            Some(person) => format!("person `{person}` was not found in the compiled tree"),
            None => "tree-sketch needs a focus person; pass `--person <person-slug>` or set `[root].entity` in a tree view / `[tree].main_person` in the world config".to_string(),
        })
    })?;
    let depth = depth.min(5);
    let redacted_names = redacted_person_names(&tree);

    println!(
        "Tree sketch for {}",
        tree_sketch_person_label(&tree, focus, redact, &redacted_names)
    );
    println!(
        "Focus: {}",
        tree_sketch_person_label(&tree, focus, redact, &redacted_names)
    );

    let mut parent_lines = ancestor_lines(&tree, focus, depth, redact, &redacted_names);
    parent_lines.extend(derived_step_parent_lines(
        &tree,
        &derived_kinship,
        focus,
        redact,
        &redacted_names,
    ));
    print_tree_group("Parents / ancestors", parent_lines);
    print_tree_group(
        "Siblings",
        sibling_lines(&tree, focus, redact, &redacted_names),
    );
    print_tree_group(
        "Spouses / partners",
        partner_lines(&tree, focus, redact, &redacted_names),
    );
    let mut child_lines = descendant_lines(&tree, focus, depth, redact, &redacted_names);
    child_lines.extend(derived_step_child_lines(
        &tree,
        &derived_kinship,
        focus,
        redact,
        &redacted_names,
    ));
    print_tree_group("Children / descendants", child_lines);

    Ok(())
}

pub(crate) fn print_tree_view_inspection(
    world_root: &Path,
    view: Option<&str>,
    person: Option<&str>,
    redact: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let tree = compile_local_tree_with_view(world_root, view)?;
    let view_document = tree_view_document(world_root, view)?;
    let focus = person
        .and_then(|person| resolve_tree_person_id(&tree, person))
        .or(tree.main_person)
        .or_else(|| tree.people.first().map(|person| person.id))
        .ok_or_else(|| cli_error("compiled tree view contains no people"))?;
    let redacted_names = redacted_person_names(&tree);

    println!("Tree view inspection");
    println!("  view: {}", view.unwrap_or("<default>"));
    if let Some(document) = &view_document {
        println!("  path: {}", document.path);
        println!(
            "  title: {}",
            document.title.as_deref().unwrap_or("<untitled>")
        );
    }
    println!(
        "  focus: {}",
        tree_sketch_person_label(&tree, focus, redact, &redacted_names)
    );

    if let Ok(raw_tree) = compile_local_tree(world_root) {
        print_projection_delta(&raw_tree, &tree);
        if let Ok(mut spec) = crate::run_build::tree_svg_view_spec_from_world(world_root, view) {
            spec.projection.focus_person = Some(focus);
            print_svg_render_report(&raw_tree, &spec, redact, &redacted_names);
        }
    }

    print_tree_view_settings(view_document.as_ref());

    println!("\nProjected graph:");
    println!("  people: {}", tree.people.len());
    println!("  relationships: {}", tree.relationships.len());
    println!("  events: {}", tree.events.len());

    let mut relationship_counts = BTreeMap::<String, usize>::new();
    for relationship in &tree.relationships {
        *relationship_counts
            .entry(relationship.kind.as_value().to_string())
            .or_default() += 1;
    }
    println!("\nRelationship kinds:");
    if relationship_counts.is_empty() {
        println!("  none");
    } else {
        for (kind, count) in relationship_counts {
            println!("  {kind}: {count}");
        }
    }

    let issues = inspect_tree_layout_issues(&tree, focus, redact, &redacted_names);
    println!("\nPotential layout/data issues:");
    if issues.is_empty() {
        println!("  none found");
    } else {
        for issue in issues {
            println!("  - {issue}");
        }
    }

    Ok(())
}

fn tree_view_document(
    world_root: &Path,
    view: Option<&str>,
) -> Result<Option<kleio::LocalTomlDocument>, Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let documents = bundle
        .toml_documents
        .iter()
        .filter(|document| document.kind.as_deref() == Some("tree-view"));

    if let Some(view) = view {
        let view_id = format!("tree:{view}");
        Ok(documents
            .into_iter()
            .find(|document| {
                document.id.as_deref() == Some(view_id.as_str())
                    || document.path == format!("views/trees/{view}.toml")
            })
            .cloned())
    } else {
        Ok(documents.into_iter().next().cloned())
    }
}

fn print_projection_delta(raw_tree: &kleio::TreeDocument, projected_tree: &kleio::TreeDocument) {
    let raw_people = raw_tree.people.len();
    let projected_people = projected_tree.people.len();
    let raw_relationships = raw_tree.relationships.len();
    let projected_relationships = projected_tree.relationships.len();
    println!("\nProjection delta:");
    println!(
        "  people: {projected_people} of {raw_people} included ({} excluded)",
        raw_people.saturating_sub(projected_people)
    );
    println!(
        "  relationships: {projected_relationships} of {raw_relationships} included ({} excluded)",
        raw_relationships.saturating_sub(projected_relationships)
    );

    let projected_people_ids = projected_tree
        .people
        .iter()
        .map(|person| person.id)
        .collect::<BTreeSet<_>>();
    let excluded_by_sex = raw_tree
        .people
        .iter()
        .filter(|person| !projected_people_ids.contains(&person.id))
        .fold(BTreeMap::<String, usize>::new(), |mut counts, person| {
            let label = match person.sex.as_ref() {
                Some(kleio::Sex::Male) => "male",
                Some(kleio::Sex::Female) => "female",
                Some(kleio::Sex::Other) => "other",
                Some(kleio::Sex::Unknown) | None => "unknown",
            };
            *counts.entry(label.to_string()).or_default() += 1;
            counts
        });
    if !excluded_by_sex.is_empty() {
        println!("  excluded people by sex:");
        for (label, count) in excluded_by_sex {
            println!("    {label}: {count}");
        }
    }

    let projected_relationship_ids = projected_tree
        .relationships
        .iter()
        .map(|relationship| relationship.id)
        .collect::<BTreeSet<_>>();
    let excluded_relationships = raw_tree
        .relationships
        .iter()
        .filter(|relationship| !projected_relationship_ids.contains(&relationship.id))
        .fold(
            BTreeMap::<String, usize>::new(),
            |mut counts, relationship| {
                *counts
                    .entry(relationship.kind.as_value().to_string())
                    .or_default() += 1;
                counts
            },
        );
    if !excluded_relationships.is_empty() {
        println!("  excluded relationships by kind:");
        for (kind, count) in excluded_relationships {
            println!("    {kind}: {count}");
        }
    }
}

fn print_svg_render_report(
    raw_tree: &kleio::TreeDocument,
    spec: &kleio_svg::TreeSvgViewSpec,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) {
    let report = inspect_tree_svg_view(raw_tree, spec);
    println!("\nSVG projection report:");
    println!("  projected_people: {}", report.projected_people);
    println!(
        "  projected_relationships: {}",
        report.projected_relationships
    );
    println!(
        "  parent_child_relationships: {}",
        report.parent_child_relationships
    );
    println!("  partner_relationships: {}", report.partner_relationships);

    if !report.oversized_parent_sets.is_empty() {
        println!("  oversized parent sets:");
        for (person_id, count) in report.oversized_parent_sets {
            println!(
                "    {}: {count} parent/guardian relationships",
                tree_sketch_person_label(raw_tree, person_id, redact, redacted_names)
            );
        }
    }
    if !report.multiple_current_partners.is_empty() {
        println!("  multiple current partners:");
        for (person_id, count) in report.multiple_current_partners {
            println!(
                "    {}: {count} current spouse/partner relationships",
                tree_sketch_person_label(raw_tree, person_id, redact, redacted_names)
            );
        }
    }
    if !report.repeated_ancestors.is_empty() {
        println!("  repeated ancestors:");
        for (person_id, count) in report.repeated_ancestors {
            println!(
                "    {}: {count} paths",
                tree_sketch_person_label(raw_tree, person_id, redact, redacted_names)
            );
        }
    }
}

fn print_tree_view_settings(document: Option<&kleio::LocalTomlDocument>) {
    println!("\nView settings:");
    let Some(document) = document else {
        println!("  tree view: <none; compiler defaults>");
        return;
    };

    if let Some(root) = document
        .data
        .get("root")
        .and_then(|root| root.get("entity"))
        .and_then(serde_json::Value::as_str)
    {
        println!("  root.entity: {root}");
    }

    let projection = document
        .data
        .get("projection")
        .or_else(|| document.data.get("filter"));
    print_json_section(
        "projection",
        projection,
        &[
            "relationship_kinds",
            "generations_up",
            "generations_down",
            "include_partners",
            "include_siblings",
            "include_unconnected",
        ],
    );
    print_json_section(
        "layout",
        document.data.get("layout"),
        &["algorithm", "orientation"],
    );
    print_json_section(
        "node",
        document
            .data
            .get("node")
            .or_else(|| document.data.get("display")),
        &[
            "content",
            "show_life_dates",
            "show_places",
            "show_sources",
            "show_relationship_labels",
            "show_person_ids",
            "show_sex",
        ],
    );
}

fn print_json_section(section_name: &str, section: Option<&serde_json::Value>, keys: &[&str]) {
    println!("  [{section_name}]");
    let Some(section) = section else {
        println!("    <not configured>");
        return;
    };
    let mut printed = false;
    for key in keys {
        if let Some(value) = section.get(*key) {
            println!("    {key}: {}", format_json_value(value));
            printed = true;
        }
    }
    if !printed {
        println!("    <no recognized fields>");
    }
}

fn format_json_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(value) => value.clone(),
        serde_json::Value::Array(values) => values
            .iter()
            .map(format_json_value)
            .collect::<Vec<_>>()
            .join(", "),
        other => other.to_string(),
    }
}

fn inspect_tree_layout_issues(
    tree: &kleio::TreeDocument,
    focus: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let mut issues = Vec::new();

    for person in &tree.people {
        let parents = tree
            .relationships
            .iter()
            .filter(|relationship| {
                relationship.kind.is_parent_child() && relationship.target == person.id
            })
            .collect::<Vec<_>>();
        if parents.len() > 2 {
            issues.push(format!(
                "{} has {} parent/guardian relationships",
                tree_sketch_person_label(tree, person.id, redact, redacted_names),
                parents.len()
            ));
        }

        let current_partners = tree
            .relationships
            .iter()
            .filter(|relationship| {
                matches!(
                    relationship.kind,
                    kleio::RelationshipKind::Spouse | kleio::RelationshipKind::Partner
                ) && (relationship.source == person.id || relationship.target == person.id)
            })
            .count();
        if current_partners > 1 {
            issues.push(format!(
                "{} has {current_partners} current spouse/partner relationships",
                tree_sketch_person_label(tree, person.id, redact, redacted_names)
            ));
        }
    }

    let mut ancestor_paths = BTreeMap::<kleio::PersonId, usize>::new();
    count_ancestor_paths(tree, focus, &mut BTreeSet::new(), &mut ancestor_paths);
    for (person_id, count) in ancestor_paths {
        if count > 1 {
            issues.push(format!(
                "{} appears through {count} ancestor paths (pedigree collapse or cousin marriage)",
                tree_sketch_person_label(tree, person_id, redact, redacted_names)
            ));
        }
    }

    issues.sort();
    issues.dedup();
    issues
}

fn count_ancestor_paths(
    tree: &kleio::TreeDocument,
    person_id: kleio::PersonId,
    path: &mut BTreeSet<kleio::PersonId>,
    counts: &mut BTreeMap<kleio::PersonId, usize>,
) {
    if !path.insert(person_id) {
        return;
    }
    for parent in tree
        .relationships
        .iter()
        .filter(|relationship| {
            relationship.kind.is_parent_child() && relationship.target == person_id
        })
        .map(|relationship| relationship.source)
    {
        *counts.entry(parent).or_default() += 1;
        count_ancestor_paths(tree, parent, path, counts);
    }
    path.remove(&person_id);
}

pub(crate) fn resolve_tree_person_id(
    tree: &kleio::TreeDocument,
    person: &str,
) -> Option<kleio::PersonId> {
    let source_ids = tree
        .people
        .iter()
        .filter_map(|candidate| {
            let source_id = candidate.source_record.as_ref()?.0.strip_prefix("local:")?;
            Some((source_id.to_string(), candidate.id))
        })
        .collect::<BTreeMap<_, _>>();

    source_ids
        .get(person)
        .copied()
        .or_else(|| source_ids.get(&person_id(person)).copied())
}

pub(crate) fn configured_tree_sketch_person(
    world_root: &Path,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let tree_view_root = bundle
        .toml_documents
        .iter()
        .filter(|document| document.kind.as_deref() == Some("tree-view"))
        .find_map(|document| {
            document
                .data
                .get("root")?
                .get("entity")?
                .as_str()
                .map(ToOwned::to_owned)
        });
    if tree_view_root.is_some() {
        return Ok(tree_view_root);
    }

    Ok(bundle
        .toml_documents
        .iter()
        .find(|document| {
            document.kind.as_deref() == Some("registry") || document.path == "world.toml"
        })
        .and_then(|document| document.data.get("tree"))
        .and_then(|tree| tree.get("main_person"))
        .and_then(|main_person| main_person.as_str())
        .map(ToOwned::to_owned))
}

pub(crate) fn redacted_person_names(
    tree: &kleio::TreeDocument,
) -> BTreeMap<kleio::PersonId, String> {
    tree.people
        .iter()
        .enumerate()
        .map(|(index, person)| (person.id, redacted_id("person", index + 1)))
        .collect()
}

pub(crate) fn tree_sketch_person_label(
    tree: &kleio::TreeDocument,
    person: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> String {
    if redact {
        redacted_names
            .get(&person)
            .cloned()
            .unwrap_or_else(|| "person:<redacted>".to_string())
    } else {
        tree.person_display_name(person)
            .map(str::to_string)
            .unwrap_or_else(|| format!("Person {}", person.0))
    }
}

pub(crate) fn relationship_line_label(relationship: &kleio::TreeRelationship) -> String {
    relationship.label.clone().unwrap_or_else(|| {
        relationship
            .kind
            .label_with_parent_role(relationship.parent_role)
    })
}

pub(crate) fn ancestor_lines(
    tree: &kleio::TreeDocument,
    focus: kleio::PersonId,
    depth: usize,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut visited = BTreeSet::from([focus]);
    collect_ancestor_lines(
        tree,
        focus,
        depth,
        1,
        redact,
        redacted_names,
        &mut visited,
        &mut lines,
    );
    lines
}

pub(crate) fn collect_ancestor_lines(
    tree: &kleio::TreeDocument,
    person: kleio::PersonId,
    remaining_depth: usize,
    level: usize,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
    visited: &mut BTreeSet<kleio::PersonId>,
    lines: &mut Vec<String>,
) {
    if remaining_depth == 0 {
        return;
    }

    for relationship in tree
        .relationships
        .iter()
        .filter(|relationship| relationship.kind.is_parent_child() && relationship.target == person)
    {
        let parent = relationship.source;
        let indent = "  ".repeat(level.saturating_sub(1));
        let label = tree_sketch_person_label(tree, parent, redact, redacted_names);
        lines.push(format!(
            "{indent}- {label} ({})",
            relationship_line_label(relationship)
        ));
        if visited.insert(parent) {
            collect_ancestor_lines(
                tree,
                parent,
                remaining_depth - 1,
                level + 1,
                redact,
                redacted_names,
                visited,
                lines,
            );
        }
    }
}

pub(crate) fn descendant_lines(
    tree: &kleio::TreeDocument,
    focus: kleio::PersonId,
    depth: usize,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut visited = BTreeSet::from([focus]);
    collect_descendant_lines(
        tree,
        focus,
        depth,
        1,
        redact,
        redacted_names,
        &mut visited,
        &mut lines,
    );
    lines
}

pub(crate) fn collect_descendant_lines(
    tree: &kleio::TreeDocument,
    person: kleio::PersonId,
    remaining_depth: usize,
    level: usize,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
    visited: &mut BTreeSet<kleio::PersonId>,
    lines: &mut Vec<String>,
) {
    if remaining_depth == 0 {
        return;
    }

    for relationship in tree
        .relationships
        .iter()
        .filter(|relationship| relationship.kind.is_parent_child() && relationship.source == person)
    {
        let child = relationship.target;
        let indent = "  ".repeat(level.saturating_sub(1));
        let label = tree_sketch_person_label(tree, child, redact, redacted_names);
        lines.push(format!(
            "{indent}- {label} ({})",
            relationship_line_label(relationship)
        ));
        if visited.insert(child) {
            collect_descendant_lines(
                tree,
                child,
                remaining_depth - 1,
                level + 1,
                redact,
                redacted_names,
                visited,
                lines,
            );
        }
    }
}

pub(crate) fn sibling_lines(
    tree: &kleio::TreeDocument,
    focus: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let parents = tree
        .relationships
        .iter()
        .filter(|relationship| relationship.kind.is_parent_child() && relationship.target == focus)
        .map(|relationship| relationship.source)
        .collect::<BTreeSet<_>>();
    let mut siblings = BTreeSet::new();
    for parent in parents {
        siblings.extend(
            tree.relationships
                .iter()
                .filter(|relationship| {
                    relationship.kind.is_parent_child()
                        && relationship.source == parent
                        && relationship.target != focus
                })
                .map(|relationship| relationship.target),
        );
    }

    for relationship in tree
        .relationships
        .iter()
        .filter(|relationship| relationship.kind == kleio::RelationshipKind::Sibling)
    {
        if relationship.source == focus {
            siblings.insert(relationship.target);
        } else if relationship.target == focus {
            siblings.insert(relationship.source);
        }
    }

    siblings
        .into_iter()
        .map(|sibling| {
            format!(
                "- {}",
                tree_sketch_person_label(tree, sibling, redact, redacted_names)
            )
        })
        .collect()
}

pub(crate) fn partner_lines(
    tree: &kleio::TreeDocument,
    focus: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    tree.relationships
        .iter()
        .filter_map(|relationship| {
            let kind = relationship.kind.as_value();
            if !matches!(kind, "spouse" | "partner" | "former-spouse") {
                return None;
            }
            let other = if relationship.source == focus {
                relationship.target
            } else if relationship.target == focus {
                relationship.source
            } else {
                return None;
            };
            Some(format!(
                "- {} ({})",
                tree_sketch_person_label(tree, other, redact, redacted_names),
                relationship_line_label(relationship)
            ))
        })
        .collect()
}

pub(crate) fn derived_step_parent_lines(
    tree: &kleio::TreeDocument,
    derived_kinship: &[kleio::LocalDerivedKinshipRelationship],
    focus: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let Some(focus_source_id) = tree_source_id(tree, focus) else {
        return Vec::new();
    };

    derived_kinship
        .iter()
        .filter(|relationship| {
            relationship.relationship_kind == "step-parent-child"
                && relationship.target == focus_source_id
        })
        .filter_map(|relationship| {
            let source = tree_person_by_source_id(tree, &relationship.source)?;
            Some(format!(
                "- {} ({})",
                tree_sketch_person_label(tree, source, redact, redacted_names),
                derived_step_parent_label(relationship)
            ))
        })
        .collect()
}

pub(crate) fn derived_step_child_lines(
    tree: &kleio::TreeDocument,
    derived_kinship: &[kleio::LocalDerivedKinshipRelationship],
    focus: kleio::PersonId,
    redact: bool,
    redacted_names: &BTreeMap<kleio::PersonId, String>,
) -> Vec<String> {
    let Some(focus_source_id) = tree_source_id(tree, focus) else {
        return Vec::new();
    };

    derived_kinship
        .iter()
        .filter(|relationship| {
            relationship.relationship_kind == "step-parent-child"
                && relationship.source == focus_source_id
        })
        .filter_map(|relationship| {
            let target = tree_person_by_source_id(tree, &relationship.target)?;
            Some(format!(
                "- {} ({})",
                tree_sketch_person_label(tree, target, redact, redacted_names),
                derived_step_child_label(relationship)
            ))
        })
        .collect()
}

pub(crate) fn derived_step_parent_label(
    relationship: &kleio::LocalDerivedKinshipRelationship,
) -> String {
    derived_period_label("inferred step-parent", relationship)
}

pub(crate) fn derived_step_child_label(
    relationship: &kleio::LocalDerivedKinshipRelationship,
) -> String {
    derived_period_label("inferred step-child", relationship)
}

pub(crate) fn derived_period_label(
    prefix: &str,
    relationship: &kleio::LocalDerivedKinshipRelationship,
) -> String {
    match (&relationship.valid_from, &relationship.valid_until) {
        (Some(from), Some(until)) => format!("{prefix}, {from} to {until}"),
        (Some(from), None) => format!("{prefix}, since {from}"),
        (None, Some(until)) => format!("{prefix}, until {until}"),
        (None, None) => prefix.to_string(),
    }
}

pub(crate) fn tree_source_id(
    tree: &kleio::TreeDocument,
    person: kleio::PersonId,
) -> Option<String> {
    tree.people
        .iter()
        .find(|candidate| candidate.id == person)?
        .source_record
        .as_ref()?
        .0
        .strip_prefix("local:")
        .map(ToOwned::to_owned)
}

pub(crate) fn tree_person_by_source_id(
    tree: &kleio::TreeDocument,
    source_id: &str,
) -> Option<kleio::PersonId> {
    tree.people.iter().find_map(|candidate| {
        let candidate_source_id = candidate.source_record.as_ref()?.0.strip_prefix("local:")?;
        (candidate_source_id == source_id).then_some(candidate.id)
    })
}

pub(crate) fn print_tree_group(label: &str, lines: Vec<String>) {
    println!("\n{label}:");
    if lines.is_empty() {
        println!("  none found");
    } else {
        for line in lines {
            println!("  - {line}");
        }
    }
}
