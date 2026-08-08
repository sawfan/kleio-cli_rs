use super::*;
use crate::cli_values::DoctorLevelArg;

pub(crate) fn print_workspace_next_steps(root: &Path, world_slug: &str, person_slug: &str) {
    println!(
        "next: grow the tree with `kleio-cli add-relative <slug> --relation parent|child|sibling|partner|spouse --of {person_slug} --person-name \"Name\" --root {}`",
        root.display()
    );
    println!(
        "add life events with `kleio-cli add-event <slug> --type residence|marriage|death|moment --person {person_slug} --root {}`",
        root.display()
    );
    println!(
        "then check your work with `kleio-cli summary --world {world_slug}` and `kleio-cli doctor --world {world_slug}`"
    );
    println!("build outputs with `kleio-cli build --world {world_slug}`");
}

pub(crate) fn print_media_check_report(
    world_root: &Path,
    report: &kleio::LocalMediaCheckReport,
    show_all: bool,
    redact: bool,
) {
    if redact {
        println!("Media/source file references for [redacted]");
    } else {
        println!("Media/source file references for {}", world_root.display());
    }
    println!("Referenced files: {}", report.referenced_files());
    println!("Present: {}", report.present_files());
    println!("Missing: {}", report.missing_files());

    if redact {
        println!(
            "\nDetails redacted. Run without --redact locally to inspect file paths and references."
        );
        return;
    }

    let references = report
        .references
        .iter()
        .filter(|reference| show_all || !reference.exists)
        .collect::<Vec<_>>();
    if references.is_empty() {
        if show_all {
            println!("\nReferences: none found");
        } else {
            println!("\nMissing: none found");
        }
    } else {
        println!("\n{}:", if show_all { "References" } else { "Missing" });
        for reference in references {
            let status = if reference.exists {
                "present"
            } else {
                "missing"
            };
            if show_all {
                println!("- {} [{status}]", reference.path);
            } else {
                println!("- {}", reference.path);
            }
            for source in &reference.referenced_by {
                println!("  referenced by {source}");
            }
        }
    }
}

pub(crate) fn diagnostic_kind_label(kind: kleio::LocalWorldDiagnosticKind) -> &'static str {
    match kind {
        kleio::LocalWorldDiagnosticKind::PersonMissingName => "person-missing-name",
        kleio::LocalWorldDiagnosticKind::PersonMissingBirthEvent => "person-missing-birth-event",
        kleio::LocalWorldDiagnosticKind::EventMissingParticipant => "event-missing-participant",
        kleio::LocalWorldDiagnosticKind::EventMissingTime => "event-missing-time",
        kleio::LocalWorldDiagnosticKind::EventMissingSource => "event-missing-source",
        kleio::LocalWorldDiagnosticKind::RelationshipMissingSource => "relationship-missing-source",
        kleio::LocalWorldDiagnosticKind::ReferencedFileMissing => "referenced-file-missing",
        kleio::LocalWorldDiagnosticKind::RecordUnexpectedPath => "record-unexpected-path",
        kleio::LocalWorldDiagnosticKind::PossibleDuplicatePerson => "possible-duplicate-person",
        kleio::LocalWorldDiagnosticKind::SuspiciousParentChildDirection => {
            "suspicious-parent-child-direction"
        }
    }
}

pub(crate) fn print_world_doctor_report(
    world_root: &Path,
    report: &kleio::LocalWorldDoctorReport,
    level: DoctorLevelArg,
    redact: bool,
) {
    if redact {
        println!("Checked world at [redacted]");
    } else {
        println!("Checked world at {}", world_root.display());
    }
    let diagnostics = report
        .diagnostics
        .iter()
        .filter(|diagnostic| level.includes(diagnostic))
        .collect::<Vec<_>>();
    if diagnostics.is_empty() {
        println!("No authoring warnings found for {:?} checks.", level);
        return;
    }

    println!("Warnings: {}", diagnostics.len());
    for diagnostic in diagnostics {
        if redact {
            println!(
                "- {} warning [details redacted]",
                diagnostic_kind_label(diagnostic.kind)
            );
        } else {
            println!("- {} ({})", diagnostic.message, diagnostic.path);
        }
    }
}

pub(crate) fn print_world_summary(
    world_root: &Path,
    summary: &kleio::LocalWorldSummary,
    redact: bool,
) {
    if redact {
        println!("World: [redacted]");
        println!("Path: [redacted]");
    } else {
        let title = summary
            .world_title
            .as_deref()
            .or(summary.world_id.as_deref())
            .unwrap_or("unknown world");
        println!("World: {title}");
        println!("Path: {}", world_root.display());
    }
    println!("People: {}", summary.counts.people);
    println!("Places: {}", summary.counts.places);
    println!("Organizations: {}", summary.counts.organizations);
    println!("Objects: {}", summary.counts.objects);
    println!("Concepts: {}", summary.counts.concepts);
    println!("Events: {}", summary.counts.events);
    for (event_type, count) in &summary.counts.events_by_type {
        println!("  {event_type}: {count}");
    }
    println!("Relationships: {}", summary.counts.relationships);
    println!("Sources: {}", summary.counts.sources);
    println!("Assertions: {}", summary.counts.assertions);
    println!("Collections: {}", summary.counts.collections);
    println!(
        "Views: {} timelines, {} trees, {} maps, {} calendars, {} visualizations",
        summary.counts.timeline_views,
        summary.counts.tree_views,
        summary.counts.map_views,
        summary.counts.calendar_views,
        summary.counts.visualization_views
    );

    if summary.warnings.is_empty() {
        println!("\nNeeds attention: none found");
    } else {
        println!("\nNeeds attention:");
        for warning in &summary.warnings {
            if redact {
                println!(
                    "- {} warning [details redacted]",
                    summary_warning_kind_label(warning.kind)
                );
            } else {
                println!("- {} ({})", warning.message, warning.path);
            }
        }
    }
}

pub(crate) fn summary_warning_kind_label(
    kind: kleio::LocalWorldSummaryWarningKind,
) -> &'static str {
    match kind {
        kleio::LocalWorldSummaryWarningKind::PersonMissingBirthEvent => {
            "person-missing-birth-event"
        }
        kleio::LocalWorldSummaryWarningKind::EventMissingTime => "event-missing-time",
        kleio::LocalWorldSummaryWarningKind::EventMissingSource => "event-missing-source",
        kleio::LocalWorldSummaryWarningKind::RelationshipMissingSource => {
            "relationship-missing-source"
        }
        kleio::LocalWorldSummaryWarningKind::RecordUnexpectedPath => "record-unexpected-path",
        kleio::LocalWorldSummaryWarningKind::ReferencedFileMissing => "referenced-file-missing",
        kleio::LocalWorldSummaryWarningKind::PossibleDuplicatePerson => "possible-duplicate-person",
        kleio::LocalWorldSummaryWarningKind::SuspiciousParentChildDirection => {
            "suspicious-parent-child-direction"
        }
    }
}
