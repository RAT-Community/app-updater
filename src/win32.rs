use crate::raii::HandleRaii;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::time::Duration;
use windows::core::PCWSTR;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, PROCESSENTRY32W,
    TH32CS_SNAPPROCESS,
};

pub fn is_pid_running(pid: u32) -> anyhow::Result<bool> {
    unsafe {
        let snapshot = HandleRaii::from(CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?);
        let mut pe32 = PROCESSENTRY32::default();
        pe32.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32First(snapshot.get(), &mut pe32).is_err() {
            return Ok(false);
        }

        loop {
            if pe32.th32ProcessID == pid {
                return Ok(true);
            }
            if Process32Next(snapshot.get(), &mut pe32).is_err() {
                break;
            }
        }

        Ok(false)
    }
}

pub fn wait_for_process_exit(pid: u32, time: Duration) -> anyhow::Result<()> {
    while is_pid_running(pid)? {
        std::thread::sleep(time);
    }
    Ok(())
}

pub fn w(s: &str) -> PCWSTR {
    let wide: Vec<u16> = OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    PCWSTR(wide.as_ptr())
}

pub struct WinStr {
    buffer: Vec<u16>,
}

impl WinStr {
    pub fn from_str(s: &str) -> Self {
        let buffer: Vec<u16> = OsStr::new(s)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        Self { buffer }
    }
    pub fn as_ptr(&self) -> PCWSTR {
        PCWSTR(self.buffer.as_ptr())
    }
}
