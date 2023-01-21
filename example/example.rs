use craft::devices;
use craft::house;
use craft::macros;

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        id: "1".to_string(),
    };
    let socket2 = SmartSocket {
        id: "2".to_string(),
    };
    let thermometer = SmartThermometer {
        id: "3".to_string(),
        temperature: 24.0,
    };

    // Инициализация дома
    let house = SmartHouse::new(
        "Moscow, Arbatskaya, 10, 1".to_string(),
        dict!(
            "Bedroom".to_string() => Room::new(
                String::from("Bedroom"),
                vec![socket1.id.clone()]
            ),

            "Kitchen".to_string() => Room::new(
                "Kitchen".to_string(),
                vec![
                    socket2.id.clone(),
                    thermometer.id.clone()
                ]
            )
        ),
    );

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermometer: &thermometer,
    };
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}