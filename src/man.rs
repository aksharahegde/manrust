use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

pub fn discover_man_pages() -> Result<Vec<String>> {
    let mut commands = Vec::new();
    let man_dir = "/usr/share/man/man1";

    let entries = fs::read_dir(man_dir)
        .with_context(|| format!("Failed to read directory: {}", man_dir))?;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let file_name = entry.file_name();
        
        if let Some(name_str) = file_name.to_str() {
            if name_str.ends_with(".gz") {
                if let Some(cmd) = name_str.split('.').next() {
                    commands.push(cmd.to_string());
                }
            }
        }
    }

    commands.sort();
    commands.dedup();
    Ok(commands)
}

pub fn fetch_man_page(cmd: &str) -> Result<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("man {} | col -b", cmd))
        .output()
        .with_context(|| format!("Failed to execute man command for: {}", cmd))?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .with_context(|| format!("Failed to parse man page output for: {}", cmd))
    } else {
        Ok(format!("Man page for '{}' not found.", cmd))
    }
}

