use std::{collections::HashMap, vec};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, punctuated::Punctuated, Expr, FnArg, Ident, ItemFn, Lit};

#[proc_macro_attribute]
pub fn genai_tool(args: TokenStream, item: TokenStream) -> TokenStream {
    let genai_tool_name: Ident = parse(args).expect("type name expected as argument");
    let item_fn: ItemFn = parse(item.clone()).expect("only free style functions supported");
    generate_tool_specification(item_fn, genai_tool_name)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn generate_tool_specification(
    ItemFn {
        attrs,
        vis,
        sig,
        block,
    }: ItemFn,
    genai_tool_name: Ident
) -> syn::Result<proc_macro2::TokenStream> {
    let mut edited_sig = sig.clone();
    let fn_ident = sig.ident.clone();
    let tool_name = sig.ident.clone().to_string();
    let mut inputs = vec![];
    let mut descriptions: HashMap<usize, String> = HashMap::new();
    for (index, input) in edited_sig.inputs.into_iter().enumerate() {
        if let FnArg::Typed(pt) = input.clone() {
            let mut pt = pt.clone();
            pt.attrs = pt
                .attrs
                .iter()
                .filter_map(|attr| {
                    if attr.path().is_ident("description") {
                        let Expr::Lit(desc_lit) = attr.parse_args().unwrap() else {
                            return None;
                        };
                        let Lit::Str(desc) = desc_lit.lit else {
                            return None;
                        };
                        descriptions.insert(index, desc.value());
                        None
                    } else {
                        Some(attr.clone())
                    }
                })
                .collect::<Vec<_>>();
            inputs.push(FnArg::Typed(pt));
        } else {
            inputs.push(input);
        }
    }
    edited_sig.inputs = Punctuated::new();
    for input in inputs {
        edited_sig.inputs.push_value(input);
        edited_sig
            .inputs
            .push_punct(syn::token::Comma(sig.ident.span()));
    }

    #[rustfmt::skip]
    let description_str = attrs
        .iter()
        .filter_map(|a| {
            let syn::Meta::NameValue(ref meta) = a.meta else { return None };
            if !meta.path.is_ident("doc") { return None };
            
            let Expr::Lit(ref value) = meta.value else { return None };
            let Lit::Str(ref comment) = value.lit else { return None };
            return Some(comment.value().trim().to_string());
        })
        .collect::<Vec<_>>()
        .join(" ");

    let parameters = sig
        .inputs
        .iter()
        .filter_map(|p| {
            let FnArg::Typed(pat_type) = p else {
                return None;
            };
            Some(pat_type)
        })
        .collect::<Vec<_>>();

    let parameters_specs = parameters
        .clone()
        .iter()
        .enumerate()
        .map(|(index, pat_type)| {
            let pn = &pat_type.pat;
            let ty = &pat_type.ty;
            let description: String = descriptions.remove(&index).unwrap_or("".into());
            let spec = quote! {
                let spec = schemars::schema_for!(#ty);
                let mut spec = serde_json::to_value(spec).unwrap();
                {
                    let mut spec = spec.as_object_mut().unwrap();
                    if (#description.len() > 0) {
                     spec.insert("description".into(), Value::String(#description.to_string()));
                    }
                    spec.remove_entry("$schema");
                    spec.remove_entry("title");
                }
                spec
            };

            quote! {
                let #pn = { #spec };
            }
        })
        .collect::<Vec<_>>();

    let parameter_names = parameters
        .clone()
        .iter()
        .map(|pat_type| {
            let pn = &pat_type.pat;
            let pn_name = quote! {#pn}.to_string();
            quote! {
                #pn_name
            }
        })
        .collect::<Vec<_>>();
    let parameter_assigns = parameters
        .clone()
        .iter()
        .map(|pat_type| {
            let pn = &pat_type.pat;
            let pn_name = quote! {#pn}.to_string();
            quote! {
                #pn_name: #pn
            }
        })
        .collect::<Vec<_>>();

    let parameter_extraction = parameters
        .clone()
        .iter()
        .map(|pat_type| {
            let pn = &pat_type.pat;
            let pn_name = quote! {#pn}.to_string();
            let arg_type = &pat_type.ty;
            quote! {
                let #pn: #arg_type = serde_json::from_value(arguments[#pn_name].clone()).unwrap();
            }
        })
        .collect::<Vec<_>>();

    let parameter_idents = parameters
        .clone()
        .iter()
        .map(|pat_type| {
            let pn = &pat_type.pat;
            quote! { #pn }
        })
        .collect::<Vec<_>>();

    Ok(quote::quote! {
        #(#attrs)*
        #vis #edited_sig
        #block

        struct #genai_tool_name;
        impl #genai_tool_name {
            fn specification() -> genai::tools::ToolSpecification {

                #(#parameters_specs);*

                let spec = genai::tools::ToolSpecification {
                    name: String::from(#tool_name),
                    description: String::from(#description_str),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            #(#parameter_assigns),*
                        },
                        "required": [#(#parameter_names),*]
                    })
                };
                spec
            }

            fn handler( arguments: serde_json::Value ) -> serde_json::Value {
                #(#parameter_extraction);*;

                let result = #fn_ident(#(#parameter_idents),*);

                serde_json::json!(result)

            }

            fn tool() -> genai::tools::GenAITool {
                genai::tools::GenAITool { 
                    specification: #genai_tool_name::specification(),
                    handler: #genai_tool_name::handler
                }
            }
        }
    })
}
