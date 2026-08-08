use super::*;
use crate::error::cli_error;

#[derive(Debug, Default)]
struct FileRedactor {
    tokens: BTreeMap<String, String>,
    next_person: usize,
    next_event: usize,
    next_source: usize,
    next_place: usize,
    next_relationship: usize,
    next_assertion: usize,
    next_other: usize,
}

impl FileRedactor {
    fn redact_text(&mut self, text: &str, redact_body: bool) -> String {
        let mut output = String::new();
        let mut in_frontmatter = false;
        let mut frontmatter_closed = false;
        let mut body_redacted = false;
        for line in text.lines() {
            if line.trim() == "+++" && !frontmatter_closed {
                in_frontmatter = !in_frontmatter;
                if !in_frontmatter {
                    frontmatter_closed = true;
                }
                output.push_str(line);
                output.push('\n');
                continue;
            }

            if redact_body && frontmatter_closed {
                if !body_redacted && !line.trim().is_empty() {
                    output.push('\n');
                    output.push_str("[body redacted]");
                    output.push('\n');
                    body_redacted = true;
                }
                continue;
            }

            if in_frontmatter || !frontmatter_closed {
                output.push_str(&self.redact_line(line));
            } else {
                output.push_str(line);
            }
            output.push('\n');
        }
        output
    }

    fn redact_line(&mut self, line: &str) -> String {
        if let Some(redacted) = self.redact_unquoted_sensitive_assignment(line) {
            return redacted;
        }

        let mut output = String::new();
        let mut chars = line.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '"' {
                let mut value = String::new();
                let mut escaped = false;
                for inner in chars.by_ref() {
                    if escaped {
                        value.push(inner);
                        escaped = false;
                    } else if inner == '\\' {
                        escaped = true;
                    } else if inner == '"' {
                        break;
                    } else {
                        value.push(inner);
                    }
                }
                output.push('"');
                output.push_str(&self.redact_value(&value));
                output.push('"');
            } else {
                output.push(ch);
            }
        }
        output
    }

    fn redact_unquoted_sensitive_assignment(&self, line: &str) -> Option<String> {
        let trimmed_start = line.trim_start();
        let indent_len = line.len() - trimmed_start.len();
        let (key, _) = trimmed_start.split_once('=')?;
        let key = key.trim();
        if !matches!(
            key,
            "latitude" | "longitude" | "lat" | "lng" | "birth_date" | "death_date" | "time"
        ) {
            return None;
        }
        let indent = &line[..indent_len];
        Some(format!("{indent}{key} = \"<date-or-number>\""))
    }

    fn redact_value(&mut self, value: &str) -> String {
        if value.trim().is_empty() || value.starts_with('#') {
            return value.to_string();
        }
        if matches!(
            value,
            "person"
                | "event"
                | "relationship"
                | "source"
                | "place"
                | "assertion"
                | "birth"
                | "death"
                | "residence"
                | "marriage"
                | "migration"
                | "immigration"
                | "emigration"
                | "observation"
                | "moment"
                | "event-support"
                | "medium"
                | "high"
                | "low"
                | "subject"
                | "parent"
                | "partner"
                | "spouse"
                | "sibling"
        ) {
            return value.to_string();
        }
        if looks_like_date_or_time(value) || value.parse::<f64>().is_ok() {
            return "<date-or-number>".to_string();
        }
        if let Some((prefix, _)) = value.split_once(':')
            && matches!(
                prefix,
                "person" | "event" | "source" | "place" | "relationship" | "assertion"
            )
        {
            return self.token_for(prefix, value);
        }
        if value.contains('/') || value.contains('.') {
            return self.token_for("path", value);
        }
        self.token_for("text", value)
    }

    fn token_for(&mut self, prefix: &str, value: &str) -> String {
        if let Some(token) = self.tokens.get(value) {
            return token.clone();
        }
        let (label, index) = match prefix {
            "person" => {
                self.next_person += 1;
                ("person", self.next_person)
            }
            "event" => {
                self.next_event += 1;
                ("event", self.next_event)
            }
            "source" => {
                self.next_source += 1;
                ("source", self.next_source)
            }
            "place" => {
                self.next_place += 1;
                ("place", self.next_place)
            }
            "relationship" => {
                self.next_relationship += 1;
                ("relationship", self.next_relationship)
            }
            "assertion" => {
                self.next_assertion += 1;
                ("assertion", self.next_assertion)
            }
            _ => {
                self.next_other += 1;
                ("redacted", self.next_other)
            }
        };
        let token = format!("{label}:<redacted-{index}>");
        self.tokens.insert(value.to_string(), token.clone());
        token
    }
}

pub(crate) fn looks_like_date_or_time(value: &str) -> bool {
    let value = value.trim();
    value.len() >= 4 && value.chars().take(4).all(|ch| ch.is_ascii_digit())
}

pub(crate) fn redact_file(
    path: &Path,
    redact_body: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    Ok(FileRedactor::default().redact_text(&text, redact_body))
}

pub(crate) fn redact_world_dump(
    world_root: &Path,
    redact_body: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut redactor = FileRedactor::default();
    let mut path_redactor = FileRedactor::default();
    let mut files = Vec::new();
    collect_redactable_world_files(world_root, world_root, &mut files)?;
    files.sort();

    let mut output = String::new();
    output.push_str("===== REDACTED KLEIO WORLD DUMP =====\n");
    output.push_str("Generated by `kleio-cli redact-world-dump`.\n");
    output.push_str("Skipped: build/, imports/, media/, hidden files/directories.\n");
    output.push_str(&format!("Markdown body text redacted: {redact_body}\n"));

    for relative_path in &files {
        let redacted_path = redact_relative_path(relative_path, &mut path_redactor);
        let source_path = world_root.join(relative_path);
        let text = std::fs::read_to_string(&source_path)?;
        output.push_str("\n===== ");
        output.push_str(&redacted_path.display().to_string());
        output.push_str(" =====\n");
        output.push_str(&redactor.redact_text(&text, redact_body));
    }

    Ok(output)
}

pub(crate) fn redact_world_tree(
    world_root: &Path,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    collect_redactable_world_files(world_root, world_root, &mut files)?;
    files.sort();
    let mut redactor = FileRedactor::default();
    Ok(files
        .iter()
        .map(|path| redact_relative_path(path, &mut redactor))
        .collect())
}

pub(crate) fn redact_world(
    world_root: &Path,
    out: &Path,
    redact_body: bool,
    force: bool,
) -> Result<usize, Box<dyn std::error::Error>> {
    if out.exists() {
        if !force {
            return Err(cli_error(format!(
                "output directory {} already exists; pass --force to replace it",
                out.display()
            )));
        }
        std::fs::remove_dir_all(out)?;
    }
    std::fs::create_dir_all(out)?;

    let mut redactor = FileRedactor::default();
    let mut files = Vec::new();
    collect_redactable_world_files(world_root, world_root, &mut files)?;
    files.sort();

    let mut path_redactor = FileRedactor::default();
    let redacted_paths = files
        .iter()
        .map(|relative_path| {
            let redacted_path = redact_relative_path(relative_path, &mut path_redactor);
            (relative_path.clone(), redacted_path)
        })
        .collect::<Vec<_>>();

    for (relative_path, redacted_path) in &redacted_paths {
        let source_path = world_root.join(relative_path);
        let output_path = out.join(redacted_path);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = std::fs::read_to_string(&source_path)?;
        std::fs::write(output_path, redactor.redact_text(&text, redact_body))?;
    }

    let readme_path = out.join("README.md");
    std::fs::write(readme_path, redacted_world_readme(files.len(), redact_body))?;

    Ok(files.len())
}

pub(crate) fn redacted_world_readme(file_count: usize, redact_body: bool) -> String {
    format!(
        r#"# Redacted Kleio world

This directory was generated by `kleio-cli redact-world`.

It contains redacted authored Markdown/TOML files for diagnostics and structural
review. References are redacted consistently within this output so the shape of
people, events, sources, and relationships can still be inspected.

## Included

- Authored `.md` and `.toml` files from the selected world.
- Directory categories such as `entities/`, `events/`, `relationships/`,
  `sources/`, `views/`, and `schemas/`.

## Skipped

- `build/` generated outputs.
- `imports/` raw import artifacts.
- `media/` files and blobs.
- Hidden files and directories.

## Redaction

- Redacted authored files: {file_count}
- Markdown body text redacted: {redact_body}
- `README.md` is added by the redaction command and is not counted above.
- Quoted TOML/frontmatter values are replaced with placeholders.
- Date-like and numeric string values are replaced with `<date-or-number>`.
- Output file names are redacted while directory categories are preserved.

## Review before sharing

Inspect this directory before sharing. Redaction is best-effort and intended for
diagnostics, not as a guarantee that the output is safe for public release.
"#
    )
}

pub(crate) fn redact_relative_path(path: &Path, redactor: &mut FileRedactor) -> PathBuf {
    let mut redacted = PathBuf::new();
    for component in path.components() {
        let value = component.as_os_str().to_string_lossy();
        if value.ends_with(".md") || value.ends_with(".toml") {
            let extension = Path::new(value.as_ref())
                .extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or("txt");
            let stem = Path::new(value.as_ref())
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("file");
            let token = redactor.token_for("path", stem);
            let safe_stem = token
                .chars()
                .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
                .collect::<String>()
                .trim_matches('-')
                .to_string();
            redacted.push(format!("{}.{}", safe_stem, extension));
        } else {
            redacted.push(value.as_ref());
        }
    }
    redacted
}

pub(crate) fn collect_redactable_world_files(
    world_root: &Path,
    dir: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if file_name.starts_with('.')
            || (file_type.is_dir() && matches!(file_name.as_ref(), "build" | "imports" | "media"))
        {
            continue;
        }

        if file_type.is_dir() {
            collect_redactable_world_files(world_root, &path, files)?;
        } else if file_type.is_file()
            && matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("md" | "toml")
            )
        {
            files.push(path.strip_prefix(world_root).unwrap_or(&path).to_path_buf());
        }
    }
    Ok(())
}
