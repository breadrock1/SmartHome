pub mod sockets {
    pub enum SocketType {
        Simple,
        Thermometer,
    }

    #[derive(Clone)]
    pub struct SmartSocket {
        id: String,
    }

    #[derive(Clone)]
    pub struct SmartThermometer {
        id: String,
        temperature: f32,
    }

    pub trait SocketTrait {
        fn new(id: String) -> Self
        where
            Self: Sized;
        fn get_id(&self) -> &String;
        fn set_id(&mut self, id: String);
        fn get_info(&self) -> String;
    }

    impl SocketTrait for SmartSocket {
        fn new(id: String) -> Self {
            SmartSocket { id }
        }

        fn get_id(&self) -> &String {
            &self.id
        }

        fn set_id(&mut self, id: String) {
            self.id = id
        }

        fn get_info(&self) -> String {
            format!("socket id: {}; ", &self.id)
        }
    }

    impl SocketTrait for SmartThermometer {
        fn new(id: String) -> Self {
            SmartThermometer {
                id,
                temperature: 0 as f32,
            }
        }

        fn get_id(&self) -> &String {
            &self.id
        }

        fn set_id(&mut self, id: String) {
            self.id = id
        }

        fn get_info(&self) -> String {
            format!("thermometer id: {}, value: {}; ", &self.id, &self.temperature)
        }
    }

    pub trait ThermDeviceTrait {
        fn get_temperature(&self) -> f32;
        fn set_temperature(&mut self, value: f32);
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
