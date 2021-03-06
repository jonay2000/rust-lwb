use crate::codegen::error::CodegenError;
use crate::codegen::sanitize_identifier;
use crate::parser::peg::parser_sugar_ast::{Annotation, SyntaxFileAst};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_trait_impls(syntax: &SyntaxFileAst) -> Result<TokenStream, CodegenError> {
    let mut impls = Vec::new();

    for sort in syntax.sorts.values() {
        if sort.annotations.contains(&Annotation::Hidden) {
            continue;
        }

        let sortname = format_ident!("{}", sanitize_identifier(&sort.name));
        let sortname_str = &sort.name;

        let constructor_names = sort
            .constructors
            .iter()
            .filter(|i| !i.dont_put_in_ast)
            .filter(|i| {
                !i.annotations
                    .iter()
                    .any(|i| matches!(i, Annotation::Error(_)))
            })
            .map(|i| format_ident!("{}", sanitize_identifier(&i.name)))
            .collect_vec();

        let constructor_names_str = sort
            .constructors
            .iter()
            .filter(|i| !i.dont_put_in_ast)
            .filter(|i| {
                !i.annotations
                    .iter()
                    .any(|i| matches!(i, Annotation::Error(_)))
            })
            .map(|i| i.name.as_str())
            .collect_vec();

        let (ast_info_body, constructor_body) = if constructor_names.len() == 1 {
            let constructor_name_str = &constructor_names_str[0];
            (
                quote!(
                    let Self (meta, ..) = self;
                    meta
                ),
                quote!(
                    #constructor_name_str
                ),
            )
        } else {
            (
                quote!(
                    match self {
                        #(
                            Self::#constructor_names (meta, ..) => meta
                        ),*,
                        _ => unreachable!()
                    }
                ),
                quote!(
                    match self {
                        #(
                            Self::#constructor_names (..) => #constructor_names_str
                        ),*,
                        _ => unreachable!()
                    }
                ),
            )
        };

        impls.push(quote!(
            impl<M: AstInfo> AstNode<M> for #sortname<M> {
                fn ast_info(&self) -> &M {
                    #ast_info_body
                }

                fn constructor(&self) -> &'static str {
                    #constructor_body
                }

                fn sort(&self) -> &'static str {
                    #sortname_str
                }
            }
        ));
    }

    Ok(quote!(
        use super::prelude::*;

        #(#impls)*
    ))
}
