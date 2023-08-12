pub mod errors;
pub mod house;
pub mod macros;
pub mod providers;
pub mod sockets;

use std::collections::HashMap;
pub use house::smarthouse::{Room, SmartHouse};
pub use providers::info_providers::BorrowingDeviceInfoProvider;
pub use providers::info_providers::DeviceInfoProvider;
pub use providers::info_providers::OwningDeviceInfoProvider;
pub use sockets::smart_sockets::SmartSocket;
pub use sockets::smart_sockets::SmartThermometer;
pub use sockets::smart_sockets::SocketTrait;
use crate::sockets::smart_sockets::SocketType;

pub fn get_all_rooms(house: &SmartHouse) -> HashMap<String, Room> {
    house.get_rooms()
}

pub fn get_room_devices(room: &Room) -> HashMap<String, SocketType> {
    room.get_devices()
}

pub fn get_provider_report(house: &SmartHouse, provider: &dyn DeviceInfoProvider) -> String {
    house.create_report(provider)
}
