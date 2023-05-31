pub mod house {
    use crate::providers::providers::DeviceInfoProvider;
    use crate::sockets::sockets::SocketTrait;
    use std::collections::HashMap;

    pub struct Room {
        name: String,
        devices: Vec<Box<dyn SocketTrait>>,
    }

    pub struct SmartHouse {
        address: String,
        rooms: HashMap<String, Box<Room>>
    }

    impl Room {
        pub fn new(name: String) -> Self {
            Room {
                name,
                devices: Vec::new()
            }
        }

        pub fn add_devices(&mut self, devices: Vec<Box<dyn SocketTrait>>) {
            let room_devices = &mut self.devices;
            for device in devices {
                room_devices.push(device);
            }
        }

        // pub fn add_device(&mut self, device: &SmartSocket);
        // pub fn find_device(device_name: &str) -> Option<SmartSocket>;
    }

    impl SmartHouse {
        pub fn new(address: String) -> Self {
            SmartHouse {
                address,
                rooms: HashMap::new(),
            }
        }

        pub fn new_with_rooms(address: String, rooms: HashMap<String, Box<Room>>) -> Self {
            SmartHouse { address, rooms }
        }

        pub fn add_room(&mut self, room: Box<Room>) {
            let room_name = &room.name;
            self.rooms.insert(room_name.to_owned(), room);
        }

        pub fn get_address(&self) -> &String {
            &self.address
        }

        pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
            let status_vec: Vec<String> = self
                .rooms
                .values()
                .map(|room_| {
                    let status = self.device_status(room_, provider);
                    match status {
                        Some(room_info) => room_info,
                        None => format!("Failed get status for {}", room_.name),
                    }
                })
                .collect();

            status_vec.join("\n")
        }

        fn device_status(&self, room: &Room, provider: &dyn DeviceInfoProvider) -> Option<String> {
            let room_devices = &room.devices;
            room_devices
                .iter()
                .map(|device| provider.status(&room.name, device.get_id()))
                .reduce(|first, second| first + &second)
        }
    }
}
