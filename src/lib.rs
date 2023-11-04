#![feature(extend_one)]

use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenTree, Punct, Spacing};
use syn::{parse_macro_input, parse::{Parse, ParseStream}, Error};


enum Item {
    LitStr(syn::LitStr),
    LitByteStr(syn::LitByteStr)
}


impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            input.parse().map(Item::LitStr)
        } else if lookahead.peek(syn::LitByteStr) {
            input.parse().map(Item::LitByteStr)
        } else {
            Err(lookahead.error())
        }
    }
}


impl Item {
    pub fn is_empty(&self) -> bool {
        match self {
            Item::LitStr(s) => s.value().is_empty(),
            Item::LitByteStr(b) => b.value().is_empty(),
        }
    }
    
    pub fn empty_literal(&self)  -> TokenTree {
        match self {
            Item::LitStr(_) => TokenTree::Literal(Literal::string("")),
            Item::LitByteStr(_) => TokenTree::Literal(Literal::byte_string(b"")),
        }
    }

    /// BUG if &[u8] is not valid
    pub fn to_string(&self) -> String {
        match self {
            Item::LitStr(s) => s.value(),
            Item::LitByteStr(s) => std::str::from_utf8(&s.value()).expect("The string is not valid").to_owned(),
        }
    }

    pub fn is_lit_str(&self) -> bool {
        match self {
            Item::LitStr(_) => true,
            Item::LitByteStr(_) => false,
        }
    }
/*
    pub fn is_lit_byte_str(&self) -> bool {
        !self.is_lit_str()
    }
    */

}

///
/// Transform a string as a list of strings separated by | where all combinations of character cases are generated.
/// By construction first one is fully uppercase, second one is fully lowercase.
/// It also works with byte strings.
///
/// `choice_nocase!("ab")` is replaced by `"AB" | "ab" | "aB" | "Ab" `
/// `choice_nocase!(b"ab")` is replaced by `b"AB" | b"ab" | b"aB" | b"Ab" `
#[proc_macro]
pub fn choice_nocase(stream: TokenStream) -> TokenStream {

    let input = parse_macro_input!(stream as Item);


    // leave quickly if literal is empty
    if input.is_empty() {
        let mut output = proc_macro2::TokenStream::new();
        output.extend_one(input.empty_literal());
        return output.into();
    }

    // Generate all the case combinations
    let mut previous_generation = Vec::<String>::new();

    for c in input.to_string().chars() {
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
        .map(|s| if input.is_lit_str(){
             Literal::string(&s)
        } else {
            Literal::byte_string(s.as_bytes())
        })
        .map(|lit| proc_macro2::TokenTree::Literal(lit))
        .enumerate() {
            if i!=0 {
                output.extend_one(TokenTree::Punct(Punct::new('|', Spacing::Alone)));
            }
            output.extend_one(token);
    }
    output.into()
}

