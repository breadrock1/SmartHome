pub mod house;
pub mod providers;
pub mod sockets;

use house::house::{Room, SmartHouse};
use providers::providers::DeviceInfoProvider;
use sockets::sockets::{SmartSocket, SmartThermometer, SocketTrait};
use std::error::Error;

pub fn create_house(address: String) -> Result<SmartHouse, Box<dyn Error>> {
    Ok(SmartHouse::new(address))
}

pub fn create_room(name: String) -> Result<Room, Box<dyn Error>> {
    Ok(Room::new(name))
}

pub fn create_device<T: SocketTrait>(dev_id: String) -> Result<T, Box<dyn Error>> {
    Ok(T::new(dev_id))
}

pub fn append_devices<T: SocketTrait>(room: &mut Room, devices: Vec<T>) {
    room.add_devices(devices)
}

pub fn append_room(house: &mut SmartHouse, room: Room) {
    house.add_room(room)
}

pub fn append_rooms(house: &mut SmartHouse, rooms: Vec<Room>) {
    for room in rooms {
        house.add_room(room);
    }
}
