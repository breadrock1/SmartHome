use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    DeviceExists,
    RoomExists,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::DeviceExists => write!(f, "Device already exists"),
            CustomError::RoomExists => write!(f, "Room already exists"),
        }
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        match *self {
            CustomError::DeviceExists => "Device already exists",
            CustomError::RoomExists => "Room already exists",
        }
    }
}
