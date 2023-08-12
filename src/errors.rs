use thiserror::Error;

pub type RoomResult = Result<(), RoomError>;

#[derive(Debug, Error)]
pub enum RoomError {
    #[error("Room already exists")]
    AlreadyExists,
    #[error("Failed while removing room")]
    DeleteError,
}

pub type DeviceResult = Result<(), DeviceError>;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Device already exists")]
    AlreadyExists,
    #[error("Failed while removing device")]
    DeleteError,
}
