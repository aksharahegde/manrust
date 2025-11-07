use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

pub fn get_available_sections() -> Vec<u8> {
    let mut sections = Vec::new();
    for i in 1..=9 {
        let man_dir = format!("/usr/share/man/man{}", i);
        if fs::read_dir(&man_dir).is_ok() {
            sections.push(i);
        }
    }
    sections
}

pub fn discover_man_pages(section: Option<u8>) -> Result<Vec<String>> {
    let mut commands = Vec::new();
    let sections = if let Some(sec) = section {
        vec![sec]
    } else {
        get_available_sections()
    };

    for section_num in sections {
        let man_dir = format!("/usr/share/man/man{}", section_num);
        
        if let Ok(entries) = fs::read_dir(&man_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    
                    if let Some(name_str) = file_name.to_str() {
                        if name_str.ends_with(".gz") {
                            if let Some(cmd) = name_str.split('.').next() {
                                commands.push(cmd.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    commands.sort();
    commands.dedup();
    Ok(commands)
}

pub fn fetch_man_page(cmd: &str, section: Option<u8>) -> Result<String> {
    let man_cmd = if let Some(sec) = section {
        format!("man {} {}", sec, cmd)
    } else {
        format!("man {}", cmd)
    };

    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("{} | col -b", man_cmd))
        .output()
        .with_context(|| format!("Failed to execute man command for: {}", cmd))?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .with_context(|| format!("Failed to parse man page output for: {}", cmd))
    } else {
        Ok(format!("Man page for '{}' not found.", cmd))
    }
}

