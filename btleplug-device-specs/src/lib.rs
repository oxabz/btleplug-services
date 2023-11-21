pub mod services;

use btleplug::platform::Peripheral;

use btleplug_device_specs_macros::define_service;

pub trait SpecPeripheral<P: btleplug::api::Peripheral>: Sized{
    fn from_peripheral(peripheral: P) -> btleplug::Result<Self>;

    fn matches(peripheral: &Peripheral) -> bool;
    fn matches_exactly(peripheral: &Peripheral) -> bool;

    fn as_peripheral(&self) -> &P;
}

#[cfg(test)]
mod test{
    #[test]
    fn test(){
        

    }
}