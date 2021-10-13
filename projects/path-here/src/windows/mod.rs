use std::process::Command;

use find_target::this_directory;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

use crate::{Runner, XResult};

impl Runner {
    pub fn run(&self) -> XResult {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (env, _) = hkcu.create_subkey("Environment")?;
        let here = this_directory()?.to_string_lossy().to_string();
        let here = &here[4..here.len()];
        let path: String = env.get_value("PATH")?;
        if path.contains(here) {
            println!("This directory already in $PATH");
        } else {
            let path = format!("{};{}", path, here);
            env.set_value("PATH", &path)?;
            println!("`{}` added to $PATH", here);
        }
        pause()
    }
}

fn pause() -> XResult {
    // println!("Press any key to continue...");
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    Ok(())
}