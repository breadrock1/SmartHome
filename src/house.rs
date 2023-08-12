pub mod smarthouse {
    use crate::errors::{DeviceError, DeviceResult};
    use crate::errors::{RoomError, RoomResult};
    use crate::providers::info_providers::DeviceInfoProvider;
    use crate::sockets::smart_sockets::{SocketType};
    use std::collections::HashMap;
    use std::ops::Deref;

    #[derive(Default)]
    pub struct Room {
        name: String,
        devices: HashMap<String, SocketType>,
    }

    impl Room {
        pub fn new(name: String) -> Option<Self> {
            let room = Room {
                name,
                devices: HashMap::new(),
            };

            Some(room)
        }

        pub fn get_name(&self) -> &str {
            self.name.deref()
        }

        pub fn get_devices(&self) -> &HashMap<String, SocketType> {
            &self.devices
        }

        pub fn get_device(&self, dev_name: &str) -> Option<&SocketType> {
            self.devices.get(dev_name)
        }

        pub fn add_device(&mut self, device: &SocketType) -> RoomResult {
            let device_id = device.name();
            let dev_str = device_id.deref();
            match self.devices.contains_key(dev_str) {
                true => Err(RoomError::AlreadyExists),
                false => {
                    let dev_clone = device.clone();
                    self.devices.insert(device_id.to_string(), dev_clone);
                    Ok(())
                }
            }
        }

        pub fn add_devices(&mut self, devices: Vec<SocketType>) -> DeviceResult {
            for device in devices {
                let result = self.add_device(&device);
                if result.is_err() {
                    return Err(DeviceError::AlreadyExists);
                }
            }
            Ok(())
        }

        pub fn del_device(&mut self, device: &str) -> DeviceResult {
            match &self.devices.remove(device) {
                None => Err(DeviceError::DeleteError),
                Some(_) => Ok(())
            }
        }
    }

    #[derive(Default)]
    pub struct SmartHouse {
        address: String,
        rooms: HashMap<String, Box<Room>>,
    }

    impl SmartHouse {
        pub fn new(address: String) -> Option<Self> {
            let sm = SmartHouse {
                address,
                rooms: HashMap::new(),
            };

            Some(sm)
        }

        pub fn new_with_rooms(address: String, rooms: HashMap<String, Box<Room>>) -> Option<Self> {
            let sm = SmartHouse { address, rooms };
            Some(sm)
        }

        pub fn get_address(&self) -> Option<String> {
            let curr_address = &self.address;
            Some(curr_address.into())
        }

        pub fn get_rooms(&self) -> Vec<&Room> {
            self.rooms
                .values()
                .map(|room_| room_.deref())
                .collect::<Vec<&Room>>()
        }

        pub fn get_room_by_id(&self, id: String) -> Option<&Room> {
            let all_rooms = self.get_rooms();
            let result = all_rooms.iter().find(|room| room.name.eq(&id)).unwrap();

            Some(result)
        }

        pub fn add_room(&mut self, room: Box<Room>) -> RoomResult {
            let room_name = &room.name;
            let duplicates = self.rooms.contains_key(room_name);
            match duplicates {
                true => Err(RoomError::AlreadyExists),
                false => {
                    self.rooms.insert(room_name.clone(), room);
                    Ok(())
                }
            }
        }

        pub fn del_room(&mut self, room_id: &str) -> RoomResult {
            match &self.rooms.remove(room_id) {
                None => Err(RoomError::DeleteError),
                Some(_) => Ok(()),
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
                .values()
                .map(|device| self.prov(device, provider))
                .reduce(|first, second| first + &second)
                .unwrap();

            Some(room_info + &report)
        }

        fn prov(&self, soc_t: &SocketType, prov: &dyn DeviceInfoProvider) -> String {
            match soc_t {
                SocketType::Simple(d) => prov.status(d),
                SocketType::Thermometer(d) => prov.status(d),
            }
        }
    }
}
