use console::style;

use crate::error::GojiError;
use crate::package::{PackageManager, Script};

pub fn render_intro() -> Result<(), GojiError> {
    cliclack::intro(format!("{}", style(" 𝚐𝚘𝚓𝚒 ").color256(160).bold().bright()))?;
    Ok(())
}

pub fn render_outro(manager: &PackageManager, script: &Script) -> Result<(), GojiError> {
    cliclack::outro(format!(
        "{} {} {}",
        style("✓ running").green().bold(),
        style(manager.cmd()).cyan().bold(),
        style(&script.name).dim(),
    ))?;
    Ok(())
}
