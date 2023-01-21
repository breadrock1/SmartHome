pub mod sockets {
    pub struct SmartSocket {
        pub id: String,
    }

    pub struct SmartThermometer {
        pub id: String,
        pub temperature: f32,
    }
}

pub mod providers {
    use super::sockets::{SmartSocket, SmartThermometer};

    pub struct OwningDeviceInfoProvider {
        pub socket: SmartSocket,
    }

    pub struct BorrowingDeviceInfoProvider<'a, 'b> {
        pub socket: &'a SmartSocket,
        pub thermometer: &'b SmartThermometer,
    }

    pub trait DeviceInfoProvider {
        fn status(&self, room: &str, device: &str) -> String;
    }

    impl DeviceInfoProvider for OwningDeviceInfoProvider {
        fn status(&self, room: &str, _device: &str) -> String {
            format!("Room: {}\t-> socket id: {}", room, self.socket.id)
        }
    }

    impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
        fn status(&self, room: &str, _device: &str) -> String {
            format!(
                "Room: {}\t -> socket id: {}, thermometer id: {}, value: {}\n",
                room, self.socket.id, self.thermometer.id, self.thermometer.temperature
            )
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::providers::{OwningDeviceInfoProvider, BorrowingDeviceInfoProvider};
    use super::sockets::{SmartSocket, SmartThermometer};
    use crate::house::premises::{Room, SmartHouse};

    #[test]
    fn test_borrowing_provider() {
        let socket = SmartSocket { id: "3534".to_string() };
        let thermometer = SmartThermometer { id: "3".to_string(), temperature: 24.0, };
        let info_provider = BorrowingDeviceInfoProvider { socket: &socket, thermometer: &thermometer };

        let mut rooms = HashMap::new();
        rooms.insert("TestRoom".to_string(), Room::new(
            String::from("TestRoom"),
            vec![socket.id.clone()]
            )
        );

        let address = "Moscow, Arbatskaya, 10, 1".to_string();
        let house = SmartHouse::new(address, rooms);
        let report = house.create_report(&info_provider);

        assert_eq!(report, "Room: TestRoom\t -> socket id: 3534, thermometer id: 3, value: 24\n");
    }

    #[test]
    fn test_owning_provider() {
        let socket = SmartSocket { id: "3534".to_string() };
        let info_provider = OwningDeviceInfoProvider { socket: socket };

        let mut rooms = HashMap::new();

        let mut devices: Vec<String> = Vec::new();
        devices.push(info_provider.socket.id.clone());

        rooms.insert("TestRoom".to_string(), Room::new(
            String::from("TestRoom"),
            devices
            )
        );

        let address = "Moscow, Arbatskaya, 10, 1".to_string();
        let house = SmartHouse::new(address, rooms);
        let report = house.create_report(&info_provider);

        assert_eq!(report, "Room: TestRoom\t-> socket id: 3534")
    }
}
