use proc_macro::TokenStream;
use types_reader::StructProperty;
pub fn generate(input: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    //    let input_token_stream: proc_macro2::TokenStream = input.clone().into();

    let struct_name = &input.ident;

    let fields = StructProperty::read(input)?;

    let mut fields_to_render = Vec::new();

    for prop in fields {
        let property = super::generate_property(prop)?;
        fields_to_render.push(property);
    }

    let result = quote::quote! {

        impl open_ai_utils::FunctionToolCallDescription  for #struct_name{

        fn get_description() -> serde_json::Value {
        use open_ai_utils::FunctionTypeDescription;

        let mut params = serde_json::Map::new();
        params.insert("type".into(), "object".into());

        let mut properties = serde_json::Map::new();

        #(#fields_to_render)*


        params.insert("properties".into(), properties.into());

        params.insert("required".into(), serde_json::Value::Array(vec![]));
        params.insert("additionalProperties".into(), false.into());


        serde_json::Value::Object(params)
       }

      }

    };
    Ok(result.into())
}
