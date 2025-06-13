use proc_macro::TokenStream;

extern crate proc_macro;
mod tools_model_description;

#[proc_macro_derive(OpenAiFunctionModel, attributes(property, function_description))]
pub fn open_ai_function_description(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    match crate::tools_model_description::generate(&ast) {
        Ok(result) => result.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
