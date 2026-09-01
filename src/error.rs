use std::io::ErrorKind;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Error {
    RootNodeRemoval,

    EventLoopRecreation,
    EventLoopExecution,
    FailedToSendProxy,

    FontRefCreation,

    NoFileSystem,
    InvalidDataSize,
    UnsupportedFileFormat,
    FailedToExpand,
    CannotGetDimensions,
    InvalidPNG,
    InvalidJPEG,

    DeviceLost,
    VertexOverflow(u64, u64, u64),
    IndexOverflow(u64, u64, u64),
    RequestingAdapter(String),
    RequestingDevice(String),
    SurfaceCreation(String),

    IoError(ErrorKind),
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err.kind())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RootNodeRemoval => {
                write!(f, "Root node cannot be removed")
            }
            Error::EventLoopRecreation => {
                write!(f, "Event loop cannot be recreated")
            }
            Error::EventLoopExecution => {
                write!(f, "Event loop cannot be executed")
            }
            Error::FailedToSendProxy => {
                write!(f, "Failed to fire event loop proxy")
            }
            Error::FontRefCreation => {
                write!(f, "Font reference cannot be created")
            }
            Error::NoFileSystem => {
                write!(f, "wasm32 does not support file systems")
            }
            Error::InvalidDataSize => {
                write!(f, "Invalid data length, minimium length is 8 bytes")
            }
            Error::UnsupportedFileFormat => {
                write!(
                    f,
                    "Unsupported file format, only pngs and jpegs are supported"
                )
            }
            Error::FailedToExpand => {
                write!(f, "PNG failed to run EXPAND on file")
            }
            Error::CannotGetDimensions => {
                write!(f, "Failed to get dimensions of image")
            }
            Error::InvalidPNG => {
                write!(f, "Invalid PNG file")
            }
            Error::InvalidJPEG => {
                write!(f, "Invalid JPEG file")
            }
            Error::DeviceLost => {
                write!(f, "Device lost, cannot continue")
            }
            Error::VertexOverflow(id, req, cap) => {
                write!(
                    f,
                    "Sprite index buffer overflow while rendering {}: required {} bytes, capacity {} bytes",
                    id, req, cap
                )
            }
            Error::IndexOverflow(id, req, cap) => {
                write!(
                    f,
                    "Sprite index buffer overflow while rendering {}: required {} bytes, capacity {} bytes",
                    id, req, cap
                )
            }
            Error::RequestingAdapter(str) => {
                write!(f, "Failed requesting adapter: {}", str)
            }
            Error::RequestingDevice(str) => {
                write!(f, "Failed requesting device: {}", str)
            }
            Error::SurfaceCreation(str) => {
                write!(f, "Failed creating surface: {}", str)
            }
            Error::IoError(kind) => {
                write!(f, "I/O error: {kind}")
            }
        }
    }
}
