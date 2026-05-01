use std::process::Command;

use crate::error::GojiError;
use crate::package::{PackageManager, Script};

pub struct Runner {
    manager: PackageManager,
}

impl Runner {
    pub fn new(manager: PackageManager) -> Self {
        Self { manager }
    }

    fn build_command(&self, script: &Script) -> Command {
        let mut cmd = Command::new(self.manager.cmd());
        cmd.args(self.manager.run_args()).arg(&script.name);
        cmd
    }

    /// Returns the child's exit code. `unwrap_or(1)` handles the rare case
    /// where the OS kills the process with a signal (no numeric exit code).
    pub fn run(&self, script: &Script) -> Result<i32, GojiError> {
        let status = self.build_command(script).status()?;
        Ok(status.code().unwrap_or(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_manager() {
        let _runner = Runner::new(PackageManager::Pnpm);
    }

    #[test]
    fn build_command_uses_correct_program() {
        let runner = Runner::new(PackageManager::Pnpm);
        let cmd = runner.build_command(&Script::new("build"));
        assert_eq!(cmd.get_program(), "pnpm");
    }

    #[test]
    fn build_command_includes_run_and_script_name() {
        let runner = Runner::new(PackageManager::Npm);
        let cmd = runner.build_command(&Script::new("test"));
        let args: Vec<_> = cmd.get_args().collect();
        assert_eq!(args[0], "run");
        assert_eq!(args[1], "test");
    }

    #[test]
    fn build_command_varies_by_manager() {
        for (pm, expected) in [
            (PackageManager::Npm, "npm"),
            (PackageManager::Yarn, "yarn"),
            (PackageManager::Pnpm, "pnpm"),
            (PackageManager::Bun, "bun"),
        ] {
            let runner = Runner::new(pm);
            let cmd = runner.build_command(&Script::new("start"));
            assert_eq!(cmd.get_program(), expected);
        }
    }

    #[test]
    fn run_returns_nonzero_or_io_error_for_unknown_script() {
        let runner = Runner::new(PackageManager::Npm);
        match runner.run(&Script::new("__goji_test_script_does_not_exist__")) {
            Ok(code) => assert!(code != 0),
            Err(GojiError::Io(_)) => {}
            Err(e) => panic!("unexpected error variant: {e}"),
        }
    }
}
