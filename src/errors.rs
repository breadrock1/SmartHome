use thiserror::Error;

pub type RoomResult = Result<(), RoomError>;

#[derive(Debug, Error)]
pub enum RoomError {
    #[error("Room already exists")]
    AlreadyExists,
}

pub type DeviceResult = Result<(), DeviceError>;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Device already exists")]
    AlreadyExists,
}
