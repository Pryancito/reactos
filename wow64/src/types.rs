//! Tipos comunes para WOW64

/// Tipo de resultado para WOW64
pub type Result<T> = core::result::Result<T, Error>;

/// Errores de WOW64
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Error de memoria
    OutOfMemory,
    /// Stack overflow
    StackOverflow,
    /// Stack underflow
    StackUnderflow,
    /// Dirección inválida
    InvalidAddress,
    /// Tamaño inválido
    InvalidSize,
    /// Archivo inválido
    InvalidFile,
    /// Arquitectura inválida
    InvalidArchitecture,
    /// Header inválido
    InvalidHeader,
    /// Sección inválida
    InvalidSection,
    /// Importación inválida
    InvalidImport,
    /// Exportación inválida
    InvalidExport,
    /// Error de contexto
    ContextError,
    /// Error de thunk
    ThunkError,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::OutOfMemory => write!(f, "Out of memory"),
            Error::StackOverflow => write!(f, "Stack overflow"),
            Error::StackUnderflow => write!(f, "Stack underflow"),
            Error::InvalidAddress => write!(f, "Invalid address"),
            Error::InvalidSize => write!(f, "Invalid size"),
            Error::InvalidFile => write!(f, "Invalid PE file"),
            Error::InvalidArchitecture => write!(f, "Invalid architecture (not 32-bit)"),
            Error::InvalidHeader => write!(f, "Invalid PE header"),
            Error::InvalidSection => write!(f, "Invalid section"),
            Error::InvalidImport => write!(f, "Invalid import"),
            Error::InvalidExport => write!(f, "Invalid export"),
            Error::ContextError => write!(f, "Context error"),
            Error::ThunkError => write!(f, "Thunk error"),
        }
    }
}

impl std::error::Error for Error {}
