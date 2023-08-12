#[cfg(test)]
mod tests {
    extern crate smarthome;
    use smarthome::*;
    use smarthome::sockets::smart_sockets::SocketType;

    fn init_environment() -> Option<Room> {
        let kitchen_therm = SmartSocket::new("kitchen_thermometer".to_string());
        let kitchen_socket = SmartThermometer::new("kitchen_socket".to_string());
        let devices: Vec<SocketType> =
            vec![SocketType::from(kitchen_socket), SocketType::from(kitchen_therm)];
        let mut kitchen_room = Room::new("kitchen".to_string()).unwrap();
        kitchen_room.add_devices(devices).unwrap();
        Some(kitchen_room)
    }

    #[test]
    pub fn test_get_room_devices() {
        let kitchen_room = init_environment().unwrap();
        let kitchen_devices = get_room_devices(&kitchen_room);
        assert_eq!(kitchen_devices.len(), 2);
    }

    #[test]
    pub fn test_get_all_rooms() {
        let kitchen_room = init_environment().unwrap();
        let mut house = SmartHouse::new("Moscow");
        house.add_room(kitchen_room).unwrap();
        let house_rooms = get_all_rooms(&house);
        assert_eq!(house_rooms.len(), 1);
    }

    #[test]
    pub fn test_get_provider_report_own() {
        let socket = SmartSocket::new("socket 101".to_string());
        let devices: Vec<SocketType> = vec![SocketType::from(socket.clone())];
        let mut kitchen_room = Room::new("kitchen".to_string()).unwrap();
        kitchen_room.add_devices(devices).unwrap();

        let mut house = SmartHouse::new("Moscow");
        house.add_room(kitchen_room).unwrap();

        let info_provider = OwningDeviceInfoProvider { socket };
        let report_result = house.create_report(&info_provider);

        let compare = "Room: kitchen -> device - socket id: socket 101; ";
        assert_eq!(report_result, String::from(compare));
    }

    #[test]
    pub fn test_get_provider_report_borrow() {
        let socket = SmartSocket::new("socket 101".to_string());
        let thermometer = SmartThermometer::new("thermometer 101".to_string());
        let devices: Vec<SocketType> =
            vec![SocketType::from(socket.clone()), SocketType::from(thermometer.clone())];
        let mut kitchen_room = Room::new("kitchen".to_string()).unwrap();
        kitchen_room.add_devices(devices).unwrap();

        let mut house = SmartHouse::new("Moscow");
        house.add_room(kitchen_room).unwrap();

        let info_provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermometer: &thermometer,
        };
        let report_result = house.create_report(&info_provider);

        assert!(report_result.contains("Room: kitchen"));
        assert!(report_result.contains("device - socket id: socket 101;"));
        assert!(report_result.contains("device - thermometer id: thermometer 101, value: 0;"));
    }
}
