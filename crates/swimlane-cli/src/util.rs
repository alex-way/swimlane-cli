pub fn parse_package_version(package_version: &str) -> Option<&str> {
    let package_version = package_version.trim();
    if package_version.is_empty() {
        return None;
    }

    let split = package_version.split("==").collect::<Vec<&str>>();
    if split.len() < 2 || (split.len() == 2 && split[1].is_empty()) {
        return None;
    }
    Some(split[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_version() {
        assert_eq!(parse_package_version("package"), None);
        assert_eq!(parse_package_version("package=="), None);
        assert_eq!(parse_package_version("package==1.0.0"), Some("1.0.0"));
    }
}
