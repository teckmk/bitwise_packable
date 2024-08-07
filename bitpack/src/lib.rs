extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Attribute, Meta, Expr, Token, LitStr};
use syn::parse::Parse;

#[proc_macro_derive(BitwisePackable, attributes(bitwise_packable))]
pub fn bitwise_packable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let attrs = get_packing_type(&input.attrs);
    let size = attrs.unwrap_or_else(|| "auto".to_string());

    let (pack_code, unpack_code) = match size.as_str() {
        "i8" => (quote! {
            impl #name {
                pub fn pack_booleans(&self) -> u8 {
                    let mut result = 0u8;
                    let mut bit_index = 0;
                    for (i, field) in self.fields.iter().enumerate() {
                        if bit_index >= 8 { break; }
                        result |= (field.get(i) as u8) << bit_index;
                        bit_index += 1;
                    }
                    result
                }

                pub fn unpack_booleans(&self, packed: u8) -> Vec<bool> {
                    let mut booleans = vec![false; self.fields.len()];
                    for bit_index in 0..8 {
                        if bit_index >= self.fields.len() { break; }
                        booleans[bit_index] = (packed & (1 << bit_index)) != 0;
                    }
                    booleans
                }
            }
        }, quote! {}), // Note: No unpacking code needed for i8

        "i16" => (quote! {
            impl #name {
                pub fn pack_booleans(&self) -> u16 {
                    let mut result = 0u16;
                    let mut bit_index = 0;
                    for (i, field) in self.fields.iter().enumerate() {
                        if bit_index >= 16 { break; }
                        result |= (field.get(i) as u16) << bit_index;
                        bit_index += 1;
                    }
                    result
                }

                pub fn unpack_booleans(&self, packed: u16) -> Vec<bool> {
                    let mut booleans = vec![false; self.fields.len()];
                    for bit_index in 0..16 {
                        if bit_index >= self.fields.len() { break; }
                        booleans[bit_index] = (packed & (1 << bit_index)) != 0;
                    }
                    booleans
                }
            }
        }, quote! {}),

        "i32" => (quote! {
            impl #name {
                pub fn pack_booleans(&self) -> u32 {
                    let mut result = 0u32;
                    let mut bit_index = 0;
                    for (i, field) in self.fields.iter().enumerate() {
                        if bit_index >= 32 { break; }
                        result |= (field.get(i) as u32) << bit_index;
                        bit_index += 1;
                    }
                    result
                }

                pub fn unpack_booleans(&self, packed: u32) -> Vec<bool> {
                    let mut booleans = vec![false; self.fields.len()];
                    for bit_index in 0..32 {
                        if bit_index >= self.fields.len() { break; }
                        booleans[bit_index] = (packed & (1 << bit_index)) != 0;
                    }
                    booleans
                }
            }
        }, quote! {}),

        "i64" => (quote! {
            impl #name {
                pub fn pack_booleans(&self) -> u64 {
                    let mut result = 0u64;
                    let mut bit_index = 0;
                    for (i, field) in self.fields.iter().enumerate() {
                        if bit_index >= 64 { break; }
                        result |= (field.get(i) as u64) << bit_index;
                        bit_index += 1;
                    }
                    result
                }

                pub fn unpack_booleans(&self, packed: u64) -> Vec<bool> {
                    let mut booleans = vec![false; self.fields.len()];
                    for bit_index in 0..64 {
                        if bit_index >= self.fields.len() { break; }
                        booleans[bit_index] = (packed & (1 << bit_index)) != 0;
                    }
                    booleans
                }
            }
        }, quote! {}),

        _ => (
            {
                quote! {
                    impl #name {
                        pub fn pack_booleans(&self) -> Bitfield {
                            let mut bitfield = Bitfield::new(self.num_bits());
                            let mut bit_index = 0;
                            for (i, field) in self.fields.iter().enumerate() {
                                if bit_index >= bitfield.parts.len() * 64 { break; }
                                bitfield.set(bit_index, field.get(i));
                                bit_index += 1;
                            }
                            bitfield
                        }

                        pub fn unpack_booleans(&self, bitfield: Bitfield) -> Vec<bool> {
                            let mut booleans = vec![false; self.num_bits()];
                            for bit_index in 0..self.num_bits() {
                                booleans[bit_index] = bitfield.get(bit_index);
                            }
                            booleans
                        }

                        fn num_bits(&self) -> usize {
                            self.fields.len()
                        }
                    }
                }
            },
            quote! {},
        ),
    };

    let expanded = quote! {
        #pack_code
        #unpack_code
    };

    TokenStream::from(expanded)
}

struct SizeAttr {
    size: LitStr,
}

impl Parse for SizeAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let size = input.parse()?;
        Ok(SizeAttr { size })
    }
}



fn get_packing_type(attrs: &[Attribute]) -> Option<String> {
    attrs.iter()
        .filter(|attr| attr.path().is_ident("bitwise_packable"))
        .find_map(|attr| {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                meta_list.parse_args_with(|input: syn::parse::ParseStream| {
                    while !input.is_empty() {
                        let ident: syn::Ident = input.parse()?;
                        if ident == "size" {
                            let _: Token![=] = input.parse()?;
                            let size: Expr = input.parse()?;
                            if let Expr::Lit(expr_lit) = size {
                                if let syn::Lit::Str(lit_str) = expr_lit.lit {
                                    return Ok(Some(lit_str.value()));
                                }
                            }
                        }
                        if !input.is_empty() {
                            let _: Token![,] = input.parse()?;
                        }
                    }
                    Ok(None)
                }).ok().flatten()
            } else {
                None
            }
        })
}