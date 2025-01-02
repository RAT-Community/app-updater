#![cfg(windows)]
use anyhow::Context;
use app_updater::win32::WinStr;
use app_updater::{start_update, UpdateInfo};
use std::{env, process::exit};
use windows::core::w;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

fn main() {
    let update_info = match parse_args() {
        Ok(info) => info,
        Err(e) => {
            let message = format!("参数错误: {}", e);
            unsafe {
                MessageBoxW(
                    None,
                    WinStr::from_str(&message).as_ptr(),
                    w!("更新失败"),
                    MB_OK,
                );
            }
            exit(1);
        }
    };

    match start_update(&update_info) {
        Ok(_) => exit(0),
        Err(e) => {
            let message = format!("错误信息:\n{}", e);
            unsafe {
                MessageBoxW(
                    None,
                    WinStr::from_str(&message).as_ptr(),
                    w!("更新失败"),
                    MB_OK,
                );
            }
            exit(1);
        }
    }
}

fn parse_args() -> anyhow::Result<UpdateInfo> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        return Err(anyhow::anyhow!("Invalid number of arguments"));
    }

    let download_path = &args[1];
    let resource_path = &args[2];
    let electron_exe = &args[3];
    let electron_pid = args[4]
        .parse::<u32>()
        .context("Failed to parse Electron PID")?;

    Ok(UpdateInfo {
        download_path: download_path.clone(),
        resource_path: resource_path.clone(),
        electron_exe: electron_exe.clone(),
        pid: electron_pid,
    })
}
