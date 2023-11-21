use crate::define_service;

define_service! {
    pub GenericAccessService,
    "00001800-0000-1000-8000-00805f9b34fb";
    [R] "00002A00-0000-1000-8000-00805f9b34fb" => device_name: String,
    [R] "00002A01-0000-1000-8000-00805f9b34fb" => appearance: Vec<u8>,
    [R] "00002A04-0000-1000-8000-00805f9b34fb" => preferred_connection_parameters: Vec<u8>,
}