#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbosityMode {
    Compact,
    Verbose,
}

pub fn default_verbosity_for_size(width: u16, height: u16) -> VerbosityMode {
    if width < 100 || height < 28 {
        VerbosityMode::Compact
    } else {
        VerbosityMode::Verbose
    }
}

impl VerbosityMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Compact => Self::Verbose,
            Self::Verbose => Self::Compact,
        }
    }
}
