use std::fs;
use tempfile::TempDir;

use goji_cli::error::GojiError;
use goji_cli::package::{PackageJson, PackageManager};

fn temp_project(lockfile: &str, script_names: &[&str]) -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join(lockfile), "").unwrap();

    let entries = script_names
        .iter()
        .map(|n| format!("\"{n}\": \"echo {n}\""))
        .collect::<Vec<_>>()
        .join(",");
    fs::write(
        dir.path().join("package.json"),
        format!("{{\"scripts\":{{{entries}}}}}"),
    )
    .unwrap();

    dir
}

#[test]
fn pnpm_project_detects_manager_and_loads_sorted_scripts() {
    let dir = temp_project("pnpm-lock.yaml", &["build", "test", "dev"]);

    let manager = PackageManager::detect(dir.path());
    assert!(matches!(manager, PackageManager::Pnpm));

    let scripts = PackageJson::load(&dir.path().join("package.json"))
        .unwrap()
        .into_scripts()
        .unwrap();

    assert_eq!(scripts.len(), 3);
    assert_eq!(scripts[0].name, "build");
    assert_eq!(scripts[1].name, "dev");
    assert_eq!(scripts[2].name, "test");
}

#[test]
fn yarn_project_detects_correctly() {
    let dir = temp_project("yarn.lock", &["start"]);
    assert!(matches!(
        PackageManager::detect(dir.path()),
        PackageManager::Yarn
    ));
}

#[test]
fn project_without_scripts_surfaces_no_scripts_error() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("package.json"), r#"{"name":"empty"}"#).unwrap();

    let err = PackageJson::load(&dir.path().join("package.json"))
        .unwrap()
        .into_scripts()
        .unwrap_err();

    assert!(matches!(err, GojiError::NoScripts));
}

#[test]
fn missing_package_json_surfaces_io_error() {
    let dir = TempDir::new().unwrap();
    let err = PackageJson::load(&dir.path().join("package.json")).unwrap_err();
    assert!(matches!(err, GojiError::Io(_)));
}
