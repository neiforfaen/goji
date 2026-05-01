use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use super::script::Script;
use crate::error::GojiError;

#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub scripts: Option<HashMap<String, String>>,
}

impl PackageJson {
    pub fn load(path: &Path) -> Result<Self, GojiError> {
        let content = std::fs::read_to_string(path)?;
        let package: Self = serde_json::from_str(&content)?;
        Ok(package)
    }

    pub fn into_scripts(self) -> Result<Vec<Script>, GojiError> {
        let map = self.scripts.ok_or(GojiError::NoScripts)?;

        if map.is_empty() {
            return Err(GojiError::NoScripts);
        }

        let mut scripts: Vec<Script> = map.into_keys().map(Script::new).collect();

        scripts.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(scripts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_temp(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "{content}").unwrap();
        f
    }

    #[test]
    fn load_parses_valid_scripts() {
        let f = write_temp(r#"{"scripts":{"build":"tsc","test":"jest"}}"#);
        let pkg = PackageJson::load(f.path()).unwrap();
        assert_eq!(pkg.scripts.unwrap().len(), 2);
    }

    #[test]
    fn load_succeeds_without_scripts_key() {
        let f = write_temp(r#"{"name":"my-app"}"#);
        let pkg = PackageJson::load(f.path()).unwrap();
        assert!(pkg.scripts.is_none());
    }

    #[test]
    fn load_returns_io_error_for_missing_file() {
        let err = PackageJson::load(Path::new("/no/such/package.json")).unwrap_err();
        assert!(matches!(err, GojiError::Io(_)));
    }

    #[test]
    fn load_returns_json_error_for_invalid_json() {
        let f = write_temp("not json at all {{{");
        let err = PackageJson::load(f.path()).unwrap_err();
        assert!(matches!(err, GojiError::Json(_)));
    }

    #[test]
    fn into_scripts_returns_alphabetically_sorted_names() {
        let pkg = PackageJson {
            scripts: Some(HashMap::from([
                ("zebra".into(), "z".into()),
                ("alpha".into(), "a".into()),
                ("middle".into(), "m".into()),
            ])),
        };
        let scripts = pkg.into_scripts().unwrap();
        assert_eq!(scripts[0].name, "alpha");
        assert_eq!(scripts[1].name, "middle");
        assert_eq!(scripts[2].name, "zebra");
    }

    #[test]
    fn into_scripts_single_entry() {
        let pkg = PackageJson {
            scripts: Some(HashMap::from([("build".into(), "tsc".into())])),
        };
        let scripts = pkg.into_scripts().unwrap();
        assert_eq!(scripts.len(), 1);
        assert_eq!(scripts[0].name, "build");
    }

    #[test]
    fn into_scripts_errors_when_scripts_is_none() {
        let err = PackageJson { scripts: None }.into_scripts().unwrap_err();
        assert!(matches!(err, GojiError::NoScripts));
    }

    #[test]
    fn into_scripts_errors_when_scripts_map_is_empty() {
        let err = PackageJson {
            scripts: Some(HashMap::new()),
        }
        .into_scripts()
        .unwrap_err();
        assert!(matches!(err, GojiError::NoScripts));
    }
}
