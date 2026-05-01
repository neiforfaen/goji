use thiserror::Error;

#[derive(Debug, Error)]
pub enum GojiError {
    #[error("Could not find a package.json in the current directory or any parent")]
    NoPackageRoot,

    #[error("No scripts are defined in package.json")]
    NoScripts,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse package.json: {0}")]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_package_root_message() {
        assert_eq!(
            GojiError::NoPackageRoot.to_string(),
            "Could not find a package.json in the current directory or any parent"
        );
    }

    #[test]
    fn no_scripts_message() {
        assert_eq!(
            GojiError::NoScripts.to_string(),
            "No scripts are defined in package.json"
        );
    }

    #[test]
    fn io_error_wraps_source() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "nope");
        let e = GojiError::Io(io);
        assert!(e.to_string().contains("IO error"));
    }

    #[test]
    fn json_error_wraps_source() {
        let json_err = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
        let e = GojiError::Json(json_err);
        assert!(e.to_string().contains("parse"));
    }

    #[test]
    fn from_io_error_conversion() {
        let io = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let e: GojiError = io.into();
        assert!(matches!(e, GojiError::Io(_)));
    }

    #[test]
    fn from_json_error_conversion() {
        let json_err = serde_json::from_str::<serde_json::Value>("bad").unwrap_err();
        let e: GojiError = json_err.into();
        assert!(matches!(e, GojiError::Json(_)));
    }

    #[test]
    fn debug_format_includes_variant_name() {
        assert!(format!("{:?}", GojiError::NoPackageRoot).contains("NoPackageRoot"));
        assert!(format!("{:?}", GojiError::NoScripts).contains("NoScripts"));
    }
}
