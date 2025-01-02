pub mod raii;
pub mod win32;

use anyhow::Context;
use std::path::Path;
use std::process::Command;

pub struct UpdateInfo {
    pub download_path: String,
    pub resource_path: String,
    pub electron_exe: String,
    pub pid: u32,
}

pub fn start_update(update_info: &UpdateInfo) -> anyhow::Result<()> {
    win32::wait_for_process_exit(update_info.pid, std::time::Duration::from_secs(10))
        .context("Failed to wait for Electron process exit")?;
    replace_files(&update_info.download_path, &update_info.resource_path)?;
    start_electron(&update_info.electron_exe);
    Ok(())
}

fn replace_files(download_path: &str, resource_path: &str) -> anyhow::Result<()> {
    let download_dir = Path::new(download_path);
    let resource_dir = Path::new(resource_path);

    if !download_dir.exists() {
        eprintln!("Download path does not exist: {}", download_path);
        return Err(anyhow::anyhow!("Download path does not exist"));
    }

    if !resource_dir.exists() {
        eprintln!("Resource path does not exist: {}", resource_path);
        return Err(anyhow::anyhow!("Resource path does not exist"));
    }

    copy_dir_r(download_dir, resource_dir)?;
    Ok(())
}

fn start_electron(electron_exe: &str) {
    Command::new(electron_exe)
        .spawn()
        .expect("Failed to start Electron");
}

fn copy_dir_r(src: &Path, dst: &Path) -> anyhow::Result<()> {
    if !src.is_dir() {
        return Err(anyhow::anyhow!("Source is not a directory"));
    }

    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let dst_path = dst.join(entry_name);

        if entry_path.is_dir() {
            copy_dir_r(&entry_path, &dst_path)?;
        } else {
            std::fs::copy(&entry_path, &dst_path)?;
        }
    }

    Ok(())
}
