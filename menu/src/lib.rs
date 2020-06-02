// work in progress
// the goal is to make something like this:

// use std::io::stdin;
// use crate::play;

// #[derive(Menu)]
// enum MainMenu {
//     #[field(display = "Play", perform = play::run)]
//     Play,
//     ...
// }

// fn main() -> GameResult<()> {
//     let menu = MainMenu::build();
//     println!("{}", menu);
//     let mut choice = String::new();
//     stdin().read_line(&mut choice);
//     menu.perform(choice)?;
//     Ok(())
// }

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Menu, attributes(field))]
pub fn build_menu(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let name = &input.ident;

    println!("{:#?}", input);

    let m_name = format!("{}Menu", name);
    let m_ident = syn::Ident::new(&m_name, name.span());

    let expanded = quote! {
        struct #m_ident {

        }

        impl #name {
            fn build() -> #m_ident {
                #m_ident {

                }
            }
        }
    };

    expanded.into()
}
