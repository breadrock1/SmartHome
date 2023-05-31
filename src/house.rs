pub mod house {
    use crate::providers::providers::DeviceInfoProvider;
    use crate::sockets::sockets::SocketTrait;
    use std::collections::HashMap;
    use std::ops::Deref;

    pub struct Room {
        name: String,
        devices: Vec<Box<dyn SocketTrait>>,
    }

    impl Room {
        pub fn new(name: String) -> Self {
            Room {
                name,
                devices: Vec::new(),
            }
        }

        pub fn get_devices(&self) -> &Vec<Box<dyn SocketTrait>> {
            &self.devices
        }

        pub fn add_device(&mut self, device: Box<dyn SocketTrait>) {
            self.devices.push(device)
        }

        pub fn add_devices(&mut self, devices: Vec<Box<dyn SocketTrait>>) {
            for device in devices {
                self.add_device(device);
            }
        }
    }

    pub struct SmartHouse {
        address: String,
        rooms: HashMap<String, Box<Room>>,
    }

    impl SmartHouse {
        pub fn new(address: String) -> Self {
            SmartHouse { address, rooms: HashMap::new() }
        }

        pub fn new_with_rooms(address: String, rooms: HashMap<String, Box<Room>>) -> Self {
            SmartHouse { address, rooms }
        }

        pub fn get_address(&self) -> &String {
            &self.address
        }

        pub fn get_rooms(&self) -> Vec<&Room> {
            self.rooms.values()
                .map(|room_| room_.deref())
                .collect::<Vec<&Room>>()
        }

        pub fn add_room(&mut self, room: Box<Room>) {
            let room_name = &room.name;
            self.rooms.insert(room_name.clone(), room);
        }

        pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
            self.get_rooms().iter()
                .map(|room_| {
                    match self.device_status(room_, provider) {
                        Some(room_info) => room_info,
                        None => format!("Failed get status for {}", room_.name),
                    }
                })
                .collect::<Vec<String>>()
                .join("\n")
        }

        fn device_status(&self, room: &Room, provider: &dyn DeviceInfoProvider) -> Option<String> {
            let room_devices = &room.devices;
            let room_info = format!("Room: {} -> ", &room.name);
            let report = room_devices
                .iter()
                .map(|device| provider.status(device))
                .reduce(|first, second| first + &second)
                .unwrap();

            Some(room_info + &report)
        }
    }
}
