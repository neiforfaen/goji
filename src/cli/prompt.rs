use crate::error::GojiError;
use crate::package::Script;

pub fn select_script(scripts: &[Script]) -> Result<Script, GojiError> {
    let items: Vec<(Script, String, String)> = scripts
        .iter()
        .map(|s| (s.clone(), s.name.clone(), String::new()))
        .collect();

    let selected = cliclack::select("")
        .max_rows(6)
        .filter_mode()
        .items(&items)
        .interact()?;

    Ok(selected)
}
