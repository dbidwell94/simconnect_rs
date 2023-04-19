use darling::{FromField, FromVariant, ToTokens};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident};

#[derive(FromField)]
#[darling(attributes(datum))]
struct Opts {
    sim_var: syn::Path,
    sim_unit: Option<syn::Path>,
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
        let unit = Opts::from_field(field)
            .expect("All fields in a SimConnect struct need to contain a #[datum(..)] attribute")
            .sim_unit;

        let to_return;
        if let None = unit {
            to_return = quote! {
                None
            }
        } else {
            let unit = unit.unwrap();
            to_return = quote! {
                Some(Box::new(#unit))
            }
        }
        to_return
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
                            sim_unit: #sim_unit,
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
        if !var.fields.is_empty() {
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
        if !field.fields.is_empty() {
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

#[proc_macro_derive(FromStr)]
pub fn enum_from_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let to_return = quote! {
        impl std::str::FromStr for #ident {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let self_iter = Self::iter_enum();
                let lower_s = s.to_lowercase();

                for item in self_iter {
                    let lower_item = item.to_string().to_lowercase();
                    if lower_s == lower_item {
                        return Ok(item);
                    }
                }

                Err(anyhow::anyhow!("Unable to serialize {s} to SystemEvent"))
            }
        }
    };

    to_return.into()
}

#[proc_macro_derive(SimConnectToStruct)]
pub fn c_data_to_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct { fields, .. }) => fields,
        _ => panic!("Expected a struct with named fields"),
    };
    let field_ident = fields
        .clone()
        .into_iter()
        .map(|f| f.ident.expect("Expected named fields"));

    let mut previous_field_pointer: Option<Ident> = None;
    let mut converted_field_idents: Vec<Ident> = Vec::new();
    let borrowed_vec = &mut converted_field_idents;

    let base_pointer_name = Ident::new(&format!("pointer"), Span::call_site());
    let cloned_base_pointer_name = base_pointer_name.clone();
    let total_fields = &fields.len();
    let mut current_iter = 0usize;

    let converter: Vec<proc_macro2::TokenStream> = fields
        .into_iter()
        .map(move |f| {
            current_iter += 1;
            let previous_field_name = previous_field_pointer
                .as_ref()
                .unwrap_or(&cloned_base_pointer_name);

            let at_last_field = total_fields + 1 == current_iter;


            let field_ident = f.ident.expect("Expected named fields");
            let field_type = f.ty;
            let field_size_ident = Ident::new(&format!("{field_ident}_size"), Span::call_site());

            let string_requested = field_type.clone().to_token_stream().to_string() == "String";

            let field_ident_pointer =
                Ident::new(&format!("{field_ident}_pointer"), Span::call_site());
            let field_ident_pointer_type = if string_requested {
                quote! {*const i8}
            } else {
                quote! {*mut #field_type}
            };
            let temporary_c_string = string_requested.then(|| {
                let temp_ident = Ident::new(&format!("{field_ident}_cstr"), Span::call_site());
                quote! {
                    let #temp_ident = std::ffi::CStr::from_ptr(#field_ident_pointer);
                }
            });
            let field_len = (!at_last_field).then(|| {if string_requested {
                let c_str_ident = Ident::new(&format!("{field_ident}_cstr"), Span::call_site());
                quote! {
                    let #field_size_ident: usize = #c_str_ident.to_bytes_with_nul().len() + 1;
                }
            } else {
                quote! {
                    let #field_size_ident: usize = 1;
                }
            }});

            let transmute = if previous_field_pointer.is_none() {
                quote! {std::mem::transmute(#previous_field_name)}
            } else {
                let temp_prev_field_size = previous_field_name.to_token_stream().to_string().replace("_pointer", "_size");
                let prev_field_size_ident = Ident::new(&format!("{temp_prev_field_size}"), Span::call_site());
                quote! {
                    std::mem::transmute(#previous_field_name.add(#prev_field_size_ident))
                }
            };

            let deref_field = if string_requested {
                let c_str_ident = Ident::new(&format!("{field_ident}_cstr"), Span::call_site());
                quote! {
                    let #field_ident = #c_str_ident.to_str().expect("Unable to convert CStr to str").to_string();
                }
            } else {
                quote! {
                    let #field_ident = *#field_ident_pointer;
                }
            };

            borrowed_vec.push(field_ident.clone());

            let to_return = quote! {
                let #field_ident_pointer: #field_ident_pointer_type = #transmute;
                #temporary_c_string
                #field_len
                #deref_field
            };
            previous_field_pointer = Some(field_ident_pointer.clone());

            to_return
        })
        .collect();

    let to_return = quote! {
        impl SimConnectToStruct for #ident {
            type Error = ();
            type ReturnType = #ident;

            unsafe fn parse_struct(pointer: std::ptr::NonNull<u32>) -> Result<Self::ReturnType, Self::Error> {
                let #base_pointer_name = pointer.as_ptr();
                # (
                    #converter
                )*
                Ok(Self {
                    #(
                        #field_ident: #converted_field_idents,
                    )*
                })
            }
        }
    };

    to_return.into()
}
