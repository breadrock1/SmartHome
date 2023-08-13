pub mod errors;
pub mod house;
pub mod macros;
pub mod providers;
pub mod sockets;

pub use house::smarthouse::{Room, SmartHouse};
pub use providers::info_providers::BorrowingDeviceInfoProvider;
pub use providers::info_providers::DeviceInfoProvider;
pub use providers::info_providers::OwningDeviceInfoProvider;
pub use sockets::smart_sockets::SmartSocket;
pub use sockets::smart_sockets::SmartThermometer;
pub use sockets::smart_sockets::SocketTrait;
