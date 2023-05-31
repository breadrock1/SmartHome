pub mod providers {
    use crate::sockets::sockets::{SmartSocket, SmartThermometer};
    use crate::sockets::sockets::{SocketTrait};

    pub enum ProviderType {
        Owning,
        Borrowing,
    }

    pub struct OwningDeviceInfoProvider {
        pub socket: SmartSocket,
    }

    pub struct BorrowingDeviceInfoProvider<'a, 'b> {
        pub socket: &'a SmartSocket,
        pub thermometer: &'b SmartThermometer,
    }

    pub trait DeviceInfoProvider {
        fn status(&self, device: &Box<dyn SocketTrait>) -> String;
    }

    impl DeviceInfoProvider for OwningDeviceInfoProvider {
        fn status(&self, device: &Box<dyn SocketTrait>) -> String {
            format!("{}", &device.get_info())
        }
    }

    impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
        fn status(&self, device: &Box<dyn SocketTrait>) -> String {
            format!("{}", &device.get_info())
        }
    }
}
