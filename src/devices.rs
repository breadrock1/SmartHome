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