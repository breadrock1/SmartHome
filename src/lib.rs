pub mod house;
pub mod providers;
pub mod sockets;

pub use house::house::{Room, SmartHouse};
pub use providers::providers::DeviceInfoProvider;
pub use providers::providers::OwningDeviceInfoProvider;
pub use providers::providers::BorrowingDeviceInfoProvider;
pub use sockets::sockets::SocketTrait;
pub use sockets::sockets::SmartSocket;
pub use sockets::sockets::SmartThermometer;

pub fn get_all_rooms(house: &SmartHouse) -> Vec<&Room> {
    house.get_rooms()
}

pub fn get_room_devices(room: &Room) -> &Vec<Box<dyn SocketTrait>>{
    room.get_devices()
}

pub fn get_provider_report(house: &SmartHouse, provider: &dyn DeviceInfoProvider) -> String {
    house.create_report(provider)
}
