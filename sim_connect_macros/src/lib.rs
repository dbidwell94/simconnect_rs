use darling::FromField;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[derive(FromField)]
#[darling(attributes(datum))]
struct Opts {
    sim_var: syn::Path,
    sim_unit: syn::Path,
    data_type: syn::Path,
}

#[proc_macro_derive(StructToSimConnect, attributes(datum))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let sim_var = fields.iter().map(|field| {
        Opts::from_field(field)
            .expect("All fields in a SimConnect struct need to contain a #[datum(..)] attribute")
            .sim_var
    });
    let sim_unit = fields.iter().map(|field| {
        Opts::from_field(field)
            .expect("All fields in a SimConnect struct need to contain a #[datum(..)] attribute")
            .sim_unit
    });
    let data_type = fields.iter().map(|field| {
        Opts::from_field(field)
            .expect("All fields in a SimConnect struct need to contain a #[datum(..)] attribute")
            .data_type
    });
    let id = (0..sim_var.len()).map(|id| id as u32);

    quote! {
        impl StructToSimConnect for #ident {
            fn get_fields() -> Vec<sim_connect_rs::SimConnectDatum> {
                vec![
                    #(
                        sim_connect_rs::SimConnectDatum {
                            id: #id,
                            sim_var: #sim_var,
                            sim_unit: Box::new(#sim_unit),
                            data_type: #data_type
                        },
                    )*
                ]
            }
        }
    }
    .into()
}