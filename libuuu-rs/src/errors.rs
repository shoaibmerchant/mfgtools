use std::fmt;

#[derive(Debug, Default, Clone, Copy)]
pub enum UUUErrorCodes {
    #[default]
    UnknownError,
    VersionError,
    GetLastErrError,
    ScriptError,
    CommandError,
}

impl fmt::Display for UUUErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UUUErrorCodes::UnknownError => write!(f, "UnknownError"),
            UUUErrorCodes::VersionError => write!(f, "VersionError"),
            UUUErrorCodes::GetLastErrError => write!(f, "GetLastErrError"),
            UUUErrorCodes::CommandError => write!(f, "CommandError"),
            UUUErrorCodes::ScriptError => write!(f, "ScriptError"),
        }
    }
}

#[derive(Debug)]
pub struct UUUError {
    pub code: UUUErrorCodes,
    pub message: String,
}

impl std::fmt::Display for UUUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UUUErrorCodes:(code: {:?}, message: {})",
            self.code, self.message
        )
    }
}

impl UUUError {
    pub fn new(code: UUUErrorCodes, message: String) -> Self {
        Self { code, message }
    }
}