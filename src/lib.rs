use std::fs;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, AttributeArgs, NestedMeta, ItemFn, Lit, Ident };

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(day))] => format!("inputs/{}.in", day.token()),
        _ => panic!("Expected one integer argument"),
    };

    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let input = fs::read_to_string(input_path).expect("The input file {input_path} could not be found");

    let tokens = quote! {
        #aoc_solution
        fn main() {
            let now = std::time::Instant::now();
            let (p1, p2) = aoc_solution(#input);
            let elapsed = now.elapsed();
            println!("Part one: {p1}");
            println!("Part two: {p2}");
            if elapsed.as_millis() > 0 {
                println!("Time: {}ms", elapsed.as_millis());
            } else {
                println!("Time: {}μs", elapsed.as_micros());
            }
        }
    };

    TokenStream::from(tokens)
}
