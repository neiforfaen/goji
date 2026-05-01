use std::path::{Path, PathBuf};

use crate::error::GojiError;

#[derive(Debug, Clone)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl PackageManager {
    pub fn find_root() -> Result<PathBuf, GojiError> {
        let cwd = std::env::current_dir()?;
        Self::find_root_from(&cwd)
    }

    fn find_root_from(start: &Path) -> Result<PathBuf, GojiError> {
        let markers = [
            "pnpm-lock.yaml",
            "yarn.lock",
            "bun.lockb",
            "package-lock.json",
            ".git",
        ];

        for dir in start.ancestors() {
            if markers.iter().any(|m| dir.join(m).exists()) {
                return Ok(dir.to_path_buf());
            }
        }

        Err(GojiError::NoPackageRoot)
    }

    pub fn detect(root: &Path) -> Self {
        if root.join("pnpm-lock.yaml").exists() {
            Self::Pnpm
        } else if root.join("yarn.lock").exists() {
            Self::Yarn
        } else if root.join("bun.lockb").exists() {
            Self::Bun
        } else {
            Self::Npm
        }
    }

    pub fn cmd(&self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Yarn => "yarn",
            Self::Pnpm => "pnpm",
            Self::Bun => "bun",
        }
    }

    pub fn run_args(&self) -> &'static [&'static str] {
        match self {
            Self::Npm => &["run"],
            Self::Yarn => &["run"],
            Self::Pnpm => &["run"],
            Self::Bun => &["run"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_root(files: &[&str]) -> TempDir {
        let dir = TempDir::new().unwrap();
        for &file in files {
            fs::write(dir.path().join(file), "").unwrap();
        }
        dir
    }

    #[test]
    fn detect_pnpm_by_lockfile() {
        let dir = make_root(&["pnpm-lock.yaml"]);
        assert!(matches!(
            PackageManager::detect(dir.path()),
            PackageManager::Pnpm
        ));
    }

    #[test]
    fn detect_yarn_by_lockfile() {
        let dir = make_root(&["yarn.lock"]);
        assert!(matches!(
            PackageManager::detect(dir.path()),
            PackageManager::Yarn
        ));
    }

    #[test]
    fn detect_bun_by_lockfile() {
        let dir = make_root(&["bun.lockb"]);
        assert!(matches!(
            PackageManager::detect(dir.path()),
            PackageManager::Bun
        ));
    }

    #[test]
    fn detect_defaults_to_npm() {
        let dir = TempDir::new().unwrap();
        assert!(matches!(
            PackageManager::detect(dir.path()),
            PackageManager::Npm
        ));
    }

    #[test]
    fn detect_pnpm_wins_over_yarn_when_both_present() {
        let dir = make_root(&["pnpm-lock.yaml", "yarn.lock"]);
        assert!(matches!(
            PackageManager::detect(dir.path()),
            PackageManager::Pnpm
        ));
    }

    #[test]
    fn cmd_returns_correct_executable() {
        assert_eq!(PackageManager::Npm.cmd(), "npm");
        assert_eq!(PackageManager::Yarn.cmd(), "yarn");
        assert_eq!(PackageManager::Pnpm.cmd(), "pnpm");
        assert_eq!(PackageManager::Bun.cmd(), "bun");
    }

    #[test]
    fn run_args_is_run_for_all_managers() {
        let managers = [
            PackageManager::Npm,
            PackageManager::Yarn,
            PackageManager::Pnpm,
            PackageManager::Bun,
        ];
        for m in managers {
            assert_eq!(m.run_args(), &["run"]);
        }
    }

    #[test]
    fn find_root_from_finds_marker_in_start_dir() {
        let dir = make_root(&["pnpm-lock.yaml"]);
        let root = PackageManager::find_root_from(dir.path()).unwrap();
        assert_eq!(root, dir.path());
    }

    #[test]
    fn find_root_from_walks_up_to_ancestor() {
        let dir = make_root(&["package-lock.json"]);
        let nested = dir.path().join("a").join("b").join("c");
        fs::create_dir_all(&nested).unwrap();
        let root = PackageManager::find_root_from(&nested).unwrap();
        assert_eq!(root, dir.path());
    }

    #[test]
    fn find_root_from_errors_when_no_marker_found() {
        let dir = TempDir::new().unwrap();
        let err = PackageManager::find_root_from(dir.path()).unwrap_err();
        assert!(matches!(err, GojiError::NoPackageRoot));
    }

    #[test]
    fn clone_preserves_variant() {
        let m = PackageManager::Pnpm;
        assert_eq!(m.clone().cmd(), m.cmd());
    }

    #[test]
    fn debug_includes_variant_name() {
        assert!(format!("{:?}", PackageManager::Bun).contains("Bun"));
    }
}
