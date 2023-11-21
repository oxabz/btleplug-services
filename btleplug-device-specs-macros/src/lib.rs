extern crate proc_macro;
use proc_macro::TokenStream;

use syn::parse_macro_input;
use quote::quote;

mod parse;
mod generation;

#[proc_macro]
/**

Defines a GATT service of a peripheral for easier interaction. 
# Example 

```rust, no_run 
define_service!{
    pub SensorService,
    "8D53DC1D-1DB7-4CD3-868B-8A527460AA50";
    [RWwN] "77073BB0-9E25-481C-B519-DEF98B669B74" => commands : Vec<u8>,
}
```
The macro first take the service name and it's visibility then the service uuid.
Then the characteristics are listed in the following manners :
    [access] <uuid> => <characteristic> : <type>
- access : can be [R]ead, [W]rite, [N]otify(not implemented yet), [w]rite_wo_resp
- characteristic : will determine the name of the function used to work with the characteristic
- type : is the type of the characteristic. It has to implement Into<Vec<u8>> and From<Vec<u8>>. If the types are not uniform across different access use Vec<u8>
 */
pub fn define_service(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as parse::DefineServiceInput);

    let service_vis = input.service_vis;
    let service_name = input.service_ident;
    let uuid = input.service_uuid.value();

    /* Generates the uuid constants for the characteristics of the service */
    let uuid_consts = input.characteristics.iter().map(generation::characteristic_2_uuid_const);

    /* Generate the different access functions for each characteristic */
    let implementation = input.characteristics.iter().map(|c|{
        let mut w_fn = None;
        let mut wo_fn = None;
        let mut r_fn = None;

        if c.accesses.to_string().contains('w'){
            wo_fn = Some(generation::characteristic_2_write_function(c, false));
        }

        if c.accesses.to_string().contains('W'){
            w_fn = Some(generation::characteristic_2_write_function(c, true));
        }
        
        if c.accesses.to_string().contains('R'){
            r_fn = Some(generation::characteristic_2_read_function(c));
        }

        quote!(
            #w_fn
            #wo_fn
            #r_fn
        )
    });

    let tokens = quote! {
        #service_vis struct #service_name<'p, P: btleplug::api::Peripheral>{
            inner: &'p P,
        }

        impl<'p, P: btleplug::api::Peripheral> #service_name<'p, P> {
            pub const UUID: uuid::Uuid = uuid::uuid!(#uuid);

            #(#uuid_consts)*

            #(#implementation)*

            pub fn new(peripheral: &'p P) -> Self{
                Self{
                    inner: peripheral
                }
            }
        }
    };

    tokens.into()
}