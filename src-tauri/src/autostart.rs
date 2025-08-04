use std::path::PathBuf;
use tauri::api::path::home_dir;
use anyhow::Result;

/// Cross-platform helper to create / delete an autostart entry.
pub enum AutoStart {
    Enable,
    Disable,
}

pub fn apply(mode: AutoStart) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        use std::fs;
        let startup = home_dir()
            .unwrap()
            .join(r"AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup")
            .join("ClipDock.lnk");
        match mode {
            AutoStart::Enable => {
                // create a shortcut (.lnk) that points to the exe
                let exe = std::env::current_exe()?;
                std::os::windows::fs::symlink_file(exe, startup)?;
            }
            AutoStart::Disable => {
                if startup.exists() { fs::remove_file(startup)?; }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs;
        let dir = home_dir().unwrap().join(".config/autostart");
        std::fs::create_dir_all(&dir)?;
        let desktop = dir.join("clipdock.desktop");
        match mode {
            AutoStart::Enable => {
                let exe = std::env::current_exe()?;
                std::fs::write(
                    &desktop,
                    format!(
                        "[Desktop Entry]\nType=Application\nName=ClipDock\nExec={}\n",
                        exe.display()
                    ),
                )?;
            }
            AutoStart::Disable => {
                if desktop.exists() { fs::remove_file(desktop)?; }
            }
        }
    }

    Ok(())
}
