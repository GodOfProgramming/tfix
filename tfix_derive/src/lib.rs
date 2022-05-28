use proc_macro::TokenStream;
use proc_macro2::{ TokenStream as TokenStream2, Span };
use quote::quote;
use syn::{Ident, Item, ItemFn, ItemMod, Token};

#[proc_macro_attribute]
pub fn fixture(attr: TokenStream, input: TokenStream) -> TokenStream {
  println!("before = {}", input);

  let fixture = syn::parse(attr).unwrap();
  let ast = syn::parse(input).unwrap();

  let gen = configure_tests(fixture, ast);

  println!("after = {}", gen);

  gen
}

fn configure_tests(fixture: Ident, mut ast: ItemMod) -> TokenStream {
  for item in &mut ast.content.as_mut().unwrap().1 {
    if let Item::Fn(f) = item {
      let fn_name = Ident::new(&format!("{}_{}", ast.ident, f.sig.ident), Span::call_site());
      let ident = f.sig.ident.clone();

      let wrapped_fn: TokenStream2 = quote! {
        #[test]
        fn #fn_name() {
          #f
          let mut fixture = #fixture::set_up();
          #ident(&mut fixture);
          fixture.tear_down();
        }
      }
      .into();

      let wrapped_fn: TokenStream = wrapped_fn.into();

      println!("trying to parse\n{}", wrapped_fn);

      let wrapped_fn: ItemFn = syn::parse(wrapped_fn).expect("could not parse wrapped function");

      *f = wrapped_fn;
    }
  }

  let module = quote! {
    #ast
  };

  module.into()
}
