pub mod sockets {
    pub enum SocketType {
        Stupid(SmartSocket),
        Thermometer(SmartThermometer)
    }

    pub struct SmartSocket {
        pub id: String
    }

    pub struct SmartThermometer {
        pub id: String,
        pub temperature: f32,
    }

    pub trait DeviceTrait {
        fn new(id: String) -> Self;
        fn get_id(&self) -> &String;
        fn set_id(&mut self, id: String);
    }

    trait ThermDeviceTrait {
        fn get_temperature(&self) -> f32;
        fn set_temperature(&mut self, value: f32);
    }

    impl DeviceTrait for SmartSocket {
        fn new(id: String) -> Self {
            SmartSocket { id }
        }

        fn get_id(&self) -> &String {
            &self.id
        }

        fn set_id(&mut self, id: String) {
            self.id = id
        }
    }

    impl DeviceTrait for SmartThermometer {
        fn new(id: String) -> Self {
            SmartThermometer {
                id,
                temperature: 0 as f32
            }
        }

        fn get_id(&self) -> &String {
            &self.id
        }

        fn set_id(&mut self, id: String) {
            self.id = id
        }
    }

    impl ThermDeviceTrait for SmartThermometer {
        fn get_temperature(&self) -> f32 {
            self.temperature
        }

        fn set_temperature(&mut self, value: f32) {
            self.temperature = value
        }
    }
}