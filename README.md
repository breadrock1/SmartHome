# SmartHome
There is simple project based on Rust which created while studying Rust programming language on Otus platform.

### Target:

Develop a prototype of the "smart home" library.

Description/Step-by-step homework activity:

Library development at home in a room where problems with the device may occur.

 - The house has a name and contains several rooms.
 - The library allows you to request a list of rooms in the house.
 - The substitution has a unique name and contains the name of several devices.
 - The device has a unique value to place the name.
 - The library allows you to get a list of devices for children.
 - The library has a function that returns a textual report about the state of the house.
 - This function is accepted as a generic type argument, receiving text information about the state of the device, for the conclusion in the report. This information must belong for each device based on the location of the device in the house: room name and device name. If the device is not found in the source of information, then instead of the text about returning a message about.
 - Give an example of a type that produces text information about home usage for reporting purposes. Template for describing library entities: [is here](https://gist.github.com/76dff7177f19ff7e802b1e121865afe4)

### Criteria for evaluation:

"Accepted" if:
 - The cargo clippy utility does not issue warnings.
 - The cargo fmt --check command produces no warnings.
 - In the main function, the "Smart Home" and the source of information about devices are initialized.
 - The screen displays a list of rooms and devices in them.
 - A status report is displayed on the screen.

### Competencies:

Knowledge of the Rust language:
 - using Rust's basic syntactic elements;
 - using advanced Rust syntactic elements;
 - use of difficult-to-understand Rust syntactic elements.
