use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub(crate) enum DoctorLevelArg {
    Complete,
    Structure,
}

impl DoctorLevelArg {
    pub(crate) fn includes(self, diagnostic: &kleio::LocalWorldDiagnostic) -> bool {
        match self {
            Self::Complete => true,
            Self::Structure => !matches!(
                diagnostic.kind,
                kleio::LocalWorldDiagnosticKind::EventMissingSource
                    | kleio::LocalWorldDiagnosticKind::RelationshipMissingSource
                    | kleio::LocalWorldDiagnosticKind::ReferencedFileMissing
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum SexArg {
    Female,
    Male,
    Unknown,
    Other,
}

impl SexArg {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Female => "female",
            Self::Male => "male",
            Self::Unknown => "unknown",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum RelativeArg {
    Parent,
    StepParent,
    Child,
    Sibling,
    Partner,
    Spouse,
}

impl RelativeArg {
    pub(crate) fn default_relationship_kind(self) -> &'static str {
        match self {
            Self::Parent | Self::Child => "biological-parent-child",
            Self::StepParent => "step-parent-child",
            Self::Sibling => "sibling",
            Self::Partner => "partner",
            Self::Spouse => "spouse",
        }
    }

    pub(crate) fn slug_suffix(self) -> &'static str {
        match self {
            Self::Parent | Self::Child => "parent-child",
            Self::StepParent => "step-parent-child",
            Self::Sibling => "sibling",
            Self::Partner => "partner",
            Self::Spouse => "spouse",
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum ParentRoleArg {
    Father,
    Mother,
    Parent,
    Unknown,
}

impl ParentRoleArg {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Father => "father",
            Self::Mother => "mother",
            Self::Parent => "parent",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum CollectionOrderArg {
    Chronological,
    Manual,
    ManualThenChronological,
}

impl From<CollectionOrderArg> for LocalCollectionOrder {
    fn from(value: CollectionOrderArg) -> Self {
        match value {
            CollectionOrderArg::Chronological => Self::Chronological,
            CollectionOrderArg::Manual => Self::Manual,
            CollectionOrderArg::ManualThenChronological => Self::ManualThenChronological,
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum ViewKindArg {
    Timeline,
    Tree,
    Map,
    Calendar,
    Visualization,
}

impl From<ViewKindArg> for LocalViewKind {
    fn from(value: ViewKindArg) -> Self {
        match value {
            ViewKindArg::Timeline => Self::Timeline,
            ViewKindArg::Tree => Self::Tree,
            ViewKindArg::Map => Self::Map,
            ViewKindArg::Calendar => Self::Calendar,
            ViewKindArg::Visualization => Self::Visualization,
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum ImportKindArg {
    Gedcom,
    Wikidata,
    Csv,
}

impl From<ImportKindArg> for LocalImportKind {
    fn from(value: ImportKindArg) -> Self {
        match value {
            ImportKindArg::Gedcom => Self::Gedcom,
            ImportKindArg::Wikidata => Self::Wikidata,
            ImportKindArg::Csv => Self::Csv,
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum SchemaKindArg {
    Component,
    Bundle,
    Event,
    View,
    Vocab,
}

impl From<SchemaKindArg> for LocalSchemaKind {
    fn from(value: SchemaKindArg) -> Self {
        match value {
            SchemaKindArg::Component => Self::Component,
            SchemaKindArg::Bundle => Self::Bundle,
            SchemaKindArg::Event => Self::Event,
            SchemaKindArg::View => Self::View,
            SchemaKindArg::Vocab => Self::Vocab,
        }
    }
}
