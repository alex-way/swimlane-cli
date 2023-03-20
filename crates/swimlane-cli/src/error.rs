use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwimlaneCliError {
    #[error("Swimlane error")]
    SwimlaneError(#[from] swimlane::error::SwimlaneClientError),
    #[error("Error occurred whilst trying to create a new Swimlane migrator instance")]
    SwimlaneMigratorCreationError(#[from] swimlane_migrator::SwimlaneMigratorNewError),
    #[error("Swimlane migrator error")]
    SwimlaneMigratorError(#[from] swimlane_migrator::SwimlaneMigratorError),
    #[error("No package or requirements file specified")]
    NoPackageOrRequirementsFileSpecified,
    #[error("Package {0} does not exist")]
    PackageDoesNotExist(String),
    #[error("Generic error")]
    GenericError(#[source] Box<dyn std::error::Error>),
}
