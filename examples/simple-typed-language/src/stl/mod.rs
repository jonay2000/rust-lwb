#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
// |==========================================================|
// |      WARNING: THIS FILE IS AUTOMATICALLY GENERATED.      |
// |      CHANGES TO IT WILL BE DELETED WHEN REGENERATED.     |
// | IN GENERAL, THIS FILE SHOULD NOT BE MODIFIED IN ANY WAY. |
// |==========================================================|
// Generated at 17/04/2022 23:43:35 +02:00 - 17/04/2022 21:43:35 UTC

#[rustfmt::skip]
mod ast;
pub use ast::*;

#[rustfmt::skip]
mod from_pairs;
pub use from_pairs::*;

#[rustfmt::skip]
mod ast_impls;
pub use ast_impls::*;

#[rustfmt::skip]
mod parser;
pub use parser::*;

#[rustfmt::skip]
mod prelude {
    pub use rust_lwb::codegen_prelude::*;
    pub use super::*;
}