use proc_macro::TokenStream;
use types_reader::StructProperty;
pub fn generate(input: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    //    let input_token_stream: proc_macro2::TokenStream = input.clone().into();

    let struct_name = &input.ident;

    let fields = StructProperty::read(input)?;

    let mut fields_to_render = Vec::new();

    let mut function_description = None;

    for prop in fields {
        if let Some(fd) = prop.attrs.try_get_attr("function_description") {
            if function_description.is_some() {
                return Err(fd.throw_error_at_param_token("Single function_description is allowed"));
            }

            let description = fd.get_named_param("description")?;
            let name = fd.get_named_param("name")?;

            function_description = Some((
                name.unwrap_any_value_as_str()?.as_str()?.to_string(),
                description.unwrap_any_value_as_str()?.as_str()?.to_string(),
            ));
        }

        let property = super::generate_property(prop)?;
        fields_to_render.push(property);
    }

    if function_description.is_none() {
        panic!("Single function_description attribute is required");
    }

    let (func_name, function_description) = function_description.unwrap();

    let result = quote::quote! {

        impl #struct_name{

          pub fn get_description() -> serde_json::Value {
         use open_ai_utils::FunctionTypeDescription;
        let mut result = serde_json::Map::new();

        result.insert("type".into(), "function".to_owned().into());

        let mut function = serde_json::Map::new();

        function.insert("name".into(), #func_name.to_owned().into());

        let mut params = serde_json::Map::new();
        params.insert("type".into(), "object".into());

        let mut properties = serde_json::Map::new();

        #(#fields_to_render)*


        params.insert("properties".into(), properties.into());

        params.insert("required".into(), serde_json::Value::Array(vec![]));
        params.insert("additionalProperties".into(), false.into());

        function.insert("parameters".into(), params.into());

        function.insert("description".into(), #function_description.to_owned().into());

        result.insert("function".into(), function.into());

        serde_json::Value::Object(result)
          }

        }

    };
    Ok(result.into())
}

/*
   fn get_description()->serde_json:Value{

            }
*/
