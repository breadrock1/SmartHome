pub mod info_providers {
    use crate::sockets::smart_sockets::SocketTrait;
    use crate::sockets::smart_sockets::{SmartSocket, SmartThermometer};

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
        fn status(&self, device: &dyn SocketTrait) -> String;
    }

    impl DeviceInfoProvider for OwningDeviceInfoProvider {
        fn status(&self, device: &dyn SocketTrait) -> String {
            format!("device - {}", &device.get_info())
        }
    }

    impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
        fn status(&self, device: &dyn SocketTrait) -> String {
            format!("device - {}", &device.get_info())
        }
    }
}
