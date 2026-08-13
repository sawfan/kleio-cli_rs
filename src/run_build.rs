#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_specs_keep_layout_separate_from_projection() {
        for layout in TreeSvgLayout::ALL {
            let spec = fixture_tree_svg_spec_for_layout(layout);

            assert_eq!(spec.geometry.layout, layout);
            assert_eq!(
                spec.projection,
                kleio_svg::TreeSvgProjectionOptions::default()
            );
        }
    }
}

use super::*;
use crate::authoring::*;
use crate::cli::Command;
use crate::error::cli_error;
use kleio::{ParentRole, RelationshipKind};

pub(crate) fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Compile(crate::cli_build::CompileArgs { root, world, out }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let out = out.unwrap_or(build_paths.compiled_json);
            let bundle = write_local_data_json(&world_root, &out)?;
            println!(
                "wrote {} records and {} TOML documents to {}",
                bundle.markdown_records.len(),
                bundle.toml_documents.len(),
                out.display()
            );
        }
        Command::CompileEcs(crate::cli_build::CompileEcsArgs { root, world, out }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let out = out.unwrap_or(build_paths.ecs_json);
            let bundle = write_local_ecs_json(&world_root, &out)?;
            println!(
                "wrote ECS bundle with {} entities to {}",
                bundle.entities.len(),
                out.display()
            );
        }
        Command::Build(crate::cli_build::BuildArgs {
            root,
            world,
            timeline_view,
            tree_view,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let output = build_local_world_with_options(
                &world_root,
                &LocalWorldBuildOptions {
                    timeline_view: timeline_view.as_deref(),
                    tree_view: tree_view.as_deref(),
                },
            )?;
            println!(
                "built world at {}: {} Markdown records, {} TOML documents, {} ECS entities",
                world_root.display(),
                output.markdown_records,
                output.toml_documents,
                output.ecs_entities
            );
            if let (Some(path), Some(events), Some(collections)) = (
                &output.timeline_json_path,
                output.timeline_events,
                output.timeline_collections,
            ) {
                println!(
                    "wrote timeline projection with {events} events and {collections} collections to {}",
                    path.display()
                );
            }
            if let (Some(path), Some(people), Some(events), Some(relationships)) = (
                &output.tree_json_path,
                output.tree_people,
                output.tree_events,
                output.tree_relationships,
            ) {
                println!(
                    "wrote tree projection with {people} people, {events} events, and {relationships} relationships to {}",
                    path.display()
                );
            }
        }
        Command::CompileTimeline(crate::cli_build::CompileTimelineArgs {
            root,
            world,
            view,
            out,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let build_dir = build_paths
                .compiled_json
                .parent()
                .unwrap_or(world_root.as_path());
            let output_slug = view.as_deref().unwrap_or("timeline");
            let out = out.unwrap_or_else(|| build_dir.join(format!("{output_slug}.timeline.json")));
            let timeline = write_local_timeline_json(&world_root, view.as_deref(), &out)?;
            println!(
                "wrote timeline projection with {} events and {} collections to {}",
                timeline.events.len(),
                timeline.collections.len(),
                out.display()
            );
        }
        Command::CompileTree(crate::cli_build::CompileTreeArgs {
            root,
            world,
            view,
            out,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let build_dir = build_paths
                .compiled_json
                .parent()
                .unwrap_or(world_root.as_path());
            let out = out.unwrap_or_else(|| {
                view.as_deref()
                    .map(|view| build_dir.join(format!("{view}.tree.json")))
                    .unwrap_or_else(|| build_dir.join("kleio-tree.json"))
            });
            let tree = write_local_tree_json_with_view(&world_root, view.as_deref(), &out)?;
            println!(
                "wrote tree with {} people, {} events, and {} relationships to {}",
                tree.people.len(),
                tree.events.len(),
                tree.relationships.len(),
                out.display()
            );
        }
        Command::CompileTrees(crate::cli_build::CompileTreesArgs {
            root,
            world,
            view,
            person,
            out,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let build_dir = build_paths
                .compiled_json
                .parent()
                .unwrap_or(world_root.as_path());
            let out = out.unwrap_or_else(|| {
                view.as_deref()
                    .map(|view| build_dir.join(format!("{view}.trees.json")))
                    .unwrap_or_else(|| build_dir.join("kleio-trees.json"))
            });
            let mut document =
                compile_local_trees_document_with_view(&world_root, view.as_deref())?;
            if let Some(person) = person {
                let Some(tree) = document.trees.first_mut() else {
                    return Err(cli_error("compiled trees document contained no trees"));
                };
                tree.main_person = Some(resolve_export_focus_person(tree, &person)?);
                document.main_tree_id = tree.metadata.id.0.clone();
            }
            let json = serde_json::to_string_pretty(&document)?;
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&out, format!("{json}\n"))?;
            let people = document
                .trees
                .iter()
                .map(|tree| tree.people.len())
                .sum::<usize>();
            let events = document
                .trees
                .iter()
                .map(|tree| tree.events.len())
                .sum::<usize>();
            let relationships = document
                .trees
                .iter()
                .map(|tree| tree.relationships.len())
                .sum::<usize>();
            println!(
                "wrote browser trees document with {} tree(s), {people} people, {events} events, and {relationships} relationships to {}",
                document.trees.len(),
                out.display()
            );
        }
        Command::ExportTreeSvg(crate::cli_build::ExportTreeSvgArgs {
            root,
            world,
            view,
            person,
            layout,
            out,
            png,
            png_width,
            png_height,
            svg: svg_overrides,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let build_dir = build_paths
                .compiled_json
                .parent()
                .unwrap_or(world_root.as_path());
            let out = out.unwrap_or_else(|| {
                view.as_deref()
                    .map(|view| build_dir.join(format!("{view}.tree.svg")))
                    .unwrap_or_else(|| build_dir.join("kleio-tree.svg"))
            });
            let tree = compile_local_tree_with_view(&world_root, view.as_deref())?;
            let mut spec = tree_svg_view_spec_from_world(&world_root, view.as_deref())?;
            let focus = match person {
                Some(person) => Some(resolve_export_focus_person(&tree, &person)?),
                None => tree.main_person,
            };
            spec.projection.focus_person = focus;
            if let Some(layout) = layout {
                spec.geometry.layout = parse_tree_svg_layout(&layout)?;
            }
            apply_tree_svg_overrides(&mut spec, &svg_overrides)?;
            let svg = render_tree_svg_view(&tree, &spec);
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&out, &svg)?;
            if png {
                let png_path = out.with_extension("png");
                let bytes = svg_to_png_bytes(&svg, png_width, png_height)
                    .map_err(|message| cli_error(format!("could not render PNG: {message}")))?;
                fs::write(&png_path, bytes)?;
                println!("wrote {}", png_path.display());
            }
            println!(
                "wrote tree SVG with {} people and {} relationships to {}",
                tree.people.len(),
                tree.relationships.len(),
                out.display()
            );
        }
        Command::ExportTreeLayouts(crate::cli_build::ExportTreeLayoutsArgs {
            root,
            world,
            view,
            person,
            out_dir,
            png,
            png_width,
            png_height,
            svg: svg_overrides,
        }) => {
            let world_root = resolve_world_root(root, world.as_deref())?;
            let build_paths = resolve_world_build_paths(&world_root)?;
            let build_dir = build_paths
                .compiled_json
                .parent()
                .unwrap_or(world_root.as_path());
            let out_dir = out_dir.unwrap_or_else(|| build_dir.join("tree-layouts"));
            fs::create_dir_all(&out_dir)?;
            let tree = compile_local_tree_with_view(&world_root, view.as_deref())?;
            let base_spec = tree_svg_view_spec_from_world(&world_root, view.as_deref())?;
            let focus = match person {
                Some(person) => Some(resolve_export_focus_person(&tree, &person)?),
                None => tree.main_person,
            };
            let label = view.as_deref().unwrap_or("kleio-tree");

            for layout in TreeSvgLayout::ALL {
                let mut spec = base_spec.clone();
                spec.geometry.layout = layout;
                spec.projection.focus_person = focus;
                apply_tree_svg_overrides(&mut spec, &svg_overrides);
                spec.node_content.show_relationship_labels = true;
                spec.title = Some(format!("{} — {}", tree.metadata.title, layout.label()));
                let svg = render_tree_svg_view(&tree, &spec);
                let svg_path = out_dir.join(format!("{label}.{}.svg", layout.slug()));
                fs::write(&svg_path, &svg)?;
                println!("wrote {}", svg_path.display());

                if png {
                    let png_path = out_dir.join(format!("{label}.{}.png", layout.slug()));
                    let bytes = svg_to_png_bytes(&svg, png_width, png_height)
                        .map_err(|message| cli_error(format!("could not render PNG: {message}")))?;
                    fs::write(&png_path, bytes)?;
                    println!("wrote {}", png_path.display());
                }
            }
        }
        Command::ExportTreeFixtures(crate::cli_build::ExportTreeFixturesArgs {
            out_dir,
            png,
            png_width,
            png_height,
            contact_sheet,
            contact_cell_width,
            contact_cell_height,
            standalone_snapshots,
            standalone_width,
            standalone_height,
            svg: svg_overrides,
        }) => {
            let out_dir = out_dir.unwrap_or_else(|| PathBuf::from("target/kleio-tree-fixtures"));
            fs::create_dir_all(&out_dir)?;
            let fixtures = tree_svg_fixture_documents();
            let mut rendered_svgs = Vec::<FixtureRender>::new();
            let mut manifest = String::from(
                "# Kleio tree SVG fixtures\n\nThese files are generated by `kleio-cli export-tree-fixtures` for visual inspection of tree SVG rendering behavior.\n\n",
            );
            for (slug, tree) in &fixtures {
                let description = tree_svg_fixture_description(slug);
                manifest.push_str(&format!("## {slug}\n\n{description}\n\n"));
                manifest.push_str("Generated files:\n\n");
                for layout in TreeSvgLayout::ALL {
                    let mut spec = fixture_tree_svg_spec_for_layout(layout);
                    apply_tree_svg_overrides(&mut spec, &svg_overrides);
                    spec.title = Some(format!("{} — {}", tree.metadata.title, layout.label()));
                    let svg = if layout == TreeSvgLayout::RelationshipPath {
                        let mut options = TreeSvgOptions::from(&spec);
                        if let Some((from, to)) = fixture_relationship_pair(tree, slug) {
                            options.relationship_from = Some(from);
                            options.relationship_to = Some(to);
                            options.root = Some(from);
                        }
                        render_tree_svg(tree, &options)
                    } else {
                        render_tree_svg_view(tree, &spec)
                    };
                    let svg_filename = format!("{slug}.{}.svg", layout.slug());
                    let svg_path = out_dir.join(&svg_filename);
                    fs::write(&svg_path, &svg)?;
                    rendered_svgs.push(FixtureRender {
                        fixture_slug: slug,
                        layout,
                        svg: svg.clone(),
                    });
                    println!("wrote {}", svg_path.display());
                    manifest.push_str(&format!("- [{}]({svg_filename})", layout.label()));
                    if png {
                        let png_filename = format!("{slug}.{}.png", layout.slug());
                        let png_path = out_dir.join(&png_filename);
                        let bytes =
                            svg_to_png_bytes(&svg, png_width, png_height).map_err(|message| {
                                cli_error(format!("could not render fixture PNG: {message}"))
                            })?;
                        fs::write(&png_path, bytes)?;
                        println!("wrote {}", png_path.display());
                        manifest.push_str(&format!(" / [PNG]({png_filename})"));
                    }
                    manifest.push('\n');
                }
                manifest.push('\n');
            }
            if standalone_snapshots {
                render_standalone_tree_svg_snapshots(
                    &fixtures,
                    &out_dir,
                    standalone_width as f32,
                    standalone_height as f32,
                    &svg_overrides,
                    &mut manifest,
                )?;
            }
            if contact_sheet {
                let contact_svg = render_fixture_contact_sheet(
                    &fixtures,
                    &rendered_svgs,
                    contact_cell_width as f32,
                    contact_cell_height as f32,
                );
                let contact_svg_path = out_dir.join("contact-sheet.svg");
                fs::write(&contact_svg_path, &contact_svg)?;
                println!("wrote {}", contact_svg_path.display());
                manifest.push_str("## Contact sheet\n\n- [SVG](contact-sheet.svg)");
                if png {
                    let contact_png_path = out_dir.join("contact-sheet.png");
                    let width = contact_sheet_width(&fixtures, contact_cell_width);
                    let height = contact_sheet_height(&fixtures, contact_cell_height);
                    let bytes =
                        svg_to_png_bytes(&contact_svg, width, height).map_err(|message| {
                            cli_error(format!("could not render contact-sheet PNG: {message}"))
                        })?;
                    fs::write(&contact_png_path, bytes)?;
                    println!("wrote {}", contact_png_path.display());
                    manifest.push_str(" / [PNG](contact-sheet.png)");
                }
                manifest.push_str("\n\n");
            }
            let manifest_path = out_dir.join("index.md");
            fs::write(&manifest_path, manifest)?;
            println!("wrote {}", manifest_path.display());
        }
        _ => unreachable!("command routed to the wrong handler"),
    }
    Ok(())
}

fn parse_tree_svg_layout(value: &str) -> Result<TreeSvgLayout, Box<dyn std::error::Error>> {
    match value.trim() {
        "hourglass" => Ok(TreeSvgLayout::Hourglass),
        "descendants" | "top-down" => Ok(TreeSvgLayout::Descendants),
        "ancestors" | "bottom-up" => Ok(TreeSvgLayout::Ancestors),
        "radial" | "radial-generations" => Ok(TreeSvgLayout::RadialGenerations),
        "fan" => Ok(TreeSvgLayout::Fan),
        "bow-tie" | "bowtie" => Ok(TreeSvgLayout::BowTie),
        "generational" | "generation-bands" => Ok(TreeSvgLayout::Generational),
        "relationship-path" | "relationship" => Ok(TreeSvgLayout::RelationshipPath),
        "network" | "kinship-network" => Ok(TreeSvgLayout::Network),
        "outline" | "indented-descendants" => Ok(TreeSvgLayout::Outline),
        other => Err(cli_error(format!(
            "unknown tree SVG layout `{other}`; expected hourglass, descendants, ancestors, radial, fan, bow-tie, generational, relationship-path, network, or outline"
        ))),
    }
}

struct FixtureRender {
    fixture_slug: &'static str,
    layout: TreeSvgLayout,
    svg: String,
}

fn render_standalone_tree_svg_snapshots(
    fixtures: &[(&'static str, kleio::TreeDocument)],
    out_dir: &Path,
    width: f32,
    height: f32,
    overrides: &crate::cli_build::TreeSvgOverrideArgs,
    manifest: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let snapshot_dir = out_dir.join("standalone");
    fs::create_dir_all(&snapshot_dir)?;
    manifest.push_str("## Standalone diagnostic snapshots\n\n");
    for (fixture_slug, layout) in standalone_tree_svg_snapshot_cases() {
        let Some((_, tree)) = fixtures.iter().find(|(slug, _)| *slug == fixture_slug) else {
            continue;
        };
        let mut spec = fixture_tree_svg_spec_for_layout(layout);
        spec.width = Some(width);
        spec.height = Some(height);
        apply_tree_svg_overrides(&mut spec, overrides)?;
        spec.title = Some(format!(
            "{} — {} standalone",
            tree.metadata.title,
            layout.label()
        ));
        let svg = if layout == TreeSvgLayout::RelationshipPath {
            let mut options = TreeSvgOptions::from(&spec);
            if let Some((from, to)) = fixture_relationship_pair(tree, fixture_slug) {
                options.relationship_from = Some(from);
                options.relationship_to = Some(to);
                options.root = Some(from);
            }
            render_tree_svg(tree, &options)
        } else {
            render_tree_svg_view(tree, &spec)
        };
        let filename = format!("{fixture_slug}__{}.svg", layout.slug());
        let path = snapshot_dir.join(&filename);
        fs::write(&path, svg)?;
        println!("wrote {}", path.display());
        manifest.push_str(&format!(
            "- [Standalone {fixture_slug} / {}](standalone/{filename})\n",
            layout.label()
        ));
    }
    manifest.push('\n');
    Ok(())
}

fn standalone_tree_svg_snapshot_cases() -> Vec<(&'static str, TreeSvgLayout)> {
    vec![
        ("nuclear-family", TreeSvgLayout::RadialGenerations),
        ("second-marriage", TreeSvgLayout::RadialGenerations),
        ("collateral-family", TreeSvgLayout::RelationshipPath),
        ("nuclear-family", TreeSvgLayout::Network),
        ("second-marriage", TreeSvgLayout::Network),
        ("adoption-step-family", TreeSvgLayout::Network),
        ("pedigree-collapse", TreeSvgLayout::Network),
        ("collateral-family", TreeSvgLayout::Network),
        ("three-generation-pedigree", TreeSvgLayout::Fan),
        ("three-generation-pedigree", TreeSvgLayout::BowTie),
        ("collateral-family", TreeSvgLayout::Generational),
        ("focus-ancestors-descendants", TreeSvgLayout::Outline),
    ]
}

fn render_fixture_contact_sheet(
    fixtures: &[(&'static str, kleio::TreeDocument)],
    rendered_svgs: &[FixtureRender],
    cell_w: f32,
    cell_h: f32,
) -> String {
    let left_header_w = 220.0;
    let top_header_h = 82.0;
    let padding = 24.0;
    let width = left_header_w + cell_w * TreeSvgLayout::ALL.len() as f32 + padding * 2.0;
    let height = top_header_h + cell_h * fixtures.len() as f32 + padding * 2.0;
    let mut out = String::new();
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\n",
        width.round() as u32,
        height.round() as u32,
        width.round() as u32,
        height.round() as u32
    ));
    out.push_str("  <title>Kleio tree SVG fixture contact sheet</title>\n");
    out.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#f8fafc\"/>\n");
    out.push_str("  <style>text{font-family:Inter,ui-sans-serif,system-ui,sans-serif}.head{font-size:18px;font-weight:700;fill:#0f172a}.label{font-size:14px;font-weight:650;fill:#334155}.small{font-size:11px;fill:#64748b}.cell{fill:#fff;stroke:#cbd5e1;stroke-width:1}</style>\n");
    out.push_str(
        "  <text class=\"head\" x=\"24\" y=\"36\">Kleio tree SVG fixture layouts</text>\n",
    );
    out.push_str("  <text class=\"small\" x=\"24\" y=\"56\">Rows are fixtures; columns are layout algorithms. Use this image to review broad rendering behavior.</text>\n");

    for (column, layout) in TreeSvgLayout::ALL.iter().enumerate() {
        let x = padding + left_header_w + column as f32 * cell_w + cell_w / 2.0;
        out.push_str(&format!(
            "  <text class=\"label\" text-anchor=\"middle\" x=\"{}\" y=\"{}\">{}</text>\n",
            fmt_cli_num(x),
            fmt_cli_num(padding + top_header_h - 20.0),
            escape_cli_xml(layout.label())
        ));
    }

    for (row, (slug, _tree)) in fixtures.iter().enumerate() {
        let y = padding + top_header_h + row as f32 * cell_h;
        out.push_str(&format!(
            "  <text class=\"label\" x=\"{}\" y=\"{}\">{}</text>\n",
            fmt_cli_num(padding),
            fmt_cli_num(y + 24.0),
            escape_cli_xml(slug)
        ));
        out.push_str(&format!(
            "  <text class=\"small\" x=\"{}\" y=\"{}\">{}</text>\n",
            fmt_cli_num(padding),
            fmt_cli_num(y + 42.0),
            escape_cli_xml(tree_svg_fixture_short_description(slug))
        ));
        for (column, layout) in TreeSvgLayout::ALL.iter().enumerate() {
            let x = padding + left_header_w + column as f32 * cell_w;
            let cell_x = x + 8.0;
            let cell_y = y + 8.0;
            let cell_inner_w = cell_w - 16.0;
            let cell_inner_h = cell_h - 16.0;
            out.push_str(&format!(
                "  <rect class=\"cell\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"10\"/>\n",
                fmt_cli_num(cell_x),
                fmt_cli_num(cell_y),
                fmt_cli_num(cell_inner_w),
                fmt_cli_num(cell_inner_h)
            ));
            if let Some(rendered) = rendered_svgs
                .iter()
                .find(|rendered| rendered.fixture_slug == *slug && rendered.layout == *layout)
            {
                out.push_str(&embedded_svg_fragment(
                    &rendered.svg,
                    cell_x,
                    cell_y,
                    cell_inner_w,
                    cell_inner_h,
                ));
            }
        }
    }

    out.push_str("</svg>\n");
    out
}

fn embedded_svg_fragment(svg: &str, x: f32, y: f32, width: f32, height: f32) -> String {
    let view_box = svg
        .split("viewBox=\"")
        .nth(1)
        .and_then(|rest| rest.split('\"').next())
        .unwrap_or("0 0 100 100");
    let body = svg
        .split_once('>')
        .map(|(_, rest)| rest.trim_end_matches("</svg>\n").trim_end_matches("</svg>"))
        .unwrap_or(svg);
    format!(
        "  <svg x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" viewBox=\"{}\" preserveAspectRatio=\"xMidYMid meet\"><g><rect width=\"100%\" height=\"100%\" fill=\"#f8fafc\"/>{}</g></svg>\n",
        fmt_cli_num(x),
        fmt_cli_num(y),
        fmt_cli_num(width),
        fmt_cli_num(height),
        escape_cli_xml(view_box),
        body
    )
}

fn contact_sheet_width(_fixtures: &[(&'static str, kleio::TreeDocument)], cell_width: u32) -> u32 {
    220 + cell_width * TreeSvgLayout::ALL.len() as u32 + 48
}

fn contact_sheet_height(fixtures: &[(&'static str, kleio::TreeDocument)], cell_height: u32) -> u32 {
    82 + cell_height * fixtures.len() as u32 + 48
}

fn tree_svg_fixture_short_description(slug: &str) -> &'static str {
    match slug {
        "nuclear-family" => "couple + siblings",
        "three-generation-pedigree" => "parents + grandparents",
        "second-marriage" => "current/former partners",
        "single-parent" => "one parent",
        "adoption-step-family" => "complex parent set",
        "focus-ancestors-descendants" => "ancestors + descendants",
        "pedigree-collapse" => "shared ancestor",
        "collateral-family" => "extended collateral kin",
        _ => "fixture",
    }
}

fn fixture_tree_svg_spec_for_layout(layout: TreeSvgLayout) -> TreeSvgViewSpec {
    let mut spec = TreeSvgViewSpec::default();
    spec.geometry.layout = layout;
    spec.node_content.show_focus_marker = true;
    spec
}

fn escape_cli_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn fmt_cli_num(value: f32) -> String {
    let rounded = (value * 100.0).round() / 100.0;
    if rounded.fract().abs() < 0.001 {
        format!("{}", rounded as i64)
    } else {
        format!("{rounded:.2}")
    }
}

fn apply_tree_svg_overrides(
    spec: &mut TreeSvgViewSpec,
    overrides: &crate::cli_build::TreeSvgOverrideArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(preset) = overrides.preset.as_deref() {
        apply_tree_svg_preset(spec, preset)?;
    }
    if let Some(value) = overrides.generations_up {
        spec.projection.generations_up = Some(value);
    }
    if let Some(value) = overrides.generations_down {
        spec.projection.generations_down = Some(value);
    }
    if overrides.include_siblings {
        spec.projection.include_siblings = true;
    }
    if overrides.include_unconnected {
        spec.projection.include_unconnected = true;
    }
    if overrides.no_partners {
        spec.projection.include_partners = false;
    }
    if let Some(value) = overrides.margin {
        spec.geometry.margin = value;
    }
    if let Some(value) = overrides.node_width {
        spec.geometry.node_width = value;
    }
    if let Some(value) = overrides.node_height {
        spec.geometry.node_height = value;
    }
    if let Some(value) = overrides.x_gap {
        spec.geometry.x_gap = value;
    }
    if let Some(value) = overrides.y_gap {
        spec.geometry.y_gap = value;
    }
    if let Some(value) = overrides.radial_gap {
        spec.geometry.radial_gap = value;
    }
    if overrides.show_person_ids {
        spec.node_content.show_person_ids = true;
    }
    if overrides.show_sex {
        spec.node_content.show_sex = true;
    }
    if overrides.show_relationship_labels {
        spec.node_content.show_relationship_labels = true;
    }
    Ok(())
}

fn apply_tree_svg_preset(
    spec: &mut TreeSvgViewSpec,
    preset: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match preset.trim() {
        "compact" => {
            spec.geometry.margin = 56.0;
            spec.geometry.node_width = 168.0;
            spec.geometry.node_height = 66.0;
            spec.geometry.x_gap = 230.0;
            spec.geometry.y_gap = 145.0;
            spec.geometry.radial_gap = 175.0;
        }
        "balanced" => {
            spec.geometry.margin = 88.0;
            spec.geometry.node_width = 196.0;
            spec.geometry.node_height = 76.0;
            spec.geometry.x_gap = 320.0;
            spec.geometry.y_gap = 205.0;
            spec.geometry.radial_gap = 235.0;
        }
        "spacious" => {
            spec.geometry.margin = 120.0;
            spec.geometry.node_width = 220.0;
            spec.geometry.node_height = 84.0;
            spec.geometry.x_gap = 390.0;
            spec.geometry.y_gap = 250.0;
            spec.geometry.radial_gap = 285.0;
        }
        "poster" => {
            spec.geometry.margin = 160.0;
            spec.geometry.node_width = 260.0;
            spec.geometry.node_height = 98.0;
            spec.geometry.x_gap = 500.0;
            spec.geometry.y_gap = 320.0;
            spec.geometry.radial_gap = 370.0;
        }
        other => {
            return Err(cli_error(format!(
                "unknown tree SVG preset `{other}`; expected compact, balanced, spacious, or poster"
            )));
        }
    }
    Ok(())
}

fn tree_svg_fixture_documents() -> Vec<(&'static str, kleio::TreeDocument)> {
    vec![
        ("nuclear-family", fixture_nuclear_family()),
        (
            "three-generation-pedigree",
            fixture_three_generation_pedigree(),
        ),
        ("second-marriage", fixture_second_marriage()),
        ("single-parent", fixture_single_parent()),
        ("adoption-step-family", fixture_adoption_step_family()),
        (
            "focus-ancestors-descendants",
            fixture_focus_ancestors_descendants(),
        ),
        ("pedigree-collapse", fixture_pedigree_collapse()),
        ("collateral-family", fixture_collateral_family()),
    ]
}

fn fixture_relationship_pair(
    tree: &kleio::TreeDocument,
    slug: &str,
) -> Option<(kleio::PersonId, kleio::PersonId)> {
    let pair = match slug {
        "nuclear-family" => ("First child", "Second child"),
        "three-generation-pedigree" => ("Focus", "Paternal grandfather"),
        "second-marriage" => ("Former child", "Current child"),
        "single-parent" => ("Child", "Parent"),
        "adoption-step-family" => ("Focus", "Adoptive parent"),
        "focus-ancestors-descendants" => ("Grandchild", "Paternal grandfather"),
        "pedigree-collapse" => ("Father", "Mother"),
        "collateral-family" => ("Focus", "Cousin"),
        _ => return None,
    };
    Some((
        find_fixture_person_by_name(tree, pair.0)?,
        find_fixture_person_by_name(tree, pair.1)?,
    ))
}

fn find_fixture_person_by_name(tree: &kleio::TreeDocument, name: &str) -> Option<kleio::PersonId> {
    tree.people
        .iter()
        .find(|person| tree.person_display_name(person.id) == Some(name))
        .map(|person| person.id)
}

fn tree_svg_fixture_description(slug: &str) -> &'static str {
    match slug {
        "nuclear-family" => {
            "Two parents and two children. Useful for checking couple units, sibling bars, and basic descendant layout."
        }
        "three-generation-pedigree" => {
            "A focus person with two parents and four grandparents. Useful for checking top-heavy pedigree placement."
        }
        "second-marriage" => {
            "A focus person with a current spouse, former spouse, and children from each relationship. Useful for checking partner-specific family groups."
        }
        "single-parent" => {
            "One parent and one child. Useful for checking connector behavior when there is no second parent."
        }
        "adoption-step-family" => {
            "Biological, adoptive, and step parent relationships for one focus person. Useful for checking complex parent sets."
        }
        "focus-ancestors-descendants" => {
            "A focus person with parents, grandparents, spouse, children, and a grandchild. Useful for checking true bidirectional hourglass behavior."
        }
        "pedigree-collapse" => {
            "A shared grandparent appears in both parental lines. Useful for checking repeated ancestors and pedigree-collapse rendering."
        }
        "collateral-family" => {
            "An extended family with grandparents, parents, aunt/uncle, sibling, cousin, partner, child, niece/nephew, cousin child, and grandchild. Useful for distinguishing Hourglass from Generational."
        }
        _ => "Fixture for visual tree rendering inspection.",
    }
}

fn fixture_nuclear_family() -> kleio::TreeDocument {
    let mut tree = kleio::TreeDocument::empty("fixture-nuclear-family", "Nuclear family");
    let father = tree.add_person("Father", Some(kleio::Sex::Male));
    let mother = tree.add_person("Mother", Some(kleio::Sex::Female));
    let child_one = tree.add_person("First child", None);
    let child_two = tree.add_person("Second child", None);
    tree.main_person = Some(father);
    tree.add_relationship(RelationshipKind::Spouse, father, mother);
    add_parent_child(&mut tree, father, child_one, Some(ParentRole::Father));
    add_parent_child(&mut tree, mother, child_one, Some(ParentRole::Mother));
    add_parent_child(&mut tree, father, child_two, Some(ParentRole::Father));
    add_parent_child(&mut tree, mother, child_two, Some(ParentRole::Mother));
    tree
}

fn fixture_three_generation_pedigree() -> kleio::TreeDocument {
    let mut tree =
        kleio::TreeDocument::empty("fixture-three-generation", "Three-generation pedigree");
    let focus = tree.add_person("Focus", None);
    let father = tree.add_person("Father", Some(kleio::Sex::Male));
    let mother = tree.add_person("Mother", Some(kleio::Sex::Female));
    let paternal_grandfather = tree.add_person("Paternal grandfather", Some(kleio::Sex::Male));
    let paternal_grandmother = tree.add_person("Paternal grandmother", Some(kleio::Sex::Female));
    let maternal_grandfather = tree.add_person("Maternal grandfather", Some(kleio::Sex::Male));
    let maternal_grandmother = tree.add_person("Maternal grandmother", Some(kleio::Sex::Female));
    tree.main_person = Some(focus);
    add_parent_child(&mut tree, father, focus, Some(ParentRole::Father));
    add_parent_child(&mut tree, mother, focus, Some(ParentRole::Mother));
    add_parent_child(
        &mut tree,
        paternal_grandfather,
        father,
        Some(ParentRole::Father),
    );
    add_parent_child(
        &mut tree,
        paternal_grandmother,
        father,
        Some(ParentRole::Mother),
    );
    add_parent_child(
        &mut tree,
        maternal_grandfather,
        mother,
        Some(ParentRole::Father),
    );
    add_parent_child(
        &mut tree,
        maternal_grandmother,
        mother,
        Some(ParentRole::Mother),
    );
    tree
}

fn fixture_second_marriage() -> kleio::TreeDocument {
    let mut tree = kleio::TreeDocument::empty(
        "fixture-second-marriage",
        "Second marriage / blended family",
    );
    let focus = tree.add_person("Focus", None);
    let current_spouse = tree.add_person("Current spouse", None);
    let former_spouse = tree.add_person("Former spouse", None);
    let current_child = tree.add_person("Current child", None);
    let former_child = tree.add_person("Former child", None);
    tree.main_person = Some(focus);
    tree.add_relationship(RelationshipKind::Spouse, focus, current_spouse);
    tree.add_relationship(RelationshipKind::FormerSpouse, focus, former_spouse);
    add_parent_child(&mut tree, focus, current_child, None);
    add_parent_child(&mut tree, current_spouse, current_child, None);
    add_parent_child(&mut tree, focus, former_child, None);
    add_parent_child(&mut tree, former_spouse, former_child, None);
    tree
}

fn fixture_single_parent() -> kleio::TreeDocument {
    let mut tree = kleio::TreeDocument::empty("fixture-single-parent", "Single-parent family");
    let parent = tree.add_person("Parent", None);
    let child = tree.add_person("Child", None);
    tree.main_person = Some(child);
    add_parent_child(&mut tree, parent, child, None);
    tree
}

fn fixture_adoption_step_family() -> kleio::TreeDocument {
    let mut tree =
        kleio::TreeDocument::empty("fixture-adoption-step-family", "Adoption and step-family");
    let focus = tree.add_person("Focus", None);
    let biological_mother = tree.add_person("Biological mother", Some(kleio::Sex::Female));
    let biological_father = tree.add_person("Biological father", Some(kleio::Sex::Male));
    let adoptive_parent = tree.add_person("Adoptive parent", None);
    let step_parent = tree.add_person("Step-parent", None);
    tree.main_person = Some(focus);
    add_parent_child_with_kind(
        &mut tree,
        biological_mother,
        focus,
        RelationshipKind::BiologicalParentChild,
        Some(ParentRole::Mother),
    );
    add_parent_child_with_kind(
        &mut tree,
        biological_father,
        focus,
        RelationshipKind::BiologicalParentChild,
        Some(ParentRole::Father),
    );
    add_parent_child_with_kind(
        &mut tree,
        adoptive_parent,
        focus,
        RelationshipKind::AdoptiveParentChild,
        Some(ParentRole::Parent),
    );
    add_parent_child_with_kind(
        &mut tree,
        step_parent,
        focus,
        RelationshipKind::StepParentChild,
        Some(ParentRole::Parent),
    );
    tree
}

fn fixture_focus_ancestors_descendants() -> kleio::TreeDocument {
    let mut tree = kleio::TreeDocument::empty(
        "fixture-focus-ancestors-descendants",
        "Focus ancestors and descendants",
    );
    let focus = tree.add_person("Focus", None);
    let father = tree.add_person("Father", Some(kleio::Sex::Male));
    let mother = tree.add_person("Mother", Some(kleio::Sex::Female));
    let paternal_grandfather = tree.add_person("Paternal grandfather", Some(kleio::Sex::Male));
    let paternal_grandmother = tree.add_person("Paternal grandmother", Some(kleio::Sex::Female));
    let maternal_grandfather = tree.add_person("Maternal grandfather", Some(kleio::Sex::Male));
    let maternal_grandmother = tree.add_person("Maternal grandmother", Some(kleio::Sex::Female));
    let spouse = tree.add_person("Spouse", None);
    let child_one = tree.add_person("Child 1", None);
    let child_two = tree.add_person("Child 2", None);
    let grandchild = tree.add_person("Grandchild", None);
    tree.main_person = Some(focus);
    tree.add_relationship(RelationshipKind::Spouse, father, mother);
    tree.add_relationship(
        RelationshipKind::Spouse,
        paternal_grandfather,
        paternal_grandmother,
    );
    tree.add_relationship(
        RelationshipKind::Spouse,
        maternal_grandfather,
        maternal_grandmother,
    );
    tree.add_relationship(RelationshipKind::Spouse, focus, spouse);
    add_parent_child(&mut tree, father, focus, Some(ParentRole::Father));
    add_parent_child(&mut tree, mother, focus, Some(ParentRole::Mother));
    add_parent_child(
        &mut tree,
        paternal_grandfather,
        father,
        Some(ParentRole::Father),
    );
    add_parent_child(
        &mut tree,
        paternal_grandmother,
        father,
        Some(ParentRole::Mother),
    );
    add_parent_child(
        &mut tree,
        maternal_grandfather,
        mother,
        Some(ParentRole::Father),
    );
    add_parent_child(
        &mut tree,
        maternal_grandmother,
        mother,
        Some(ParentRole::Mother),
    );
    add_parent_child(&mut tree, focus, child_one, None);
    add_parent_child(&mut tree, spouse, child_one, None);
    add_parent_child(&mut tree, focus, child_two, None);
    add_parent_child(&mut tree, spouse, child_two, None);
    add_parent_child(&mut tree, child_two, grandchild, None);
    tree
}

fn fixture_collateral_family() -> kleio::TreeDocument {
    let mut tree = kleio::TreeDocument::empty("fixture-collateral-family", "Collateral family");
    let paternal_grandfather = tree.add_person("Paternal grandfather", Some(kleio::Sex::Male));
    let paternal_grandmother = tree.add_person("Paternal grandmother", Some(kleio::Sex::Female));
    let father = tree.add_person("Father", Some(kleio::Sex::Male));
    let father_sibling = tree.add_person("Father's sibling", None);
    let mother = tree.add_person("Mother", Some(kleio::Sex::Female));
    let focus = tree.add_person("Focus", None);
    let focus_sibling = tree.add_person("Focus sibling", None);
    let focus_spouse = tree.add_person("Focus spouse", None);
    let cousin = tree.add_person("Cousin", None);
    let focus_child = tree.add_person("Focus child", None);
    let sibling_child = tree.add_person("Sibling child", None);
    let cousin_child = tree.add_person("Cousin child", None);
    let grandchild = tree.add_person("Grandchild", None);

    tree.main_person = Some(focus);
    tree.add_relationship(
        RelationshipKind::Spouse,
        paternal_grandfather,
        paternal_grandmother,
    );
    tree.add_relationship(RelationshipKind::Spouse, father, mother);
    tree.add_relationship(RelationshipKind::Spouse, focus, focus_spouse);
    add_parent_child(
        &mut tree,
        paternal_grandfather,
        father,
        Some(ParentRole::Father),
    );
    add_parent_child(
        &mut tree,
        paternal_grandmother,
        father,
        Some(ParentRole::Mother),
    );
    add_parent_child(
        &mut tree,
        paternal_grandfather,
        father_sibling,
        Some(ParentRole::Father),
    );
    add_parent_child(
        &mut tree,
        paternal_grandmother,
        father_sibling,
        Some(ParentRole::Mother),
    );
    add_parent_child(&mut tree, father, focus, Some(ParentRole::Father));
    add_parent_child(&mut tree, mother, focus, Some(ParentRole::Mother));
    add_parent_child(&mut tree, father, focus_sibling, Some(ParentRole::Father));
    add_parent_child(&mut tree, mother, focus_sibling, Some(ParentRole::Mother));
    add_parent_child(&mut tree, father_sibling, cousin, None);
    add_parent_child(&mut tree, focus, focus_child, None);
    add_parent_child(&mut tree, focus_spouse, focus_child, None);
    add_parent_child(&mut tree, focus_sibling, sibling_child, None);
    add_parent_child(&mut tree, cousin, cousin_child, None);
    add_parent_child(&mut tree, focus_child, grandchild, None);
    tree
}

fn fixture_pedigree_collapse() -> kleio::TreeDocument {
    let mut tree = kleio::TreeDocument::empty("fixture-pedigree-collapse", "Pedigree collapse");
    let focus = tree.add_person("Focus", None);
    let father = tree.add_person("Father", Some(kleio::Sex::Male));
    let mother = tree.add_person("Mother", Some(kleio::Sex::Female));
    let shared_grandfather = tree.add_person("Shared grandfather", Some(kleio::Sex::Male));
    let paternal_grandmother = tree.add_person("Paternal grandmother", Some(kleio::Sex::Female));
    let maternal_grandmother = tree.add_person("Maternal grandmother", Some(kleio::Sex::Female));
    tree.main_person = Some(focus);
    add_parent_child(&mut tree, father, focus, Some(ParentRole::Father));
    add_parent_child(&mut tree, mother, focus, Some(ParentRole::Mother));
    add_parent_child(
        &mut tree,
        shared_grandfather,
        father,
        Some(ParentRole::Father),
    );
    add_parent_child(
        &mut tree,
        paternal_grandmother,
        father,
        Some(ParentRole::Mother),
    );
    add_parent_child(
        &mut tree,
        shared_grandfather,
        mother,
        Some(ParentRole::Father),
    );
    add_parent_child(
        &mut tree,
        maternal_grandmother,
        mother,
        Some(ParentRole::Mother),
    );
    tree
}

fn add_parent_child(
    tree: &mut kleio::TreeDocument,
    parent: kleio::PersonId,
    child: kleio::PersonId,
    role: Option<ParentRole>,
) {
    add_parent_child_with_kind(
        tree,
        parent,
        child,
        RelationshipKind::BiologicalParentChild,
        role,
    );
}

fn add_parent_child_with_kind(
    tree: &mut kleio::TreeDocument,
    parent: kleio::PersonId,
    child: kleio::PersonId,
    kind: RelationshipKind,
    role: Option<ParentRole>,
) {
    let relationship_id = tree.add_relationship(kind, parent, child);
    if let Some(relationship) = tree
        .relationships
        .iter_mut()
        .find(|relationship| relationship.id == relationship_id)
    {
        relationship.parent_role = role;
    }
}

pub(crate) fn tree_svg_view_spec_from_world(
    world_root: &std::path::Path,
    view_slug: Option<&str>,
) -> Result<TreeSvgViewSpec, Box<dyn std::error::Error>> {
    let bundle = read_local_data_unvalidated(world_root)?;
    let view = select_tree_view_document(&bundle.toml_documents, view_slug);
    let mut spec = TreeSvgViewSpec::default();

    if let Some(view) = view {
        spec.projection.focus_person = view
            .data
            .get("root")
            .and_then(|root| root.get("entity"))
            .and_then(serde_json::Value::as_str)
            .and_then(|entity| resolve_view_person_id(entity).ok());

        let projection = view
            .data
            .get("projection")
            .or_else(|| view.data.get("filter"));
        if let Some(projection) = projection {
            spec.projection.generations_up = projection
                .get("generations_up")
                .and_then(serde_json::Value::as_u64)
                .map(|value| value as u32);
            spec.projection.generations_down = projection
                .get("generations_down")
                .and_then(serde_json::Value::as_u64)
                .map(|value| value as u32);
            spec.projection.include_partners = projection
                .get("include_partners")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(spec.projection.include_partners);
            spec.projection.include_siblings = projection
                .get("include_siblings")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(spec.projection.include_siblings);
            spec.projection.include_unconnected = projection
                .get("include_unconnected")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(spec.projection.include_unconnected);
            let relationship_kinds = projection
                .get("relationship_kinds")
                .and_then(serde_json::Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .filter_map(serde_json::Value::as_str)
                        .map(RelationshipKind::from_value)
                        .collect::<Vec<_>>()
                })
                .filter(|values| !values.is_empty());
            spec.projection.relationship_kinds = relationship_kinds;
        }

        if let Some(layout) = view.data.get("layout") {
            if let Some(algorithm) = layout.get("algorithm").and_then(serde_json::Value::as_str) {
                spec.geometry.layout = parse_tree_svg_layout(algorithm)?;
            }
            if let Some(orientation) = layout
                .get("orientation")
                .and_then(serde_json::Value::as_str)
            {
                spec.geometry.orientation = parse_tree_svg_orientation(orientation)?;
            }
        }

        if let Some(node) = view.data.get("node").or_else(|| view.data.get("display")) {
            spec.node_content.show_life_dates = node
                .get("show_life_dates")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(spec.node_content.show_life_dates);
            spec.node_content.show_relationship_labels = node
                .get("show_relationship_labels")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(spec.node_content.show_relationship_labels);
            spec.node_content.show_person_ids = node
                .get("show_person_ids")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(spec.node_content.show_person_ids);
            spec.node_content.show_sex = node
                .get("show_sex")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(spec.node_content.show_sex);
        }
    }

    Ok(spec)
}

fn select_tree_view_document<'a>(
    documents: &'a [kleio::LocalTomlDocument],
    view_slug: Option<&str>,
) -> Option<&'a kleio::LocalTomlDocument> {
    let trees = documents
        .iter()
        .filter(|document| document.kind.as_deref() == Some("tree-view"));
    if let Some(view_slug) = view_slug {
        let view_id = format!("tree:{view_slug}");
        return trees.into_iter().find(|document| {
            document.id.as_deref() == Some(view_id.as_str())
                || document.path == format!("views/trees/{view_slug}.toml")
        });
    }
    trees.into_iter().next()
}

fn resolve_view_person_id(value: &str) -> Result<kleio::PersonId, Box<dyn std::error::Error>> {
    value
        .strip_prefix("person:")
        .unwrap_or(value)
        .parse::<u64>()
        .map(kleio::PersonId)
        .map_err(|_| cli_error(format!("tree SVG view root `{value}` is not a numeric compiled PersonId; CLI export will fall back to the compiled tree main person")))
}

fn parse_tree_svg_orientation(
    value: &str,
) -> Result<TreeSvgOrientation, Box<dyn std::error::Error>> {
    match value.trim() {
        "top-down" => Ok(TreeSvgOrientation::TopDown),
        "bottom-up" => Ok(TreeSvgOrientation::BottomUp),
        "left-right" => Ok(TreeSvgOrientation::LeftRight),
        "right-left" => Ok(TreeSvgOrientation::RightLeft),
        other => Err(cli_error(format!(
            "unknown tree SVG orientation `{other}`; expected top-down, bottom-up, left-right, or right-left"
        ))),
    }
}

fn resolve_export_focus_person(
    tree: &kleio::TreeDocument,
    value: &str,
) -> Result<kleio::PersonId, Box<dyn std::error::Error>> {
    let normalized = person_id(value);
    tree.people
        .iter()
        .find(|person| {
            person
                .source_record
                .as_ref()
                .and_then(|source| source.0.strip_prefix("local:"))
                .is_some_and(|source_id| source_id == value || source_id == normalized)
        })
        .map(|person| person.id)
        .or_else(|| value.parse::<u64>().ok().map(kleio::PersonId))
        .filter(|person_id| tree.has_person(*person_id))
        .ok_or_else(|| {
            cli_error(format!(
                "person `{value}` was not found in the compiled tree"
            ))
        })
}
