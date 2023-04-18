use darling::{FromField, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields};

#[derive(FromField)]
#[darling(attributes(datum))]
struct Opts {
    sim_var: syn::Path,
    sim_unit: syn::Path,
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
    let data_type = fields.iter().map(|field| field.ty.clone());

    let id = (0..sim_var.len()).map(|id| id as u32);

    quote! {
        impl StructToSimConnect for #ident {
            fn get_fields() -> Vec<sim_connect_rs::SimConnectDatum> {
                use sim_connect_rs::sim_var_types::IntoSimVarType;
                vec![
                    #(
                        sim_connect_rs::SimConnectDatum {
                            id: #id,
                            sim_var: #sim_var,
                            sim_unit: Box::new(#sim_unit),
                            data_type: #data_type::into_sim_var()
                        },
                    )*
                ]
            }
        }
    }
    .into()
}

#[derive(FromVariant)]
#[darling(attributes(string))]
struct StringOpts {
    name: String,
}

#[proc_macro_derive(ToSimConnect, attributes(string))]
pub fn enum_to_sim_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("expected an enum"),
    };

    let variant_ident = variants.iter().map(|var| {
        if var.fields.len() > 0 {
            panic!("Enum should not have any fields");
        }
        var.ident.to_owned()
    });

    let variant_string = variants.iter().map(|var| {
        let default = var.ident.to_string();
        let opts = StringOpts::from_variant(var).unwrap_or(StringOpts { name: default });
        opts.name
    });

    let to_return = quote! {
        impl std::string::ToString for #ident {
            fn to_string(&self) -> String {
                match self {
                    # (
                        Self::#variant_ident => #variant_string.to_owned(),
                    )*
                }
            }
        }

        impl ToSimConnect for #ident {
            fn sc_string(&self) -> std::ffi::CString {
                CString::new(self.to_string()).unwrap()
            }
        }
    };

    to_return.into()
}

#[proc_macro_derive(SimUnit)]
pub fn to_sim_unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let to_return = quote! {
        impl SimUnit for #ident {}
    };

    to_return.into()
}

#[proc_macro_derive(InputEvent)]
pub fn to_input_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let to_return = quote! {
        impl InputEvent for #ident {}
    };

    to_return.into()
}

#[proc_macro_derive(IterEnum)]
pub fn plain_enum_iter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let field_name = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("Expected an enum"),
    }
    .into_iter()
    .map(|field| {
        if field.fields.len() > 0 {
            panic!("Expected an enum with no fields");
        }
        field.ident
    });

    let to_return = quote! {
        impl IterEnum for #ident {
            type Item = #ident;

            fn iter_enum() -> std::vec::IntoIter<#ident> {
                vec![
                    #(
                        Self::#field_name,
                    )*
                ].into_iter()
            }
        }
    };

    to_return.into()
}
