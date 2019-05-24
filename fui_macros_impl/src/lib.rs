extern crate proc_macro;

use proc_macro_hack::proc_macro_hack;
use proc_macro::TokenStream;

#[proc_macro_hack]
pub fn ui(input: TokenStream) -> TokenStream {
    println!("{:?}", input);
    input
}
