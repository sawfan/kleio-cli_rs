use super::*;
use crate::cli_build::*;
use crate::cli_events::*;
use crate::cli_gedcom::*;
use crate::cli_inspect::*;
use crate::cli_people::*;
use crate::cli_records::*;
use crate::cli_redact::*;
use crate::cli_workspace::*;

#[derive(Debug, Parser)]
#[command(name = "kleio-cli")]
#[command(about = "Kleio world/workspace local authoring tools")]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Create a starter Kleio workspace with a default world.
    InitWorkspace(InitWorkspaceArgs),
    /// Alias for init-workspace while older local scripts migrate.
    Init(InitArgs),
    /// Create an empty world under worlds/<world>.
    NewWorld(NewWorldArgs),
    /// List worlds registered in workspace kleio.toml.
    ListWorlds(ListWorldsArgs),
    /// Set the workspace default world.
    SetDefaultWorld(SetDefaultWorldArgs),
    /// Create a person record, optionally with a starter birth event.
    NewPerson(NewPersonArgs),
    /// Add a relative to an existing person with one command.
    AddRelative(AddRelativeArgs),
    /// Connect two existing people with a relationship.
    ConnectRelative(ConnectRelativeArgs),
    /// Create a place entity record.
    NewPlace(NewPlaceArgs),
    /// Create an organization entity record.
    NewOrganization(NewOrganizationArgs),
    /// Create an object entity record.
    NewObject(NewObjectArgs),
    /// Create a concept entity record.
    NewConcept(NewConceptArgs),
    /// Add a new spouse for an existing person, creating the person, marriage event, and spouse relationship.
    AddSpouse(AddSpouseArgs),
    /// Add a marriage event and spouse relationship between two people.
    AddMarriage(AddMarriageArgs),
    /// Add a divorce event and former-spouse relationship between two people.
    AddDivorce(AddDivorceArgs),
    /// Add a death event for one person.
    AddDeath(AddDeathArgs),
    /// Add a life event for one or more people.
    AddEvent(AddEventArgs),
    /// Create a semantic event record.
    NewEvent(NewEventArgs),
    /// Create an event collection or sequence.
    NewCollection(NewCollectionArgs),
    /// Create a relationship between two person entities.
    NewRelationship(NewRelationshipArgs),
    /// Create a source record.
    NewSource(NewSourceArgs),
    /// Create an assertion record.
    NewAssertion(NewAssertionArgs),
    /// Create a birth event for an existing person.
    NewBirth(NewBirthArgs),
    /// Create a timeline view.
    NewTimeline(NewTimelineArgs),
    /// Create a tree view.
    NewTree(NewTreeArgs),
    /// Create a map view.
    NewMap(NewMapArgs),
    /// Create a calendar view.
    NewCalendar(NewCalendarArgs),
    /// Create a visualization view.
    NewVisualization(NewVisualizationArgs),
    /// Create a schema definition record.
    NewSchema(NewSchemaArgs),
    /// Create an import report TOML file under imports/<kind>/.
    NewImportReport(NewImportReportArgs),
    /// Show progressive self-authoring suggestions for building a family tree.
    Guide(GuideArgs),
    /// List person records in a world.
    ListPeople(ListPeopleArgs),
    /// List event records in a world.
    ListEvents(ListEventsArgs),
    /// List source records in a world.
    ListSources(ListSourcesArgs),
    /// List saved views in a world.
    ListViews(ListViewsArgs),
    /// Print a small text visualization of nearby family relationships.
    TreeSketch(TreeSketchArgs),
    /// Inspect a tree view projection and report likely layout/data issues.
    InspectTreeView(InspectTreeViewArgs),
    /// Point a world's world.toml at the active primary GEDCOM file.
    SetGedcom(SetGedcomArgs),
    /// Ingest a GEDCOM file into first-pass world records.
    IngestGedcom(IngestGedcomArgs),
    /// Create an assertion connecting a source to a target field or record.
    AttachSource(AttachSourceArgs),
    /// Print a redacted copy of an authored Markdown/TOML file for sharing diagnostics.
    RedactFile(RedactFileArgs),
    /// Write a redacted copy of a world for private diagnostics sharing.
    RedactWorld(RedactWorldArgs),
    /// Print a redacted authored-file tree for the selected world.
    RedactWorldTree(RedactWorldTreeArgs),
    /// Print a redacted, pasteable dump of the selected authored world.
    RedactWorldDump(RedactWorldDumpArgs),
    /// Validate world files without writing build outputs.
    Validate(ValidateArgs),
    /// Check local media/source file references.
    CheckMedia(CheckMediaArgs),
    /// Check authored world records and report actionable warnings.
    Doctor(DoctorArgs),
    /// Summarize authored world records and common attention items.
    Summary(SummaryArgs),
    /// Validate and compile world files into a semantic JSON bundle.
    Compile(CompileArgs),
    /// Compile world files into a minimal ECS-friendly JSON bundle.
    CompileEcs(CompileEcsArgs),
    /// Compile both semantic and ECS bundles for a world.
    Build(BuildArgs),
    /// Compile world events into a timeline view JSON projection.
    CompileTimeline(CompileTimelineArgs),
    /// Compile world person records into the current tree view JSON projection.
    CompileTree(CompileTreeArgs),
    /// Compile world person records into Urania's browser tree upload JSON format.
    CompileTrees(CompileTreesArgs),
    /// Render a compiled world tree view to a browser-independent SVG file.
    ExportTreeSvg(ExportTreeSvgArgs),
    /// Render every tree SVG layout, optionally with PNG copies.
    ExportTreeLayouts(ExportTreeLayoutsArgs),
    /// Render built-in family tree SVG fixture examples for visual regression checks.
    ExportTreeFixtures(ExportTreeFixturesArgs),
}
