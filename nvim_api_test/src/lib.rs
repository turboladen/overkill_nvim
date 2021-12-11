use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn nvim_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;
    let block = input.block;

    let test_code = quote::quote! {
        #[allow(box_pointers)]
        #[no_mangle]
        pub extern "C" fn #fn_name() -> bool {
            std::panic::set_hook(Box::new(|panic_info| {
                if let Some(location) = panic_info.location() {
                    eprintln!(
                        "panic occurred in file '{}' at line {}",
                        location.file(),
                        location.line()
                    );
                } else {
                    eprintln!("panic occurred but can't get location information...");
                }

                if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                    eprintln!("FAIL: {}", s);
                    return;
                }

                if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                    eprintln!("FAIL: {}", s);
                }
            }));

            let result = std::panic::catch_unwind(|| #block);

            result.is_ok()
        }
    };

    test_code.into()
}
