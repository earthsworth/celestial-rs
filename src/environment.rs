use std::env;


#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct SystemEnvironment {
    pub os: Os,
    pub arch: Arch,
}

impl SystemEnvironment {
    pub fn new(os: Os, arch: Arch) -> Self {
        Self { os, arch }
    }

    pub fn from_current_env() -> Self {
        Self {
            os: Os::from_current_env(),
            arch: Arch::from_current_env(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Os {
    // Unsupported platform
    Unsupported,

    // These three platforms were supported by Lunar Client
    Linux,
    Windows,
    Macos,

    // unsupported by Lunar Client, but Celestial.rs introduced patches
    FreeBsd,
    OpenBsd,
}

impl Os {
    pub fn from_current_env() -> Self {
        match env::consts::OS {
            "linux" => Self::Linux,
            "windows" => Self::Windows,
            "macos" => Self::Macos,
            "openbsd" => Self::OpenBsd,
            "freebsd" => Self::FreeBsd,
            _ => Self::Unsupported,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Arch {
    // Unsupported arch
    Unsupported,

    // Official supported by Lunar Client
    X86,
    X86_64,
    Arm64,

    // Supported by Celestial.rs
    Aarch64,
}

impl Arch {
    pub fn from_current_env() -> Self {
        match env::consts::ARCH {
            "x86" => Self::X86,
            "x86_64" => Self::X86_64,
            "arm" => Self::Arm64,
            "aarch64" => Self::Aarch64,
            _ => Self::Unsupported, // unsupported arch
        }
    }
}
