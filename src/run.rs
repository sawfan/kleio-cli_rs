use crate::cli::{Args, Command};

pub(crate) fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Command::InitWorkspace(..)
        | Command::Init(..)
        | Command::NewWorld(..)
        | Command::ListWorlds(..)
        | Command::SetDefaultWorld(..)
        | Command::Guide(..) => crate::run_workspace::run(args.command)?,
        Command::NewPerson(..)
        | Command::AddRelative(..)
        | Command::ConnectRelative(..)
        | Command::AddSpouse(..) => crate::run_people::run(args.command)?,
        Command::AddMarriage(..)
        | Command::AddDivorce(..)
        | Command::AddDeath(..)
        | Command::AddEvent(..)
        | Command::NewEvent(..)
        | Command::NewBirth(..) => crate::run_events::run(args.command)?,
        Command::NewPlace(..)
        | Command::NewOrganization(..)
        | Command::NewObject(..)
        | Command::NewConcept(..)
        | Command::NewCollection(..)
        | Command::NewRelationship(..)
        | Command::NewSource(..)
        | Command::NewAssertion(..)
        | Command::NewTimeline(..)
        | Command::NewTree(..)
        | Command::NewMap(..)
        | Command::NewCalendar(..)
        | Command::NewVisualization(..)
        | Command::NewSchema(..)
        | Command::NewImportReport(..)
        | Command::AttachSource(..) => crate::run_records::run(args.command)?,
        Command::ListPeople(..)
        | Command::ListEvents(..)
        | Command::ListSources(..)
        | Command::ListViews(..)
        | Command::TreeSketch(..)
        | Command::InspectTreeView(..)
        | Command::Validate(..)
        | Command::CheckMedia(..)
        | Command::Doctor(..)
        | Command::Summary(..) => crate::run_inspect::run(args.command)?,
        Command::SetGedcom(..) | Command::IngestGedcom(..) => crate::run_gedcom::run(args.command)?,
        Command::RedactFile(..)
        | Command::RedactWorld(..)
        | Command::RedactWorldDump(..)
        | Command::RedactWorldTree(..) => crate::run_redact::run(args.command)?,
        Command::Compile(..)
        | Command::CompileEcs(..)
        | Command::Build(..)
        | Command::CompileTimeline(..)
        | Command::CompileTree(..)
        | Command::CompileTrees(..)
        | Command::ExportTreeSvg(..)
        | Command::ExportTreeLayouts(..)
        | Command::ExportTreeFixtures(..) => crate::run_build::run(args.command)?,
    }
    Ok(())
}
