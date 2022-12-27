// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;
use std::ops::Add;

#[macro_export]
macro_rules! dict {
    ( $( $key:expr => $val:expr ), * ) => {
        {
            let mut dictionary = HashMap::new();
            $( dictionary.insert($key, $val); )*
            dictionary
        }
    };
}


struct Device {
    id: String
}

impl Device {
    fn new(name: &String) -> Self {
        Device(name)
    }
}


struct Room {
    name: String,
    devices: HashMap<String, String>
}

impl Room {
    fn empty() -> Self {
        Room()
    }

    fn new(name: &String, devices: &HashMap<String, String>) -> Self {
        Room(name, devices)
    }
}


struct SmartHouse {
    address: String,
    rooms: HashMap<String, Room>
}

impl SmartHouse {
    fn empty() -> Self {
        SmartHouse()
    }

    fn new(address: &String, rooms: &HashMap<String, Room>) -> Self {
        SmartHouse(address, rooms)
    }

    fn get_rooms(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    fn devices(&self, room: &str) -> &HashMap<String, String> {
        &self.get_rooms()
            .get(room)
            .expect("Unknown room name!")
            .devices
    }


    fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
        self.get_rooms()
            .into_iter()
            .map(|tuple_| tuple_.1)
            .map(|room_| self.room_devices_status(&room_, provider))
            .collect();

        return String::from("");
    }

    fn room_devices_status(&self, room: &Room, provider: &dyn DeviceInfoProvider) -> String {
        room.devices
            .values()
            .map(|device_| provider.device_status(&room.name, device_))
            .reduce(|first, second| first.add(&second));

        return String::from("");
    }
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
struct SmartSocket(Device);
struct SmartThermometer(Device);

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

trait DeviceInfoProvider {
    fn device_status(&self, room: &str, device: &dyn Device) -> String;
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации
impl<T> DeviceInfoProvider for T {
    fn device_status(&self, room: &str, device: &str) -> String {
        format!("{} room: device {} -> enabled", room, device)
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider {
    fn device_status(&self, room: &str, device: &BorrowingDeviceInfoProvider) -> String {
        format!("{} room: device {} -> {}", room, device., device.thermo.0)
    }
}


fn main() {

    let mut house = SmartHouse::new(
        &String::from("Moscow, Arbatskay 10"),
        &dict!(

            "kitchen".to_string() => Room::new(
                &"kitchen".to_string(),
                dict!(
                    "" => Device::new("".to_string()),
                )
            ),

            "bedroom".to_string() => Room::new(
                &"bedroom".to_string(),
                dict!(
                    "" => Device::new("".to_string()),
                )
            ),

            "bathroom".to_string() => Room::new(
                &"bathroom".to_string(),
                dict!(
                    "" => Device::new("".to_string()),
                )
            )
        )
    );


    // Инициализация устройств
    let socket1 = SmartSocket { 0: Device { id: "".to_string() } };
    let socket2 = SmartSocket { 0: Device { id: "".to_string() } };
    let thermo = SmartThermometer { 0: Device { id: "".to_string() } };


    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider {
        socket: socket1,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(/* &info_provider_1 */);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(/* &info_provider_2 */);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
