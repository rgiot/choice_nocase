#![feature(extend_one)]

use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenTree, Punct, Spacing};
use syn::{parse_macro_input};

///
/// Transform a string as a list of strings separated by | where all combinations of character cases are generated.
/// By construction first one is fully uppercase, second one is fully lowercase.
///
/// `choice_nocase!("ab")` is replaced by `"AB" | "ab" | "aB" | "Ab" `
#[proc_macro]
pub fn choice_nocase(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as syn::LitStr);

    let s = input.value();

    // leave quickly if literal is empty
    if s.is_empty() {
        let mut output = proc_macro2::TokenStream::new();
        output.extend_one(TokenTree::Literal(Literal::string("")));
        return output.into();
    }

    // Generate all the case combinations
    let mut previous_generation = Vec::<String>::new();

    for c in s.chars() {
        let variants = [c.to_ascii_uppercase(), c.to_ascii_lowercase()];

        let next_generation = if previous_generation.is_empty() {
            variants.into_iter()
                .map(|c| c.to_string())
                .collect()
        } else {
            let mut next_generation = Vec::with_capacity(previous_generation.len()*2);
            for previous in previous_generation.into_iter() {
                next_generation.push(format!("{}{}", previous, variants[0]));
                next_generation.push(format!("{}{}", previous, variants[1]));
            }
            next_generation
        };

        previous_generation = next_generation;
    }

    if previous_generation.len() >2 {
        let len = previous_generation.len();
        previous_generation.swap(1, len-1);
    }

    // generate the token stream
    let mut output = proc_macro2::TokenStream::new();
    for  (i, token) in  previous_generation.into_iter()
        .map(|s| Literal::string(&s))
        .map(|lit| proc_macro2::TokenTree::Literal(lit))
        .enumerate() {
            if i!=0 {
                output.extend_one(TokenTree::Punct(Punct::new('|', Spacing::Alone)));
            }
            output.extend_one(token);
    }
    output.into()
}