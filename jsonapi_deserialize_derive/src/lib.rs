use darling::{ast, FromDeriveInput, FromField, FromMeta};
use heck::{ToLowerCamelCase, ToPascalCase, ToSnakeCase};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Generics, Type};

#[proc_macro_derive(JsonApiDeserialize, attributes(json_api))]
pub fn json_api_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_json_api_deserialize(&input).into()
}

#[derive(Debug, FromMeta)]
#[darling(default)]
#[allow(clippy::enum_variant_names)]
enum RenameAll {
    CamelCase,
    PascalCase,
    SnakeCase,
}

impl Default for RenameAll {
    fn default() -> Self {
        Self::CamelCase
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(json_api), supports(struct_any))]
struct InputReceiver {
    ident: Ident,
    #[allow(dead_code)]
    generics: Generics,
    data: ast::Data<(), FieldReceiver>,
    resource_type: Option<String>,
    #[darling(default)]
    rename_all: RenameAll,
}

#[derive(Debug, FromMeta)]
enum Relationship {
    Single,
    Optional,
    Multiple,
}

#[derive(Debug, FromField)]
#[darling(attributes(json_api))]
struct FieldReceiver {
    ident: Option<Ident>,
    #[allow(dead_code)]
    ty: Type,
    relationship: Option<Relationship>,
    resource: Option<Type>,
    rename: Option<String>,
}

fn impl_json_api_deserialize(input: &DeriveInput) -> proc_macro2::TokenStream {
    let input_receiver = InputReceiver::from_derive_input(input).unwrap();
    let struct_name = input_receiver.ident;
    let resource_type = input_receiver
        .resource_type
        .unwrap_or_else(|| struct_name.to_string().to_snake_case());

    let mut field_initializers = proc_macro2::TokenStream::new();
    let mut fields = proc_macro2::TokenStream::new();

    input_receiver.data.map_struct_fields(|field| {
        let field_name = match field.ident {
            Some(field_name) => field_name,
            None => return,
        };

        let json_field_name = match field.rename {
            Some(rename) => rename,
            None => match input_receiver.rename_all {
                RenameAll::CamelCase => field_name.to_string().to_lower_camel_case(),
                RenameAll::PascalCase => field_name.to_string().to_pascal_case(),
                RenameAll::SnakeCase => field_name.to_string().to_snake_case(),
            },
        };

        let field_tokens = match field.relationship {
            Some(Relationship::Single) => {
                let mut field_tokens = quote! {
                    let #field_name = serde_json::from_value::<jsonapi_deserialize::RawSingleRelationship>(
                        data
                            .get("relationships")
                            .ok_or_else(|| Error::MissingRelationships)?
                            .get(#json_field_name)
                            .ok_or_else(|| Error::MissingField(stringify!(#field_name)))?
                            .clone(),
                    )?.data;
                };

                if let Some(resource) = field.resource {
                    field_tokens.extend(quote! {
                        let #field_name = included_map.get::<#resource>(&#field_name.kind, &#field_name.id)?;
                    });
                }

                field_tokens
            }
            Some(Relationship::Optional) => {
                let mut field_tokens = quote! {
                    let #field_name = serde_json::from_value::<jsonapi_deserialize::RawOptionalRelationship>(
                        data
                            .get("relationships")
                            .ok_or_else(|| Error::MissingRelationships)?
                            .get(#json_field_name)
                            .ok_or_else(|| Error::MissingField(stringify!(#field_name)))?
                            .clone(),
                    )?.data;
                };

                if let Some(resource) = field.resource {
                    field_tokens.extend(quote! {
                        let #field_name = match #field_name {
                            Some(data) => Some(included_map.get::<#resource>(&data.kind, &data.id)?),
                            None => None,
                        };
                    });
                }

                field_tokens
            }
            Some(Relationship::Multiple) => {
                let mut field_tokens = quote! {
                    let #field_name = serde_json::from_value::<jsonapi_deserialize::RawMultipleRelationship>(
                        data
                            .get("relationships")
                            .ok_or_else(|| Error::MissingRelationships)?
                            .get(#json_field_name)
                            .ok_or_else(|| Error::MissingField(stringify!(#field_name)))?
                            .clone(),
                    )?.data;
                };

                if let Some(resource) = field.resource {
                    field_tokens.extend(quote! {
                        let #field_name = #field_name
                            .into_iter()
                            .map(|data| included_map.get::<#resource>(&data.kind, &data.id))
                            .collect::<Result<_, _>>()?;
                    });
                }

                field_tokens
            }
            None => {
                if field_name == "id" {
                    quote! {
                        let #field_name = serde_json::from_value(
                            data
                                .get("id")
                                .ok_or_else(|| Error::MissingId)?
                                .clone(),
                        )?;
                    }
                } else {
                    quote! {
                        let #field_name = serde_json::from_value(
                            data
                                .get("attributes")
                                .ok_or_else(|| Error::MissingAttributes)?
                                .get(#json_field_name)
                                .ok_or_else(|| Error::MissingField(stringify!(#field_name)))?
                                .clone(),
                        )?;
                    }
                }
            }
        };

        field_initializers.extend(field_tokens);
        fields.extend(quote! { #field_name, });
    });

    quote! {
        impl jsonapi_deserialize::JsonApiDeserialize for #struct_name {
            fn from_value(
                value: &serde_json::Value,
                included_map: &mut jsonapi_deserialize::IncludedMap,
            ) -> Result<Self, jsonapi_deserialize::DeserializeError> {
                use jsonapi_deserialize::DeserializeError as Error;

                let data = value.as_object().ok_or_else(|| Error::InvalidType("Expected an object"))?;

                let resource_type: String = serde_json::from_value(
                    data
                        .get("type")
                        .ok_or_else(|| Error::MissingResourceType)?
                        .clone(),
                )?;

                if resource_type != #resource_type {
                    return Err(Error::ResourceTypeMismatch {
                        expected: #resource_type.to_string(),
                        found: resource_type,
                    });
                }

                #field_initializers

                Ok(Self {
                    #fields
                })
            }
        }
    }
}
