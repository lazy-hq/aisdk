#![deny(missing_docs)]
//! Macros for the `aisdk` library.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{
    Expr, ExprLit, FnArg, ItemFn, Lit, Meta, MetaNameValue, Pat, PatType, Path, Token, Type,
    parse_macro_input, punctuated::Punctuated,
};

#[proc_macro_attribute]
/// Constructs a tool from a function defnition. A tool has a name, a description,
/// an input and a body. all three components are infered from a standard rust
/// function. The name is the defined name of the function,
/// The description is infered from the doc comments of the function, The input
/// infered from the function arguments.
///
/// # Example
///
/// ```rust,no_run
/// use aisdk::macros::tool;
/// use aisdk::core::tools::Tool;
///
/// #[tool]
/// /// Returns the username
/// fn get_username(id: String) -> Tool {
///     // Your code here
///     Ok(format!("user_{}", id))
/// }
/// ```
///
/// - `get_username` becomes the name of the tool
/// - `"Returns the username"` becomes the description of the tool
/// - `id: String` becomes the input of the tool. converted to `{"id": "string"}`
///   as json schema
///
/// The function should return a `Result<String, String>` eventhough the return statement
/// returns a `Tool` object. This is because the macro will automatically convert the
/// function into a `Tool` object and return it. You should return what the model can
/// understand as a `String`.
///
/// In the event that the model refuses to send an argument, the default implementation
/// will be used. this works perfectly for arguments that are `Option`s. Make sure to
/// use `Option` types for arguments that are optional or implement a default for those
/// that are not and handle those defaults accordingly in the tool body.
///
/// A single parameter typed as `ToolContext` is treated as runtime context and is not
/// included in the schema sent to the model.
///
/// You can override name and description using the macro arguments `name` and `desc`.
///
/// # Example with overrides
/// ```rust,no_run
/// use aisdk::macros::tool;
/// use aisdk::core::tools::Tool;
///
///     #[tool(
///         name = "the-name-for-this-tool",
///         desc = "the-description-for-this-tool"
///     )]
///     fn get_username(id: String) -> Tool {
///         // Your code here
///         Ok(format!("user_{}", id))
///     }
/// ```
pub fn tool(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let vis = &input_fn.vis;
    let return_type = &input_fn.sig.output;
    let is_async = input_fn.sig.asyncness.is_some();
    let block = &input_fn.block;
    let inputs = &input_fn.sig.inputs;
    let attrs = &input_fn.attrs;
    let args_parser = Punctuated::<MetaNameValue, Token![,]>::parse_terminated;
    let args = args_parser.parse(_attr);

    let (name_arg, description_arg) = if let Ok(args) = args {
        let mut name: Option<String> = None;
        let mut description: Option<String> = None;

        for arg in args {
            if arg.path.is_ident("desc")
                && let Expr::Lit(lit) = &arg.value
                && let Lit::Str(str_lit) = &lit.lit
            {
                description = Some(str_lit.value());
            } else if arg.path.is_ident("name")
                && let Expr::Lit(lit) = &arg.value
                && let Lit::Str(str_lit) = &lit.lit
            {
                name = Some(str_lit.value());
            }
        }

        (name, description)
    } else {
        (None, None)
    };

    let description = if let Some(desc) = description_arg {
        desc
    } else {
        // Extract doc comments
        let doc_comments: Vec<String> = attrs
            .iter()
            .filter_map(|attr| {
                if attr.path().is_ident("doc") {
                    if let Meta::NameValue(meta_name_value) = &attr.meta {
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        }) = &meta_name_value.value
                        {
                            let doc = lit_str.value();
                            Some(doc)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        doc_comments.join("\n")
    };

    let name = if let Some(name) = name_arg {
        name
    } else {
        fn_name.to_string()
    };

    let mut context_count = 0usize;
    let mut binding_tokens = Vec::new();
    let mut struct_fields = Vec::new();

    for pat_type in inputs.iter().filter_map(|arg| match arg {
        FnArg::Typed(pat_type) => Some(pat_type),
        FnArg::Receiver(_) => None,
    }) {
        let (ident, ty, is_context) = match parse_tool_parameter(pat_type) {
            Ok(parameter) => parameter,
            Err(error) => return error.to_compile_error().into(),
        };

        if is_context {
            context_count += 1;
            if context_count > 1 {
                return syn::Error::new_spanned(
                    pat_type,
                    "only one ToolContext parameter is supported",
                )
                .to_compile_error()
                .into();
            }

            binding_tokens.push(quote! {
                let #ident: #ty = _ctx.clone();
            });
            continue;
        }

        let ident_str = ident.to_string();
        binding_tokens.push(quote! {
            let #ident: #ty = ::aisdk::__private::serde_json::from_value(
                inp.as_object()
                    .unwrap()
                    .get(#ident_str)
                    .unwrap()
                    .clone()
            ).unwrap_or_default();  // use default value if model doesn't send arg
        });
        struct_fields.push(quote! { #ident: #ty });
    }

    let execute_impl = if is_async {
        quote! {
            ::aisdk::core::tools::ToolExecute::from_async(|_ctx, inp| async move {
                #(#binding_tokens)*
                #block
            })
        }
    } else {
        quote! {
            ::aisdk::core::tools::ToolExecute::from_sync(|_ctx, inp| {
                #(#binding_tokens)*
                #block
            })
        }
    };

    let expanded = quote! {
        #vis fn #fn_name() #return_type  {
            // use schemars::{schema_for, JsonSchema, Schema};
            use std::collections::HashMap;
            use ::aisdk::__private::schemars::{schema_for, JsonSchema, Schema};

            #[derive(::aisdk::__private::schemars::JsonSchema, Debug)]
            #[schemars(crate = "::aisdk::__private::schemars")]
            #[allow(dead_code)]
            //#[schemars(deny_unknown_fields)]
            struct Function {
                // Please add struct fields here
                #(#struct_fields),*
            }

            let input_schema = schema_for!(Function);
            // End

            let mut tool = ::aisdk::core::tools::Tool::builder()
                .name(#name.to_string())
                .description(#description.to_string())
                .input_schema(input_schema)
                .execute(#execute_impl);

            tool.build().expect("Failed to build tool")
        }
    };

    TokenStream::from(expanded)
}

fn parse_tool_parameter(pat_type: &PatType) -> syn::Result<(syn::Ident, Type, bool)> {
    let Pat::Ident(pat_ident) = &*pat_type.pat else {
        return Err(syn::Error::new_spanned(
            &pat_type.pat,
            "#[tool] only supports identifier parameters",
        ));
    };

    let ident = pat_ident.ident.clone();
    let ty = (*pat_type.ty).clone();
    Ok((ident, ty.clone(), is_tool_context_type(&ty)))
}

/// Checks if the given type is a ToolContext type.
fn is_tool_context_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(last_segment) = type_path.path.segments.last() else {
        return false;
    };

    if last_segment.ident != "ToolContext"
        || !matches!(last_segment.arguments, syn::PathArguments::None)
    {
        return false;
    }

    is_supported_tool_context_path(&type_path.path)
}

/// Checks if the given path is a supported ToolContext path.
fn is_supported_tool_context_path(path: &Path) -> bool {
    let segments: Vec<_> = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    let segments: Vec<_> = segments.iter().map(String::as_str).collect();

    matches!(
        segments.as_slice(),
        ["ToolContext"]
            | ["tools", "ToolContext"]
            | ["core", "tools", "ToolContext"]
            | ["aisdk", "core", "tools", "ToolContext"]
    )
}
