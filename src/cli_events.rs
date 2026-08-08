use super::*;

/// Add a marriage event and spouse relationship between two people.
#[derive(Debug, clap::Args)]
pub(crate) struct AddMarriageArgs {
    /// First spouse person slug/id.
    pub(crate) first_person: String,

    /// Second spouse person slug/id.
    pub(crate) second_person: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Event slug. Defaults to <first>-<second>-marriage.
    #[arg(long)]
    pub(crate) slug: Option<String>,

    /// Optional event title. When omitted, the marriage label is derived from participants.
    #[arg(long)]
    pub(crate) title: Option<String>,

    /// Place entity ids. May be repeated.
    #[arg(long = "place")]
    pub(crate) places: Vec<String>,

    /// Inline event location label.
    #[arg(long)]
    pub(crate) location: Option<String>,

    /// Inline event location latitude.
    #[arg(long)]
    pub(crate) latitude: Option<f64>,

    /// Inline event location longitude.
    #[arg(long)]
    pub(crate) longitude: Option<f64>,

    /// Event time/date text.
    #[arg(long)]
    pub(crate) time: Option<String>,

    /// Event date precision. Inferred from --time when omitted.
    #[arg(long)]
    pub(crate) date_precision: Option<String>,

    /// Event source ids. May be repeated.
    #[arg(long = "source")]
    pub(crate) sources: Vec<String>,

    /// Optional source record ids for the spouse relationship. May be repeated.
    #[arg(long = "relationship-source")]
    pub(crate) relationship_sources: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Add a divorce event and former-spouse relationship between two people.
#[derive(Debug, clap::Args)]
pub(crate) struct AddDivorceArgs {
    /// First former spouse person slug/id.
    pub(crate) first_person: String,

    /// Second former spouse person slug/id.
    pub(crate) second_person: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Event slug. Defaults to <first>-<second>-divorce.
    #[arg(long)]
    pub(crate) slug: Option<String>,

    /// Optional event title. When omitted, the divorce label is derived from participants.
    #[arg(long)]
    pub(crate) title: Option<String>,

    /// Place entity ids. May be repeated.
    #[arg(long = "place")]
    pub(crate) places: Vec<String>,

    /// Inline event location label.
    #[arg(long)]
    pub(crate) location: Option<String>,

    /// Inline event location latitude.
    #[arg(long)]
    pub(crate) latitude: Option<f64>,

    /// Inline event location longitude.
    #[arg(long)]
    pub(crate) longitude: Option<f64>,

    /// Event time/date text.
    #[arg(long)]
    pub(crate) time: Option<String>,

    /// Event date precision. Inferred from --time when omitted.
    #[arg(long)]
    pub(crate) date_precision: Option<String>,

    /// Event source ids. May be repeated.
    #[arg(long = "source")]
    pub(crate) sources: Vec<String>,

    /// Optional source record ids for the former-spouse relationship. May be repeated.
    #[arg(long = "relationship-source")]
    pub(crate) relationship_sources: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Add a death event for one person.
#[derive(Debug, clap::Args)]
pub(crate) struct AddDeathArgs {
    /// Person slug/id.
    pub(crate) person: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Event slug. Defaults to <person-slug>-death.
    #[arg(long)]
    pub(crate) slug: Option<String>,

    /// Optional event title. When omitted, the death label is derived from the person.
    #[arg(long)]
    pub(crate) title: Option<String>,

    /// Place entity ids. May be repeated.
    #[arg(long = "place")]
    pub(crate) places: Vec<String>,

    /// Inline event location label.
    #[arg(long)]
    pub(crate) location: Option<String>,

    /// Inline event location latitude.
    #[arg(long)]
    pub(crate) latitude: Option<f64>,

    /// Inline event location longitude.
    #[arg(long)]
    pub(crate) longitude: Option<f64>,

    /// Event time/date text.
    #[arg(long)]
    pub(crate) time: Option<String>,

    /// Event date precision. Inferred from --time when omitted.
    #[arg(long)]
    pub(crate) date_precision: Option<String>,

    /// Event source ids. May be repeated.
    #[arg(long = "source")]
    pub(crate) sources: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Add a life event for one or more people.
#[derive(Debug, clap::Args)]
pub(crate) struct AddEventArgs {
    pub(crate) event_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Event type, such as birth, death, residence, marriage, migration, observation, or moment.
    #[arg(long = "type")]
    pub(crate) event_type: String,

    /// Person participant ids/slugs. May be repeated.
    #[arg(long = "person")]
    pub(crate) people: Vec<String>,

    /// Partner participant id/slug for relationship-like events such as marriage.
    #[arg(long)]
    pub(crate) partner: Option<String>,

    /// Optional event title. When omitted, common event types derive labels from participants.
    #[arg(long)]
    pub(crate) title: Option<String>,

    /// Place entity ids. May be repeated.
    #[arg(long = "place")]
    pub(crate) places: Vec<String>,

    /// Inline event location label.
    #[arg(long)]
    pub(crate) location: Option<String>,

    /// Inline event location latitude.
    #[arg(long)]
    pub(crate) latitude: Option<f64>,

    /// Inline event location longitude.
    #[arg(long)]
    pub(crate) longitude: Option<f64>,

    /// Event time/date text.
    #[arg(long)]
    pub(crate) time: Option<String>,

    /// Event date precision. Inferred from --time when omitted.
    #[arg(long)]
    pub(crate) date_precision: Option<String>,

    /// Event source ids. May be repeated.
    #[arg(long = "source")]
    pub(crate) sources: Vec<String>,

    /// Also create a spouse/partner relationship between the first person and --partner.
    #[arg(long)]
    pub(crate) create_relationship: bool,

    /// Relationship slug to use with --create-relationship.
    #[arg(long)]
    pub(crate) relationship_slug: Option<String>,

    /// Relationship kind to use with --create-relationship.
    #[arg(long, default_value = "spouse")]
    pub(crate) relationship_kind: String,

    /// Optional source record ids for the relationship created by --create-relationship. May be repeated.
    #[arg(long = "relationship-source")]
    pub(crate) relationship_sources: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a semantic event record.
#[derive(Debug, clap::Args)]
pub(crate) struct NewEventArgs {
    pub(crate) event_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Event type, such as birth, residence, observation, or moment.
    #[arg(long = "type", default_value = "observation")]
    pub(crate) event_type: String,

    /// Optional event title. When omitted, common event types derive labels from participants.
    #[arg(long)]
    pub(crate) title: Option<String>,

    /// Event participant entity ids. May be repeated.
    #[arg(long = "participant")]
    pub(crate) participants: Vec<String>,

    /// Event place entity ids. May be repeated.
    #[arg(long = "place")]
    pub(crate) places: Vec<String>,

    /// Inline event location label.
    #[arg(long)]
    pub(crate) location: Option<String>,

    /// Inline event location latitude.
    #[arg(long)]
    pub(crate) latitude: Option<f64>,

    /// Inline event location longitude.
    #[arg(long)]
    pub(crate) longitude: Option<f64>,

    /// Event time/date text.
    #[arg(long)]
    pub(crate) time: Option<String>,

    /// Event date precision. Inferred from --time when omitted.
    #[arg(long)]
    pub(crate) date_precision: Option<String>,

    /// Event source ids. May be repeated.
    #[arg(long = "source")]
    pub(crate) sources: Vec<String>,

    /// Overwrite existing generated files if present.
    #[arg(long)]
    pub(crate) force: bool,
}

/// Create a birth event for an existing person.
#[derive(Debug, clap::Args)]
pub(crate) struct NewBirthArgs {
    /// Person slug used in the person id, e.g. alex-example.
    pub(crate) person_slug: String,

    /// Workspace root. Defaults to $KLEIO_DATA_DIR, $XDG_DATA_HOME/kleio, or ~/.local/share/kleio.
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,

    /// World slug. Defaults to the workspace default world.
    #[arg(long)]
    pub(crate) world: Option<String>,

    /// Person display name used in the event title.
    #[arg(long)]
    pub(crate) person_name: String,

    /// Optional birth date, such as 1900-01-01.
    #[arg(long)]
    pub(crate) birth_date: Option<String>,

    /// Optional birth location, stored inline on the birth event.
    #[arg(long)]
    pub(crate) birth_location: Option<String>,

    /// Optional birth latitude, stored inline on the birth event.
    #[arg(long)]
    pub(crate) birth_latitude: Option<f64>,

    /// Optional birth longitude, stored inline on the birth event.
    #[arg(long)]
    pub(crate) birth_longitude: Option<f64>,

    /// Overwrite existing generated file if present.
    #[arg(long)]
    pub(crate) force: bool,
}
