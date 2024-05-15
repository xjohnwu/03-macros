use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    // get the ident
    let ident = &input.ident;
    // get generics
    let generics = &input.generics;
    // get enum variants
    let variants = match &input.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };

    // for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("Should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(variant: #ty) -> Self {
                                #ident::#variant_ident(variant)
                            }
                      }
                    }
                }
            }
            _ => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
}
