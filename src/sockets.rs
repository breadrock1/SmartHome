pub mod smart_sockets {
    #[derive(Clone)]
    pub enum SocketType {
        Simple(SmartSocket),
        Thermometer(SmartThermometer),
    }

    impl SocketType {
        pub fn name(&self) -> String {
            let result = match self {
                SocketType::Simple(d) => &d.id,
                SocketType::Thermometer(d) => &d.id,
            };

            result.to_string()
        }
    }

    pub trait SocketTrait {
        fn new(id: String) -> Self
            where
                Self: Sized;
        fn get_id(&self) -> &String;
        fn set_id(&mut self, id: String);
        fn get_info(&self) -> String;

        fn get_type(&self) -> String;
    }

    #[derive(Default, Clone)]
    pub struct SmartSocket {
        id: String,
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

        fn get_type(&self) -> String {
            String::from("Socket")
        }
    }

    impl From<SmartSocket> for SocketType {
        fn from(value: SmartSocket) -> Self {
            SocketType::Simple(value)
        }
    }

    #[derive(Default, Clone)]
    pub struct SmartThermometer {
        id: String,
        temperature: f32,
    }

    impl SocketTrait for SmartThermometer {
        fn new(id: String) -> Self {
            SmartThermometer {
                id,
                temperature: 0f32,
            }
        }

        fn get_id(&self) -> &String {
            &self.id
        }

        fn set_id(&mut self, id: String) {
            self.id = id
        }

        fn get_info(&self) -> String {
            format!(
                "thermometer id: {}, value: {}; ",
                &self.id, &self.temperature
            )
        }

        fn get_type(&self) -> String {
            String::from("Thermometer")
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

    impl From<SmartThermometer> for SocketType {
        fn from(value: SmartThermometer) -> Self {
            SocketType::Thermometer(value)
        }
    }
}
