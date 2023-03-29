use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParsePackageVersionError {
    #[error("No package version specified")]
    Empty,
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

/// Parse a package version from a string.
/// Expected format: package==1.0.0
/// If the package version is not in the expected format, None is returned.
///
/// ```
/// use swimlane_cli::util::parse_package_version;
///
/// assert_eq!(parse_package_version("package"), None);
/// assert_eq!(parse_package_version("package=="), None);
/// assert_eq!(parse_package_version("package==1.0.0"), Some("1.0.0"));
/// ```
///
pub fn parse_package_version(package_version: &str) -> Result<&str, ParsePackageVersionError> {
    let package_version = package_version.trim();
    if package_version.is_empty() {
        return Err(ParsePackageVersionError::Empty);
    }

    let split = package_version.split("==").collect::<Vec<&str>>();
    if split.len() < 2 || (split.len() == 2 && split[1].is_empty()) {
        return Err(ParsePackageVersionError::InvalidFormat(
            package_version.to_string(),
        ));
    }
    Ok(split[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_version() {
        assert_eq!(
            parse_package_version(""),
            Err(ParsePackageVersionError::Empty)
        );
        assert_eq!(
            parse_package_version("package"),
            Err(ParsePackageVersionError::InvalidFormat(
                "package".to_string()
            ))
        );
        assert_eq!(
            parse_package_version("package=="),
            Err(ParsePackageVersionError::InvalidFormat(
                "package==".to_string()
            ))
        );
        assert_eq!(parse_package_version("package==1.0.0"), Ok("1.0.0"));
    }
}
