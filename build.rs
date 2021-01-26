use heck::SnakeCase;
#[cfg(feature = "download-schema")]
use isahc::prelude::*;
use quote::{__private::TokenStream, format_ident, quote};
use serde::Deserialize;
use std::collections::HashMap;

#[cfg(feature = "download-schema")]
const MASTER_SCHEMA_URL: &str =
    "https://raw.githubusercontent.com/deepnight/ldtk/{version}/docs/JSON_SCHEMA.json";

#[derive(Debug, Clone, Deserialize)]
struct JsonSchema {
    #[serde(rename = "LdtkJsonRoot")]
    root: Definition,
    #[serde(rename = "otherTypes")]
    other_types: HashMap<String, Definition>,
    #[serde(rename = "$ref")]
    r#ref: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Definition {
    description: Option<String>,
    required: Vec<String>,
    properties: HashMap<String, Property>,
}

#[derive(Debug, Clone, Deserialize)]
struct Property {
    description: Option<String>,
    items: Option<PropertyType>,
    #[serde(flatten)]
    r#type: Option<PropertyType>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum PropertyType {
    Ref {
        #[serde(rename = "$ref")]
        r#ref: String,
    },
    Primitive {
        r#type: Vec<JsonType>,
    },
    Nothing {},
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum JsonType {
    Array,
    Integer,
    Number,
    Object,
    String,
    Boolean,
    Null,
    Ref {
        #[serde(rename = "$ref")]
        r#ref: String,
    },
}

fn get_type_tokens(ty: &JsonType, items: &Option<PropertyType>) -> TokenStream {
    match ty {
        JsonType::Boolean => quote! {
            bool
        },
        JsonType::Array => {
            let t = items.as_ref().unwrap();

            let inner = match t {
                PropertyType::Ref { r#ref } => {
                    let ident = format_ident!("{}", r#ref.strip_prefix("#/otherTypes/").unwrap());

                    quote! {
                        #ident
                    }
                }
                PropertyType::Primitive { r#type } => {
                    let t = if r#type.len() > 1 {
                        // TODO: If there are multiple possible types, just use a `Value`
                        quote! {
                            Value
                        }
                    } else {
                        get_type_tokens(r#type.get(0).unwrap(), &None)
                    };

                    quote! {
                        #t
                    }
                }
                PropertyType::Nothing {} => quote! {
                    Value
                },
            };

            quote! {
                Vec<#inner>
            }
        }
        JsonType::Number => quote! {
            f32
        },
        JsonType::Integer => quote! {
            i32
        },
        JsonType::Object => quote! {
            HashMap<String, Value>
        },
        JsonType::String => quote! {
            String
        },
        JsonType::Null => unreachable!(),
        JsonType::Ref { r#ref } => {
            let i = format_ident!("{}", r#ref.strip_prefix("#/otherTypes/").unwrap());

            quote! {
                #i
            }
        }
    }
}

fn main() {
    #[cfg(feature = "download-schema")]
    let version = std::env::var("LDTK_VERSION").unwrap_or("master".into());
    #[cfg(feature = "download-schema")]
    let schema: JsonSchema = isahc::get(MASTER_SCHEMA_URL.replace("{version}", &version))
        .unwrap()
        .json()
        .unwrap();

    #[cfg(not(feature = "download-schema"))]
    let version = std::env::var("LDTK_VERSION").unwrap_or("v0.7.0".into());
    #[cfg(not(feature = "download-schema"))]
    let schema: JsonSchema = serde_json::from_reader(
        std::fs::OpenOptions::new()
            .read(true)
            .open(format!(
                concat!(env!("CARGO_MANIFEST_DIR"), "/schemas/{}.json"),
                version
            ))
            .unwrap(),
    )
    .unwrap();

    let mut tokens = quote! {
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        use serde_json::Value;

        // Custom tile flip struct for making interaction nicer
        /// Whether or not the tile is flipped on the x and/or y axes.
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct TileFlip {
            pub x: bool,
            pub y: bool,
        }

        impl<'de> Deserialize<'de> for TileFlip {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let bits: i32 = Deserialize::deserialize(deserializer)?;
                let x_bit = 0b01;
                let y_bit = 0b10;

                let mut flip = Self { x: false, y: false };

                if bits & x_bit != 0 {
                    flip.x = true;
                }
                if bits & y_bit != 0 {
                    flip.y = true;
                }

                Ok(flip)
            }
        }

        impl Serialize for TileFlip {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                let mut flip_bits = 0;
                let x_bit = 0b01;
                let y_bit = 0b10;

                if self.x {
                    flip_bits = flip_bits | x_bit;
                }
                if self.y {
                    flip_bits = flip_bits | y_bit;
                }

                serializer.serialize_i32(flip_bits)
            }
        }
    };

    for (def_name, def) in schema
        .other_types
        .iter()
        .chain(vec![(&String::from("Project"), &schema.root)])
    {
        // Build fields
        let mut fields = vec![];
        for (field_name, field) in &def.properties {
            // Get the doc comment
            let doc = field
                .description
                .as_ref()
                .map(Clone::clone)
                .unwrap_or(String::new());

            // If the field name is `type` prefix it with the struct name but converted to
            // snake case.
            let new_field_name = if field_name == "type" {
                format!("{}_{}", def_name.to_snake_case(), field_name)
            } else {
                field_name.clone()
            };

            // Create an identifier for the field, converting it to snake case, and making
            // sure to preserve the `__` prefix which is lost in snake case conversion.
            let field_ident = format_ident!(
                "r#{}{}",
                if new_field_name.starts_with("__") {
                    "__"
                } else {
                    ""
                },
                new_field_name.to_snake_case()
            );

            // Create the field type
            let optional = !def.required.contains(&field_name);
            let t = if field_name == "f" && def_name == "Tile" {
                quote! {
                    TileFlip
                }
            } else if let Some(field_type) = &field.r#type {
                match field_type {
                    PropertyType::Ref { r#ref } => {
                        let i = format_ident!("{}", r#ref.strip_prefix("#/otherTypes/").unwrap());
                        quote! {
                            #i
                        }
                    }
                    PropertyType::Primitive { r#type } => {
                        // Remove "null" as we handle that through the required option
                        let types = r#type
                            .iter()
                            .filter(|&x| x != &JsonType::Null)
                            .collect::<Vec<_>>();

                        // If ther are multiple possible types, just make it a JSON value for now
                        // TODO: Make an enum for the possible types?
                        if types.len() > 1 {
                            quote! {
                                Value
                            }
                        } else {
                            get_type_tokens(types.get(0).unwrap(), &field.items)
                        }
                    }
                    PropertyType::Nothing {} => quote! {
                        Value
                    },
                }
            } else {
                quote! {
                    serde_json::Value
                }
            };
            let mut t = quote! {
                #t
            };
            if optional {
                t = quote! {
                    Option<#t>
                };
            }

            fields.push(quote! {
                #[doc = #doc]
                #[serde(rename = #field_name)]
                pub #field_ident: #t
            });
        }

        // Create Rust Struct
        let struct_name = format_ident!("{}", def_name);
        let doc = def
            .description
            .as_ref()
            .map(Clone::clone)
            .unwrap_or(String::new());
        tokens.extend(quote! {
            #[doc = #doc]
            #[derive(Clone, Debug, Serialize, Deserialize)]
            pub struct #struct_name {
                #( #fields ),*
            }
        });
    }

    // Write the generated rust to a file
    std::fs::write(
        format!("{}/schema.rs", std::env::var("OUT_DIR").unwrap()),
        tokens.to_string(),
    )
    .unwrap();
}
