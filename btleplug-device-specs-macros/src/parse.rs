use proc_macro2::TokenStream;
use syn::{bracketed, Ident, LitStr, Token, Type, Visibility};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Bracket;


/**
Struct used to parse the following input :

```rust,ignore
define_service{
    pub SensorService,
    "8D53DC1D-1DB7-4CD3-868B-8A527460AA50";
    [wN] "77073BB0-9E25-481C-B519-DEF98B669B74" => commands : Vec<u8>,
}
*/
pub struct DefineServiceInput{
    pub service_vis: Visibility,
    pub service_ident: Ident,
    pub comma: Token![,],
    pub service_uuid: LitStr,
    pub semicolon: Token![;],
    pub characteristics: Punctuated<CharacteristicLine, Token![,]>
}

impl Parse for DefineServiceInput{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self{
            service_vis: input.parse()?,
            service_ident: input.parse()?,
            comma: input.parse()?,
            service_uuid: input.parse()?,
            semicolon: input.parse()?,
            characteristics: Punctuated::parse_terminated(input)?,
        })
    }
}


pub struct CharacteristicLine{
    pub accesses_bracket: Bracket,
    pub accesses: TokenStream,
    pub uuid: LitStr,
    pub arrow : (Token![=], Token![>]),
    pub characteristic: Ident,
    pub colon: Token![:],
    pub typ: Type
}

impl Parse for CharacteristicLine {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let accesses;
        Ok(
            Self {
                accesses_bracket: bracketed!(accesses in input),
                accesses: accesses.parse()?,
                uuid: input.parse()?,
                arrow: (input.parse()?, input.parse()?),
                characteristic: input.parse()?,
                colon: input.parse()?,
                typ: input.parse()?,
            }
        )
    }
}