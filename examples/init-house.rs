extern crate smarthome;
use crate::smarthome::*;


fn main() {
    let kitchen_therm = SmartSocket::new("kitchen_thermometer".to_string());
    let kitchen_socket = SmartThermometer::new("kitchen_socket".to_string());
    let devices = vec![Box::new(kitchen_socket), Box::new(kitchen_therm)];
    let mut kitchen_room = Room::new("kitchen".to_string());
    &kitchen_room.add_devices(devices);

    let mut house = SmartHouse::new("Moscow".to_string());
    house.add_room(Box::new(kitchen_room));
}
