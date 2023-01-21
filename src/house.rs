pub mod premises {
    use std::collections::HashMap;
    use crate::devices::providers::DeviceInfoProvider;

    pub struct Room {
        pub name: String,
        pub devices: Vec<String>,
    }

    impl Room {
        pub fn new(name: String, devices: Vec<String>) -> Self {
            Room { name, devices }
        }
    }

    pub struct SmartHouse {
        #[allow(unused)]
        pub address: String,
        pub rooms: HashMap<String, &'static Room>,
    }

    impl SmartHouse {
        pub fn new(address: String, rooms: HashMap<String, &'static Room>) -> Self {
            SmartHouse { address, rooms }
        }

        // pub fn add_room(&mut self, name: String, room: Room) {
        //     self.rooms.insert(name, room);
        // }
        //
        // pub fn get_rooms(&self) -> Vec<&String> {
        //     // Размер возвращаемого массива можно выбрать самостоятельно
        //     self.rooms.keys().collect()
        // }
        //
        // pub fn get_devices(&self, room: String) -> Vec<String> {
        //     // Размер возвращаемого массива можно выбрать самостоятельно
        //     let devices_opt = self.rooms.get(&room)
        //         .map(|room| room.devices.clone());
        //
        //     match devices_opt {
        //         Some(devices) => devices,
        //         None => Vec::new()
        //     }
        // }

        pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
            let status_vec: Vec<String> = self
                .rooms
                .values()
                .into_iter()
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
                .map(|device| provider.status(room.name.as_str(), device.as_str()))
                .reduce(|first, second| first + &second)
        }
    }
}
