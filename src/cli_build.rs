use super::*;
/// Render built-in family tree SVG fixture examples for visual regression checks.
#[derive(Debug, clap::Args)]
pub(crate) struct ExportTreeFixturesArgs {
    /// Output directory. Defaults to target/kleio-tree-fixtures.
    #[arg(long)]
    pub(crate) out_dir: Option<PathBuf>,

    /// Also render PNG files beside the SVG files.
    #[arg(long)]
    pub(crate) png: bool,

    /// PNG width in pixels when --png is set.
    #[arg(long, default_value_t = 2400)]
    pub(crate) png_width: u32,

    /// PNG height in pixels when --png is set.
    #[arg(long, default_value_t = 1800)]
    pub(crate) png_height: u32,

    /// Also generate a labeled contact-sheet SVG, and PNG when --png is set.
    #[arg(long)]
    pub(crate) contact_sheet: bool,

    /// Contact sheet thumbnail cell width.
    #[arg(long, default_value_t = 520)]
    pub(crate) contact_cell_width: u32,

    /// Contact sheet thumbnail cell height.
    #[arg(long, default_value_t = 390)]
    pub(crate) contact_cell_height: u32,

    /// Also generate larger standalone SVG renders for primary regression cases.
    #[arg(long)]
    pub(crate) standalone_snapshots: bool,

    /// Standalone snapshot width in SVG units.
    #[arg(long, default_value_t = 1400)]
    pub(crate) standalone_width: u32,

    /// Standalone snapshot height in SVG units.
    #[arg(long, default_value_t = 1000)]
    pub(crate) standalone_height: u32,

    #[command(flatten)]
    pub(crate) svg: TreeSvgOverrideArgs,
}

/// Validate and compile world files into a semantic JSON bundle.
#[derive(Debug, clap::Args)]
pub(crate) struct CompileArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Output JSON path. Defaults to the selected world's build directory. Relative paths are resolved from the current working directory.
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

/// Compile world files into a minimal ECS-friendly JSON bundle.
#[derive(Debug, clap::Args)]
pub(crate) struct CompileEcsArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Output JSON path. Defaults to the selected world's build directory. Relative paths are resolved from the current working directory.
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

/// Compile both semantic and ECS bundles for a world.
#[derive(Debug, clap::Args)]
pub(crate) struct BuildArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Optional timeline view slug to compile during build.
    #[arg(long)]
    pub(crate) timeline_view: Option<String>,

    /// Optional tree view slug to compile during build.
    #[arg(long)]
    pub(crate) tree_view: Option<String>,
}

/// Compile world events into a timeline view JSON projection.
#[derive(Debug, clap::Args)]
pub(crate) struct CompileTimelineArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Timeline view slug. Defaults to the first timeline view in the world.
    #[arg(long)]
    pub(crate) view: Option<String>,

    /// Output JSON path. Defaults to the selected world's build directory. Relative paths are resolved from the current working directory.
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

/// Compile world person records into the current tree view JSON projection.
#[derive(Debug, clap::Args)]
pub(crate) struct CompileTreeArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Tree view slug. Defaults to the first tree view in the world.
    #[arg(long)]
    pub(crate) view: Option<String>,

    /// Output JSON path. Defaults to the selected world's build directory. Relative paths are resolved from the current working directory.
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

/// Compile world person records into Urania's browser tree upload JSON format.
#[derive(Debug, clap::Args)]
pub(crate) struct CompileTreesArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Tree view slug. Defaults to the first tree view in the world.
    #[arg(long)]
    pub(crate) view: Option<String>,

    /// Focus person slug or full person id. Overrides the tree's main person in the browser upload.
    #[arg(long)]
    pub(crate) person: Option<String>,

    /// Output JSON path. Defaults to the selected world's build directory. Relative paths are resolved from the current working directory.
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

/// Render a compiled world tree view to a browser-independent SVG file.
#[derive(Debug, clap::Args)]
pub(crate) struct ExportTreeSvgArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Tree view slug. Defaults to the first tree view in the world.
    #[arg(long)]
    pub(crate) view: Option<String>,

    /// Focus person slug or full person id. Defaults to the tree view root or tree main person.
    #[arg(long)]
    pub(crate) person: Option<String>,

    /// SVG layout: hourglass, descendants, ancestors, or radial. Defaults to [layout].algorithm in the tree view, then hourglass.
    #[arg(long)]
    pub(crate) layout: Option<String>,

    /// Output SVG path. Defaults to the selected world's build directory. Relative paths are resolved from the current working directory.
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,

    /// Also render a PNG file next to the SVG file.
    #[arg(long)]
    pub(crate) png: bool,

    /// PNG width in pixels when --png is set.
    #[arg(long, default_value_t = 2400)]
    pub(crate) png_width: u32,

    /// PNG height in pixels when --png is set.
    #[arg(long, default_value_t = 1800)]
    pub(crate) png_height: u32,

    #[command(flatten)]
    pub(crate) svg: TreeSvgOverrideArgs,
}

/// Shared SVG projection, spacing, and node-content overrides for tree exports.
#[derive(Debug, Clone, Default, clap::Args)]
pub(crate) struct TreeSvgOverrideArgs {
    /// Apply a named SVG spacing preset before individual spacing overrides: compact, balanced, spacious, or poster.
    #[arg(long)]
    pub(crate) preset: Option<String>,

    /// Override ancestor generations included in the SVG projection.
    #[arg(long)]
    pub(crate) generations_up: Option<u32>,

    /// Override descendant generations included in the SVG projection.
    #[arg(long)]
    pub(crate) generations_down: Option<u32>,

    /// Include siblings of the focus person in the SVG projection.
    #[arg(long)]
    pub(crate) include_siblings: bool,

    /// Include people outside the focused projection.
    #[arg(long)]
    pub(crate) include_unconnected: bool,

    /// Exclude spouses/partners from the SVG projection.
    #[arg(long)]
    pub(crate) no_partners: bool,

    /// Override SVG margin.
    #[arg(long)]
    pub(crate) margin: Option<f32>,

    /// Override rendered node width.
    #[arg(long)]
    pub(crate) node_width: Option<f32>,

    /// Override rendered node height.
    #[arg(long)]
    pub(crate) node_height: Option<f32>,

    /// Override horizontal spacing between layout groups.
    #[arg(long)]
    pub(crate) x_gap: Option<f32>,

    /// Override vertical spacing between generations.
    #[arg(long)]
    pub(crate) y_gap: Option<f32>,

    /// Override radial layout ring spacing.
    #[arg(long)]
    pub(crate) radial_gap: Option<f32>,

    /// Render compiled numeric person ids inside nodes.
    #[arg(long)]
    pub(crate) show_person_ids: bool,

    /// Render sex labels inside nodes.
    #[arg(long)]
    pub(crate) show_sex: bool,

    /// Render relationship labels along connectors.
    #[arg(long)]
    pub(crate) show_relationship_labels: bool,
}

/// Render every tree SVG layout, optionally with PNG copies.
#[derive(Debug, clap::Args)]
pub(crate) struct ExportTreeLayoutsArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Tree view slug. Defaults to the first tree view in the world.
    #[arg(long)]
    pub(crate) view: Option<String>,

    /// Focus person slug or full person id. Defaults to the tree view root or tree main person.
    #[arg(long)]
    pub(crate) person: Option<String>,

    /// Output directory. Defaults to the selected world's build/tree-layouts directory.
    #[arg(long)]
    pub(crate) out_dir: Option<PathBuf>,

    /// Also render PNG files beside the SVG files.
    #[arg(long)]
    pub(crate) png: bool,

    /// PNG width in pixels when --png is set.
    #[arg(long, default_value_t = 2400)]
    pub(crate) png_width: u32,

    /// PNG height in pixels when --png is set.
    #[arg(long, default_value_t = 1800)]
    pub(crate) png_height: u32,

    #[command(flatten)]
    pub(crate) svg: TreeSvgOverrideArgs,
}
