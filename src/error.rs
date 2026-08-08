#[derive(Debug)]
struct CliError(String);

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}

pub(crate) fn cli_error(message: impl Into<String>) -> Box<dyn std::error::Error> {
    Box::new(CliError(message.into()))
}
