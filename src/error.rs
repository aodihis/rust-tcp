use thiserror::Error;
#[derive(Error, Debug)]
pub enum TcpError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid packet format: {0}")]
    InvalidPacket(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition {
        from: crate::connection::state::TcpState,
        to: crate::connection::state::TcpState,
    },
}

pub type Result<T> = std::result::Result<T, TcpError>;