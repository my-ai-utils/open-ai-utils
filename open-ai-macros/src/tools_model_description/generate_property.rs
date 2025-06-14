use proc_macro2::TokenStream;
use types_reader::{AnyValueAsStr, PropertyType, StructProperty};

pub fn generate_property(prop: StructProperty) -> Result<TokenStream, syn::Error> {
    let prop_name = prop.name.as_str();
    let attr = prop.attrs.get_attr("property")?;

    let enum_param = attr.try_get_named_param("enum");

    let enum_to_render = match enum_param {
        Some(enum_value) => {
            let array = enum_value.unwrap_as_vec()?;

            let mut array_as_tokens = Vec::new();

            for itm in array {
                let as_str = itm.unwrap_as_value()?.as_str()?;
                array_as_tokens.push(quote::quote! {#as_str, });
            }

            quote::quote! {Some( &[#(#array_as_tokens)*] )}
        }
        None => quote::quote! {None},
    };

    let value = attr.get_value_from_single_or_named("description")?;
    let value = value.as_string()?;
    let value = value.as_str();

    let result = if let PropertyType::OptionOf(opt_tp) = prop.ty {
        let as_token = opt_tp.get_token_stream();
        quote::quote! {
           properties.insert(
            #prop_name.into(),
            Option::<#as_token>::get_type_description(#value, None, #enum_to_render),
        );
         }
    } else {
        let token = prop.ty.get_token_stream();
        quote::quote! {
           properties.insert(
            #prop_name.into(),
            #token::get_type_description(#value, None, #enum_to_render),
        );
         }
    };

    Ok(result)
}
