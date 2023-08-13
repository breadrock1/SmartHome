pub mod smarthouse {
    use crate::errors::{DeviceError, DeviceResult};
    use crate::errors::{RoomError, RoomResult};
    use crate::providers::info_providers::DeviceInfoProvider;
    use crate::sockets::smart_sockets::SocketType;
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Room {
        name: String,
        devices: HashMap<String, SocketType>,
    }

    impl Room {
        pub fn new(name: &str) -> Self {
            Room {
                name: name.to_string(),
                devices: HashMap::new(),
            }
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_devices(&self) -> Vec<&SocketType> {
            self.devices.values().collect()
        }

        pub fn get_device(&self, dev_name: &str) -> Option<&SocketType> {
            self.devices.get(dev_name)
        }

        pub fn get_device_mut(&mut self, dev_name: &str) -> Option<&mut SocketType> {
            self.devices.get_mut(dev_name)
        }

        pub fn add_device(&mut self, device: SocketType) -> RoomResult {
            match self.devices.entry(device.name()) {
                Entry::Occupied(_) => Err(RoomError::AlreadyExists),
                Entry::Vacant(entry) => {
                    let _ = entry.insert(device);
                    Ok(())
                }
            }
        }

        pub fn add_devices(&mut self, devices: Vec<SocketType>) -> RoomResult {
            devices
                .into_iter()
                .for_each(|dev| self.add_device(dev).unwrap());

            Ok(())
        }

        pub fn del_device(&mut self, device: &str) -> DeviceResult {
            match self.devices.remove(device) {
                None => Err(DeviceError::DeleteError),
                Some(_) => Ok(()),
            }
        }
    }

    #[derive(Default)]
    pub struct SmartHouse {
        address: String,
        rooms: HashMap<String, Room>,
    }

    impl SmartHouse {
        pub fn new(address: &str) -> Self {
            SmartHouse {
                address: address.to_string(),
                rooms: HashMap::new(),
            }
        }

        pub fn get_address(&self) -> &str {
            self.address.as_str()
        }

        pub fn get_rooms(&self) -> Vec<&Room> {
            self.rooms.values().collect()
        }

        pub fn get_room(&self, room_name: &str) -> Option<&Room> {
            self.rooms.get(room_name)
        }

        pub fn get_room_mut(&mut self, room_name: &str) -> Option<&mut Room> {
            self.rooms.get_mut(room_name)
        }

        pub fn add_room(&mut self, room: Room) -> RoomResult {
            match self.rooms.entry(room.name.to_string()) {
                Entry::Occupied(_) => Err(RoomError::AlreadyExists),
                Entry::Vacant(entry) => {
                    let _ = entry.insert(room);
                    Ok(())
                }
            }
        }

        pub fn del_room(&mut self, room_name: &str) -> RoomResult {
            match self.rooms.remove(room_name) {
                None => Err(RoomError::DeleteError),
                Some(_) => Ok(()),
            }
        }

        pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
            self.get_rooms()
                .into_iter()
                .map(|room| match self.device_status(room, provider) {
                    Some(info) => info,
                    None => format!("Failed get status for {}", room.name),
                })
                .collect::<Vec<String>>()
                .join("\n")
        }

        fn device_status(&self, room: &Room, provider: &dyn DeviceInfoProvider) -> Option<String> {
            let room_info = format!("Room: {} -> ", room.name);

            let report = room
                .devices
                .values()
                .map(|dev| self.gen_info(dev, provider))
                .reduce(|first, second| first + &second)
                .unwrap();

            Some(room_info + &report)
        }

        fn gen_info(&self, soc_t: &SocketType, prov: &dyn DeviceInfoProvider) -> String {
            match soc_t {
                SocketType::Simple(d) => prov.status(d),
                SocketType::Thermometer(d) => prov.status(d),
            }
        }
    }
}
