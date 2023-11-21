use std::ops::Deref;

use crate::define_service;

pub struct DeviceName(String);

impl From<Vec<u8>> for DeviceName{
    fn from(value: Vec<u8>) -> Self {
        Self(String::from_utf8(value).unwrap_or("Invalid Device Name".to_string()))
    }
}

impl Deref for DeviceName{
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

define_service! {
    pub GenericAccessService,
    "00001800-0000-1000-8000-00805f9b34fb";
    [R] "00002A00-0000-1000-8000-00805f9b34fb" => device_name: DeviceName,
    [R] "00002A01-0000-1000-8000-00805f9b34fb" => appearance: Vec<u8>,
    [R] "00002A04-0000-1000-8000-00805f9b34fb" => preferred_connection_parameters: Vec<u8>,
}