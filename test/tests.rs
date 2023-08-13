#[cfg(test)]
mod tests {
    extern crate smarthome;
    use smarthome::sockets::smart_sockets::SocketType;
    use smarthome::*;

    fn init_environment() -> Option<Room> {
        let kitchen_therm = SmartSocket::new("kitchen_thermometer");
        let kitchen_socket = SmartThermometer::new("kitchen_socket");
        let devices: Vec<SocketType> = vec![
            SocketType::from(kitchen_socket),
            SocketType::from(kitchen_therm),
        ];
        let mut kitchen_room = Room::new("kitchen");
        kitchen_room.add_devices(devices).unwrap();
        Some(kitchen_room)
    }

    #[test]
    pub fn test_get_room_devices() {
        let kitchen_room = init_environment().unwrap();
        let kitchen_devices = kitchen_room.get_devices();
        assert_eq!(kitchen_devices.len(), 2);
    }

    #[test]
    pub fn test_get_all_rooms() {
        let kitchen_room = init_environment().unwrap();
        let mut house = SmartHouse::new("Moscow");
        house.add_room(kitchen_room).unwrap();
        let house_rooms: Vec<&Room> = house.get_rooms();
        assert_eq!(house_rooms.len(), 1);
    }

    #[test]
    pub fn test_get_provider_report_own() {
        let socket = SmartSocket::new("socket 101");
        let devices: Vec<SocketType> = vec![SocketType::from(socket.clone())];
        let mut kitchen_room = Room::new("kitchen");
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
        let socket = SmartSocket::new("socket 101");
        let thermometer = SmartThermometer::new("thermometer 101");
        let devices: Vec<SocketType> = vec![
            SocketType::from(socket.clone()),
            SocketType::from(thermometer.clone()),
        ];
        let mut kitchen_room = Room::new("kitchen");
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
