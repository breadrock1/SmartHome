extern crate smarthome;

use crate::smarthome::*;
use smarthome::sockets::smart_sockets::SocketType;

fn main() -> Result<(), anyhow::Error> {
    let kitchen_socket = SmartSocket::new("kitchen_socket");
    let kitchen_therm = SmartThermometer::new("kitchen_thermometer");
    let devices: Vec<SocketType> = vec![
        SocketType::from(kitchen_socket.clone()),
        SocketType::from(kitchen_therm.clone()),
    ];
    let mut kitchen_room = Room::new("kitchen");
    kitchen_room.add_devices(devices).unwrap();

    let thermometer = SmartSocket::new("new_socket");
    kitchen_room
        .add_device(SocketType::from(thermometer))
        .unwrap();

    let mut house = SmartHouse::new("Moscow");
    house.add_room(kitchen_room).unwrap();

    let info_provider = BorrowingDeviceInfoProvider {
        socket: &kitchen_socket,
        thermometer: &kitchen_therm,
    };

    println!("{}", house.create_report(&info_provider));
    Ok(())
}
