pub mod devices;
pub mod house;
pub mod macros;

use std::collections::HashMap;

use devices::providers::{OwningDeviceInfoProvider, BorrowingDeviceInfoProvider};
use devices::sockets::{SmartSocket, SmartThermometer};
use house::premises::{Room, SmartHouse};
use macros::dict;


pub fn create_house(address: &String) -> SmartHouse {
    SmartHouse::new(
        "Moscow, Arbatskaya, 10, 1".to_string(),
        dict!()
    )
}

pub fn create_room(name: String, devices: Vec<String>) -> Room {
    Room::new(name, devices)
}

pub fn append_room(house: &mut SmartHouse, room: &'static Room) {
    let room_name = &room.name;
    house.rooms.insert(room_name.to_string(), room);
}

pub fn append_rooms(house: &mut SmartHouse, rooms: Vec<&Room>) {
    rooms.into_iter()
        .for_each(|room| {
            house.rooms.insert(room.name.clone(), room);
        });
}


