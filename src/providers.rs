pub mod providers {
    use crate::sockets::sockets::{SmartSocket, SmartThermometer};
    use crate::sockets::sockets::{SocketTrait, ThermDeviceTrait};

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
            format!("Room: {}\t-> socket id: {}", room, self.socket.get_id())
        }
    }

    impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
        fn status(&self, room: &str, _device: &str) -> String {
            format!(
                "Room: {}\t -> socket id: {}, thermometer id: {}, value: {}\n",
                room,
                self.socket.get_id(),
                self.thermometer.get_id(),
                self.thermometer.get_temperature()
            )
        }
    }
}
