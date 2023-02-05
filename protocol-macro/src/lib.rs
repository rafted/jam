use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[proc_macro_derive(PacketDef)]
pub fn define_packet(input: TokenStream) -> TokenStream {
    // Parse the input token stream and extract the struct name and fields
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    (field_name.clone(), field_type.clone())
                })
                .collect::<Vec<(Ident, Type)>>(),
            _ => panic!("Unsupported field format for Packet derive macro"),
        },
        _ => panic!("Unsupported data format for Packet derive macro"),
    };

    let mut encode_expand = quote! {};
    let mut decode_expand = quote! {};

    let mut segments_combined = String::from("");

    for (field_name, field_type) in fields {
        if let Type::Path(path) = &field_type {
            let segments = path.path.segments.to_token_stream().to_string();
            segments_combined = segments_combined + &segments;

            // this is for types that contain a generic type, e.g. Option<T>. this will then
            // call the decode function through
            // Option::<T>::decode(R);
            if segments.contains("<") {
                let types = segments.split("<").collect::<Vec<&str>>();

                let enum_ty = types[0].parse::<proc_macro2::TokenStream>().unwrap();
                let data_ty = types[1]
                    .replace(">", "")
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap();

                decode_expand.extend(quote! {
                    #field_name: <#enum_ty::<#data_ty> as crate::encoding::Encodable>::decode(reader)?,
                });
            } else {
                decode_expand.extend(quote! {
                    #field_name: <#field_type as crate::encoding::Encodable>::decode(reader)?,
                });
            }
            encode_expand.extend(quote! {
                <#field_type as crate::encoding::Encodable>::encode(&self.#field_name, writer)?;
            });
        }
    }

    // Generate the implementation of the encode and decode methods
    let expanded = quote! {
        impl crate::packet::Packet for #name {
            fn decode<T: std::io::Read>(reader: &mut T) -> anyhow::Result<Self> {
                Ok(Self {
                    #decode_expand
                })
            }

            fn encode<T: std::io::Write>(&self, writer: &mut T) -> anyhow::Result<()> {
                #encode_expand
                Ok(())
            }
        }
    };

    expanded.into()
}
