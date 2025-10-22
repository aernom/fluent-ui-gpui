#[derive(Debug, PartialEq, Eq)]
pub enum Platform {
    Windows,
    Mac,
    Linux,
}

impl Platform {
    #[cfg(target_os = "windows")]
    pub fn current() -> Self {
        Self::Windows
    }

    #[cfg(target_os = "macos")]
    pub fn current() -> Self {
        Self::Mac
    }

    #[cfg(target_os = "linux")]
    pub fn current() -> Self {
        Self::Linux
    }
}
