pub mod errors;
pub mod house;
pub mod providers;
pub mod sockets;

pub use house::smarthouse::{Room, SmartHouse};
pub use providers::info_providers::BorrowingDeviceInfoProvider;
pub use providers::info_providers::DeviceInfoProvider;
pub use providers::info_providers::OwningDeviceInfoProvider;
pub use sockets::smart_sockets::SmartSocket;
pub use sockets::smart_sockets::SmartThermometer;
pub use sockets::smart_sockets::SocketTrait;

pub fn get_all_rooms(house: &SmartHouse) -> Vec<&Room> {
    house.get_rooms()
}

pub fn get_room_devices(room: &Room) -> &Vec<Box<dyn SocketTrait>> {
    room.get_devices()
}

pub fn get_provider_report(house: &SmartHouse, provider: &dyn DeviceInfoProvider) -> String {
    house.create_report(provider)
}
