use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Ident, Item, ItemFn, ItemMod};

#[proc_macro_attribute]
pub fn fixture(attr: TokenStream, input: TokenStream) -> TokenStream {
  let attr: Ident = syn::parse(attr).unwrap();
  let mut ast: ItemMod = syn::parse(input).unwrap();

  for item in &mut ast.content.as_mut().unwrap().1 {
    if let Item::Fn(f) = item {
      let fn_name = Ident::new(&format!("{}_{}", attr, f.sig.ident), Span::call_site());
      let ident = f.sig.ident.clone();

      let wrapped_fn: TokenStream2 = quote! {
        #[test]
        fn #fn_name() {
          #f
          let mut fixture = #attr::set_up();
          #ident(&mut fixture);
          fixture.tear_down();
        }
      };

      let wrapped_fn: TokenStream = wrapped_fn.into();

      let wrapped_fn: ItemFn = syn::parse(wrapped_fn).expect("could not parse wrapped function");

      *f = wrapped_fn;
    }
  }

  let module = quote! {
    #ast
  };

  module.into()
}
