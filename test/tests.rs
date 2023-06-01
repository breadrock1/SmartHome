#[cfg(test)]
mod tests {
    extern crate smarthome;
    use smarthome::*;
    use std::error::Error;

    fn init_environment() -> Result<Room, Box<dyn Error>> {
        let kitchen_therm = SmartSocket::new("kitchen_thermometer".to_string());
        let kitchen_socket = SmartThermometer::new("kitchen_socket".to_string());
        let devices: Vec<Box<dyn SocketTrait>> =
            vec![Box::new(kitchen_socket), Box::new(kitchen_therm)];
        let mut kitchen_room = Room::new("kitchen".to_string());
        kitchen_room.add_devices(devices).unwrap();
        Ok(kitchen_room)
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
        let mut house = SmartHouse::new("Moscow".to_string());
        house.add_room(Box::new(kitchen_room)).unwrap();
        let house_rooms = get_all_rooms(&house);
        assert_eq!(house_rooms.len(), 1);
    }

    #[test]
    pub fn test_get_provider_report_own() {
        let socket = SmartSocket::new("socket 101".to_string());
        let devices: Vec<Box<dyn SocketTrait>> = vec![Box::new(socket.clone())];
        let mut kitchen_room = Room::new("kitchen".to_string());
        kitchen_room.add_devices(devices).unwrap();

        let mut house = SmartHouse::new("Moscow".to_string());
        house.add_room(Box::new(kitchen_room)).unwrap();

        let info_provider = OwningDeviceInfoProvider { socket };
        let report_result = house.create_report(&info_provider);

        let compare = "Room: kitchen -> device - socket id: socket 101; ";
        assert_eq!(report_result, String::from(compare));
    }

    #[test]
    pub fn test_get_provider_report_borrow() {
        let socket = SmartSocket::new("socket 101".to_string());
        let thermometer = SmartThermometer::new("thermometer 101".to_string());
        let devices: Vec<Box<dyn SocketTrait>> =
            vec![Box::new(socket.clone()), Box::new(thermometer.clone())];
        let mut kitchen_room = Room::new("kitchen".to_string());
        kitchen_room.add_devices(devices).unwrap();

        let mut house = SmartHouse::new("Moscow".to_string());
        house.add_room(Box::new(kitchen_room)).unwrap();

        let info_provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermometer: &thermometer,
        };
        let report_result = house.create_report(&info_provider);

        let compare =
            "Room: kitchen -> device - socket id: socket 101; device - thermometer id: thermometer 101, value: 0; ";
        assert_eq!(report_result, String::from(compare));
    }
}
