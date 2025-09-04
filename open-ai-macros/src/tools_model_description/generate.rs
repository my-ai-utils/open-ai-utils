use proc_macro::TokenStream;
use types_reader::StructProperty;
pub fn generate(input: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    //    let input_token_stream: proc_macro2::TokenStream = input.clone().into();

    let struct_name = &input.ident;

    let fields = StructProperty::read(input)?;

    let mut fields_to_render = Vec::new();

    let mut required_fields = Vec::new();

    for prop in fields {
        if !prop.ty.is_option() {
            required_fields.push(prop.name.to_string());
        }

        let property = super::generate_property(prop)?;

        fields_to_render.push(property);
    }

    let result = quote::quote! {

        #[async_trait::async_trait]
        impl open_ai_utils::FunctionToolCallDescription  for #struct_name{

        async fn get_description() -> open_ai_utils::my_json::json_writer::JsonObjectWriter {
        use open_ai_utils::FunctionTypeDescription;

        let props = open_ai_utils::my_json::json_writer::JsonObjectWriter::new()

        #(#fields_to_render)*;

        open_ai_utils::my_json::json_writer::JsonObjectWriter::new().write("type", "object")
        .write("properties", props )
        .write_iter("required", [#(#required_fields,)*].into_iter())
        .write("additionalProperties", false)

       }

      }

    };
    Ok(result.into())
}
