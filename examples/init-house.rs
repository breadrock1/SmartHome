extern crate smarthome;

use smarthome::sockets::smart_sockets::SocketType;
use crate::smarthome::*;

fn main() -> Result<(), anyhow::Error> {
    let kitchen_socket = SmartSocket::new("kitchen_socket".to_string());
    let kitchen_therm = SmartThermometer::new("kitchen_thermometer".to_string());
    let devices: Vec<SocketType> = vec![
        SocketType::from(kitchen_socket.clone()),
        SocketType::from(kitchen_therm.clone()),
    ];
    let mut kitchen_room = Room::new("kitchen".to_string()).unwrap();
    kitchen_room.add_devices(devices).unwrap();

    let new_therm = SmartSocket::new("new_socket".to_string());
    let new_therm = SocketType::from(new_therm.clone());
    kitchen_room.add_device(&new_therm).unwrap();

    let mut house = SmartHouse::new("Moscow".to_string()).unwrap();
    house.add_room(Box::new(kitchen_room)).unwrap();

    let info_provider = BorrowingDeviceInfoProvider {
        socket: &kitchen_socket,
        thermometer: &kitchen_therm,
    };
    let report_result = house.create_report(&info_provider);
    println!("{}", report_result);
    Ok(())
}
