#![deny(unused_extern_crates)]
#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications
)]

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
                match (panic_info.payload().downcast_ref::<String>(), panic_info.location()) {
                    (Some(payload), Some(location)) => {
                        eprintln!("FAIL! [{}:{}]\n{}", location.file(), location.line(), payload)
                    }
                    (Some(payload), None) => {
                        eprintln!("FAIL! [unknown location]\n{}", payload)
                    }
                    (None, Some(location)) => {
                        eprintln!("FAIL! [{}:{}]", location.file(), location.line())
                    }
                    (None, None) => {
                        eprintln!("FAIL! [unknown location")
                    }
                }

            }));

            let result = std::panic::catch_unwind(|| #block);

            result.is_ok()
        }
    };

    test_code.into()
}
