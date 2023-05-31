pub mod house {
    use crate::providers::providers::DeviceInfoProvider;
    use crate::sockets::sockets::{SmartSocket, SocketTrait};
    use std::collections::HashMap;

    pub struct Room {
        name: String,
        devices: Vec<dyn SocketTrait>,
    }

    pub struct SmartHouse {
        address: String,
        rooms: HashMap<String, dyn SocketTrait>,
    }

    impl Room {
        pub fn new(name: String) -> Self {
            Room {
                name,
                devices: Vec::<dyn SocketTrait>::new(),
            }
        }

        pub fn add_devices(&mut self, devices: Vec<dyn SocketTrait>) {
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
                rooms: HashMap::<String, Room>::new(),
            }
        }

        pub fn new_with_rooms(address: String, rooms: HashMap<String, Room>) -> Self {
            SmartHouse { address, rooms }
        }

        pub fn add_room(&mut self, room: Room) {
            let room_name = &room.name;
            self.rooms.insert(room_name.to_owned(), room);
        }

        pub fn get_address(&self) -> &String {
            &self.address
        }

        pub fn get_rooms(&self) -> Vec<String> {
            todo!()
        }

        pub fn devices(&self, room: String) -> Vec<&String> {
            self.rooms
                .get(&room)
                .map(|room| &room.devices)
                .unwrap()
                .iter()
                .map(|device| device.get_id())
                .collect()
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
