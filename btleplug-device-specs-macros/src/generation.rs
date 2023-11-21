use proc_macro2::TokenStream;
use quote::quote;
use crate::parse::CharacteristicLine;

pub fn uuid_const_name(characteristic_line: &CharacteristicLine) -> syn::Ident{
    let mut const_name = characteristic_line.characteristic.to_string().to_uppercase();
    const_name.extend("_UUID".chars());
    let const_name = syn::Ident::new(&const_name, characteristic_line.characteristic.span());
    const_name
}

pub fn characteristic_2_uuid_const(characteristic_line: &CharacteristicLine) -> TokenStream{
    let const_name = uuid_const_name(characteristic_line);
    let uuid = characteristic_line.uuid.clone();

    quote!(
        pub const #const_name: uuid::Uuid = uuid::uuid!(#uuid);
    )
}

pub fn characteristic_2_write_function(characteristic_line: &CharacteristicLine, with_response: bool) -> TokenStream{
    let const_name = uuid_const_name(characteristic_line);

    let write_type = if with_response {
        quote!(btleplug::api::WriteType::WithResponse)
    } else {
        quote!(btleplug::api::WriteType::WithoutResponse)
    };

    let mut function_name = String::from("write_");
    function_name.extend(characteristic_line.characteristic.to_string().chars());

    if !with_response {
        function_name.extend("_wo_resp".chars())
    }
    let function_name = syn::Ident::new(&function_name, characteristic_line.characteristic.span());

    let value_type = characteristic_line.typ.clone();

    quote!(
        pub async fn #function_name(&self, value: #value_type) -> btleplug::Result<()>{
            let value: Vec<u8> = value.into();

            let characteristics = self.inner.characteristics();
            let characteristic = characteristics.iter().find(|c|c.uuid == Self::#const_name && c.service_uuid == Self::UUID);
            let Some(characteristic) = characteristic else {
                return Err(btleplug::Error::UnexpectedCharacteristic);
            };

            self.inner.write(characteristic, &value[..], #write_type).await
        }
    )
}

pub fn characteristic_2_read_function(characteristic_line: &CharacteristicLine) -> TokenStream{
    let const_name = uuid_const_name(characteristic_line);

    let mut function_name = String::from("read_");
    function_name.extend(characteristic_line.characteristic.to_string().chars());
    let function_name = syn::Ident::new(&function_name, characteristic_line.characteristic.span());

    let value_type = characteristic_line.typ.clone();

    quote!(
        pub async fn #function_name(&self) -> btleplug::Result<#value_type>{
            let characteristics = self.inner.characteristics();
            let characteristic = characteristics.iter().find(|c|c.uuid == Self::#const_name && c.service_uuid == Self::UUID);
            let Some(characteristic) = characteristic else {
                return Err(btleplug::Error::UnexpectedCharacteristic);
            };

            let res = self.inner.read(characteristic).await?;

            Ok(res.into())
        }
    )
}