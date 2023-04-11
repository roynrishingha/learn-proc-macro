//! The `proc_macro` crate is the compiler's API that allows us to read and manipulate Rust code from our code.
//! The `syn` crate parses Rust code from a string into a data structure that we can perform operations on.
//! The `quote` crate turns `syn` data structures back into Rust code

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};

// This funtion will be called when a user of this library specifies `#[derive(LearnProcMacro)]` on a type.
// This is possible because I've annotated the `learn_proc_macro_derive` function here with `proc_macro_derive` and specified the name `LearnProcMacro`, which matches our trait name.
// This is the convention most procedural macros follow.
#[proc_macro_derive(LearnProcMacro)]
pub fn learn_proc_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    // one way of parsing
    // let ast = syn::parse(input).expect("failed to parse");
    let ast = parse_macro_input!(input as DeriveInput);

    // Generate the output tokens using ast
    // macth for token generation Error handling
    match impl_learn_proc_macro(&ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn impl_learn_proc_macro(ast: &DeriveInput) -> Result<TokenStream> {
    let name = &ast.ident;
    let gen = quote! {
        impl LearnProcMacro for #name {
           fn learn_proc_macro() {
            println!("Hello, I am a {}", stringify!(#name));
           }
        }
    };
    Ok(gen.into())
}
