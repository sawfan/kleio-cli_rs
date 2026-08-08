use super::*;

/// Validate and compile world files into a semantic JSON bundle.
#[derive(Debug, clap::Args)]
pub(crate) struct CompileArgs {
    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Output JSON path. Defaults to <world-root>/build/kleio.compiled.json.
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

    /// Output JSON path. Defaults to <world-root>/build/kleio.ecs.json.
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

    /// Output JSON path. Defaults to <world-root>/build/<view-or-timeline>.timeline.json.
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

    /// Output JSON path. Defaults to <world-root>/build/<view-or-kleio-tree>.json.
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}
