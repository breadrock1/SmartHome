pub mod smarthouse {
    use crate::errors::CustomError;
    use crate::providers::info_providers::DeviceInfoProvider;
    use crate::sockets::smart_sockets::SocketTrait;
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

        pub fn add_device(&mut self, device: Box<dyn SocketTrait>) -> Result<(), CustomError> {
            let curr_dev_name = &device.get_id();
            let duplicates = &self
                .devices
                .iter()
                .map(|dev| dev.deref().get_id())
                .any(|dev| dev.eq(*curr_dev_name));

            match duplicates {
                true => Err(CustomError::DeviceExists),
                false => {
                    self.devices.push(device);
                    Ok(())
                }
            }
        }

        pub fn add_devices(
            &mut self,
            devices: Vec<Box<dyn SocketTrait>>,
        ) -> Result<(), CustomError> {
            for device in devices {
                let result = self.add_device(device);
                if result.is_err() {
                    return Err(CustomError::DeviceExists);
                }
            }
            Ok(())
        }
    }

    pub struct SmartHouse {
        address: String,
        rooms: HashMap<String, Box<Room>>,
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

        pub fn get_address(&self) -> &String {
            &self.address
        }

        pub fn get_rooms(&self) -> Vec<&Room> {
            self.rooms
                .values()
                .map(|room_| room_.deref())
                .collect::<Vec<&Room>>()
        }

        pub fn get_room_by_id(&self, id: String) -> Option<&Room> {
            let all_rooms = &self.get_rooms();
            let result = all_rooms.iter().find(|room| room.name.eq(&id)).unwrap();

            Some(result)
        }

        pub fn add_room(&mut self, room: Box<Room>) -> Result<(), CustomError> {
            let room_name = &room.name;
            let duplicates = &self.rooms.contains_key(room_name);
            match duplicates {
                true => Err(CustomError::RoomExists),
                false => {
                    self.rooms.insert(room_name.clone(), room);
                    Ok(())
                }
            }
        }

        pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
            self.get_rooms()
                .iter()
                .map(|room_| match self.device_status(room_, provider) {
                    Some(room_info) => room_info,
                    None => format!("Failed get status for {}", room_.name),
                })
                .collect::<Vec<String>>()
                .join("\n")
        }

        fn device_status(&self, room: &Room, provider: &dyn DeviceInfoProvider) -> Option<String> {
            let room_devices = &room.devices;
            let room_info = format!("Room: {} -> ", &room.name);
            let report = room_devices
                .iter()
                .map(|device| provider.status(device.deref()))
                .reduce(|first, second| first + &second)
                .unwrap();

            Some(room_info + &report)
        }
    }
}
