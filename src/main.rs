// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

use std::collections::HashMap;
use std::ops::Add;

#[macro_export]
macro_rules! map {
    ( $( $key:expr => $val:expr ), * ) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key, $val); )*
            map
        }
    };
}

// ========================================================================

struct Thermometer {
    identifier: u32,
    is_enabled: bool,
    consumption: u16,
    description: String,
    temperature: i8
}

impl Thermometer {
    fn new(id: u32, consumption: u16, description: String) -> Self {
        Self {
            identifier: id,
            is_enabled: false,
            consumption,
            description,
            temperature: 0
        }
    }

    pub fn current_temperature(&self) -> i8 {
        self.temperature
    }
}

pub struct Socket {
    identifier: u32,
    is_enabled: bool,
    consumption: u16,
    description: String
}

impl Socket {
    fn new(id: u32, consumption: u16, description: String) -> Self {
        Self {
            identifier: id,
            is_enabled: false,
            consumption,
            description
        }
    }
}

// ========================================================================

trait Device {
    fn get_id(&self) -> u32;

    fn turn_on(&mut self);

    fn turn_off(&mut self);
}

impl Device for Thermometer {
    fn get_id(&self) -> u32 {
        self.identifier
    }

    fn turn_on(&mut self) {
        self.is_enabled = true;
    }

    fn turn_off(&mut self) {
        self.is_enabled = false;
    }
}

impl Device for Socket {
    fn get_id(&self) -> u32 {
        self.identifier
    }

    fn turn_on(&mut self) {
        self.is_enabled = true;
    }

    fn turn_off(&mut self) {
        self.is_enabled = false;
    }
}

// ========================================================================

struct Room {
    name: String,
    devices: HashMap<String, dyn Device>
}

trait HouseRoom{
    fn new(name: &'static String, devices: &'static HashMap<String, dyn Device>) -> Self;
}

impl HouseRoom for Room {
    fn new(name: String, devices: HashMap<String, dyn Device>) -> Self {
        Room(name, devices)
    }
}

// ========================================================================

struct SmartHouse {
    address: String,
    rooms: HashMap<String, Room>
}

impl SmartHouse {
    fn new(address: String, rooms: HashMap<String, Room>) -> Self {
        SmartHouse(address, rooms)
    }

    fn devices(&self, room: &str) -> &HashMap<String, dyn Device> {
        &self.rooms
            .get(room)
            .expect("Unknown room name!")
            .devices
    }


    fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
        self.rooms
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
// ========================================================================

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: Socket,
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a Socket,
    thermometer: &'b Thermometer,
}

// ========================================================================

trait DeviceInfoProvider {
    fn device_status(&self, room: &str, device: &dyn Device) -> String;
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_status(&self, room: &str, device: &dyn Device) -> String {
        let id = device.get_id().to_string();
        format!("There is owning device {id} placed into {room} room")
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider {
    fn device_status(&self, room: &str, device: &dyn Device) -> String {
        let id = device.get_id().to_string();
        format!("There is borrowing device {id} placed into room {room}")
    }
}

// ========================================================================

fn main() {

    let mut house = SmartHouse::new(
        &String::from("Moscow, Arbatskay 10"),
        &map!(

            "kitchen".to_string() => Room::new(
                &"kitchen".to_string(),
                map!(
                    "" => Device::new("".to_string())
                )
            ),

            "bedroom".to_string() => Room::new(
                &"bedroom".to_string(),
                map!(
                    "" => Device::new("".to_string())
                )
            ),

            "bathroom".to_string() => Room::new(
                &"bathroom".to_string(),
                map!(
                    "" => Device::new("".to_string())
                )
            )
        )
    );

    println!();

    // // Инициализация устройств
    // let socket1 = SmartSocket { 0: Device { id: "".to_string() } };
    // let socket2 = SmartSocket { 0: Device { id: "".to_string() } };
    // let thermo = SmartThermometer { 0: Device { id: "".to_string() } };
    //
    //
    // // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    // let info_provider_1 = OwningDeviceInfoProvider {
    //     socket: socket1,
    // };
    // // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    // let report1 = house.create_report(/* &info_provider_1 */);
    //
    // // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    // let info_provider_2 = BorrowingDeviceInfoProvider {
    //     socket: &socket2,
    //     thermometer: &thermo,
    // };
    // // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    // let report2 = house.create_report(/* &info_provider_2 */);
    //
    // // Выводим отчёты на экран:
    // println!("Report #1: {report1}");
    // println!("Report #2: {report2}");
}
