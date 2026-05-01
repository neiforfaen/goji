use std::process;

use goji_cli::{
    cli::{init_theme, render_intro, render_outro, select_script},
    error::GojiError,
    package::{PackageJson, PackageManager},
    runner::Runner,
};

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(e) => {
            eprintln!("goji: {e}");
            process::exit(1);
        }
    }
}

fn run() -> Result<i32, GojiError> {
    init_theme();

    let root = PackageManager::find_root()?;
    let manager = PackageManager::detect(&root);

    let package = PackageJson::load(&root.join("package.json"))?;
    let scripts = package.into_scripts()?;

    render_intro()?;
    let selected = select_script(&scripts)?;

    render_outro(&manager, &selected)?;
    Runner::new(manager).run(&selected)
}
