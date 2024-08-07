extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Lit, Meta, NestedMeta, Type};

fn get_packing_type(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        // Parse the attribute to Meta
        let meta = attr.parse_meta().ok()?;

        // Check if the attribute is a list
        let Meta::List(meta_list) = meta else {
            continue;
        };

        // Check if the attribute is `bitpack`
        if !meta_list.path.is_ident("bitpack") {
            continue;
        }

        // Find the `size` key-value pair
        for nested_meta in meta_list.nested.iter() {
            if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested_meta {
                if name_value.path.is_ident("size") {
                    if let Lit::Str(lit_str) = &name_value.lit {
                        return Some(lit_str.value());
                    }
                }
            }
        }
    }

    None
}

fn get_overflow_type(attrs: &[Attribute]) -> Option<bool> {
    for attr in attrs {
        // Parse the attribute to Meta
        let meta = attr.parse_meta().ok()?;

        // Check if the attribute is a list
        let Meta::List(meta_list) = meta else {
            continue;
        };

        // Check if the attribute is `bitwise_packable`
        if !meta_list.path.is_ident("bitpack") {
            continue;
        }

        // Find the `overflow` key-value pair
        for nested_meta in meta_list.nested.iter() {
            if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested_meta {
                if name_value.path.is_ident("overflow") {
                    if let Lit::Bool(lit_bool) = &name_value.lit {
                        return Some(lit_bool.value());
                    }
                }
            }
        }
    }

    None
}

#[proc_macro_derive(BitwisePackable, attributes(bitpack))]
pub fn bitwise_packable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let data = match &input.data {
        Data::Struct(data) => data,
        _ => panic!("BitwisePackable can only be used with structs"),
    };

    // Collect boolean fields
    let fields: Vec<&Field> = data
        .fields
        .iter()
        .filter(
            |f| matches!(&f.ty, Type::Path(syn::TypePath { path, .. }) if path.is_ident("bool")),
        )
        .collect();

    let num_fields = fields.len();
    let field_names: Vec<_> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let fields_idx: Vec<usize> = (0..num_fields).collect();

    let attrs = get_packing_type(&input.attrs);
    let overflow = get_overflow_type(&input.attrs).unwrap_or(false);
    let size = attrs.unwrap_or_else(|| "auto".to_string());

    let (pack_code, unpack_code) = match size.as_str() {
        "i8" => (
            quote! {
                impl #name {
                    pub fn pack(&self) -> u8 {
                        let mut result = 0u8;
                        let mut bit_index = 0;
                        let max_bits = 8;

                        // Single overflow check
                        if #num_fields > max_bits && !#overflow {
                            panic!(
                                "Overflow occurred during packing: struct '{}' has more boolean fields than can be packed in an u8 (8 bits).",
                                stringify!(#name)
                            );
                        }

                        #(
                            if bit_index < max_bits {
                                result |= (self.#field_names as u8) << bit_index;
                                bit_index += 1;
                            } // No additional else condition needed
                        )*
                        result
                    }

                    pub fn unpack(packed: u8) -> Self {
                        let mut bit_index = 0;

                        // Overflow check
                        if #num_fields > 8 && !#overflow {
                            panic!(
                                "Overflow occurred during unpacking: struct '{}' has more boolean fields than can be unpacked from an u8 (8 bits).",
                                stringify!(#name)
                            );
                        }

                        #(
                            let #field_names = if bit_index < 8 {
                                (packed & (1 << bit_index)) != 0
                            } else {
                                false
                            };
                            bit_index += 1;
                        )*
                        Self {
                            #(#field_names),*
                        }
                    }
                }
            },
            quote! {},
        ),

        "i16" => (
            quote! {
                impl #name {
                    pub fn pack(&self) -> u16 {
                        let mut result = 0u16;
                        let mut bit_index = 0;
                        let max_bits = 16;

                        // Single overflow check
                        if #num_fields > max_bits && !#overflow {
                            panic!(
                                "Overflow occurred during packing: struct '{}' has more boolean fields than can be packed in an u16 (16 bits).",
                                stringify!(#name)
                            );
                        }

                        #(
                            if bit_index < max_bits {
                                result |= (self.#field_names as u16) << bit_index;
                                bit_index += 1;
                            } // No additional else condition needed
                        )*
                        result
                    }

                    pub fn unpack(packed: u16) -> Self {
                        let mut bit_index = 0;

                        // Overflow check
                        if #num_fields > 16 && !#overflow {
                            panic!(
                                "Overflow occurred during unpacking: struct '{}' has more boolean fields than can be unpacked from an u16 (16 bits).",
                                stringify!(#name)
                            );
                        }

                        #(
                            let #field_names = if bit_index < 16 {
                                (packed & (1 << bit_index)) != 0
                            } else {
                                false
                            };
                            bit_index += 1;
                        )*
                        Self {
                            #(#field_names),*
                        }
                    }

                }
            },
            quote! {},
        ),

        "i32" => (
            quote! {
                impl #name {
                    pub fn pack(&self) -> u32 {
                        let mut result = 0u32;
                        let mut bit_index = 0;
                        let max_bits = 32;

                        // Single overflow check
                        if #num_fields > max_bits && !#overflow {
                            panic!(
                                "Overflow occurred during packing: struct '{}' has more boolean fields than can be packed in an u32 (32 bits).",
                                stringify!(#name)
                            );
                        }

                        #(
                            if bit_index < max_bits {
                                result |= (self.#field_names as u32) << bit_index;
                                bit_index += 1;
                            } // No additional else condition needed
                        )*
                        result
                    }

                    pub fn unpack(packed: u32) -> Self {
                        let mut bit_index = 0;

                        // Overflow check
                        if #num_fields > 32 && !#overflow {
                            panic!(
                                "Overflow occurred during unpacking: struct '{}' has more boolean fields than can be unpacked from an u32 (32 bits).",
                                stringify!(#name)
                            );
                        }

                        #(
                            let #field_names = if bit_index < 32 {
                                (packed & (1 << bit_index)) != 0
                            } else {
                                false
                            };
                            bit_index += 1;
                        )*
                        Self {
                            #(#field_names),*
                        }
                    }

                }
            },
            quote! {},
        ),

        "i64" => (
            quote! {
                impl #name {
                    pub fn pack(&self) -> u64 {
                        let mut result = 0u64;
                        let mut bit_index = 0;
                        let max_bits = 64;

                        // Single overflow check
                        if #num_fields > max_bits && !#overflow {
                            panic!(
                                "Overflow occurred during packing: struct '{}' has more boolean fields than can be packed in an u64 (64 bits).",
                                stringify!(#name)
                            );
                        }

                        #(
                            if bit_index < max_bits {
                                result |= (self.#field_names as u64) << bit_index;
                                bit_index += 1;
                            } // No additional else condition needed
                        )*
                        result
                    }

                    pub fn unpack(packed: u64) -> Self {
                        let mut bit_index = 0;

                        // Overflow check
                        if #num_fields > 64 && !#overflow {
                            panic!(
                                "Overflow occurred during unpacking: struct '{}' has more boolean fields than can be unpacked from an u64 (64 bits).",
                                stringify!(#name)
                            );
                        }

                        #(
                            let #field_names = if bit_index < 64 {
                                (packed & (1 << bit_index)) != 0
                            } else {
                                false
                            };
                            bit_index += 1;
                        )*
                        Self {
                            #(#field_names),*
                        }
                    }

                }
            },
            quote! {},
        ),

        _ => (
            quote! {
                impl #name {
                    pub fn pack(&self) -> Vec<u64> {
                        let num_fields = #num_fields;
                        let mut bitfield = Bitfield::new(num_fields);

                        // Single overflow check
                        if num_fields > bitfield.parts.len() * 64 && !#overflow {
                            panic!(
                                "Overflow occurred during packing: struct '{}' has more boolean fields than can be packed in the provided Bitfield size.",
                                stringify!(#name)
                            );
                        }

                        let mut bit_index = 0;
                        #(
                            if bit_index < num_fields {
                                bitfield.set(bit_index, self.#field_names);
                                bit_index += 1;
                            }
                        )*

                        bitfield.parts
                    }

                    pub fn unpack(packed: Vec<u64>) -> Self {
                        let num_fields = #num_fields;
                        let bitfield = Bitfield {
                            parts: packed,
                        };

                        // Overflow check
                        if num_fields > bitfield.parts.len() * 64 && !#overflow {
                            panic!(
                                "Overflow occurred during unpacking: struct '{}' has more boolean fields than can be unpacked from the provided Bitfield size.",
                                stringify!(#name)
                            );
                        }

                        let mut booleans = vec![false; num_fields];
                        for i in 0..num_fields {
                            booleans[i] = bitfield.get(i);
                        }

                        Self {
                            #(
                                #field_names: booleans[#fields_idx],
                            )*
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
