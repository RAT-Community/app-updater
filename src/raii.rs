use windows::core::Free;
use windows::Win32::Foundation::HANDLE;

pub struct HandleRaii(HANDLE);

impl HandleRaii {
    pub fn from(handle: HANDLE) -> Self {
        Self(handle)
    }

    pub fn get(&self) -> HANDLE {
        self.0
    }
}

impl Drop for HandleRaii {
    fn drop(&mut self) {
        unsafe {
            self.0.free();
        }
    }
}
