#![allow(missing_docs, clippy::missing_panics_doc)]

use crate::option::{CompleteOpt, CompleteOptSettings, StringOption};

macro_rules! print_error_return_false {
    ($e:expr) => {{
        eprintln!("Got error during test: {}", $e);
        return false;
    }};
}

#[no_mangle]
pub extern "C" fn test_overkill_set_global_option() -> bool {
        let new_value = CompleteOptSettings::default().no_select().no_insert();

        match CompleteOpt::set_global(new_value) {
            Ok(_) => match CompleteOpt::get_global() {
                Ok(value) => {
                    if value != new_value {
                        eprintln!(
                            "FAIL! Expected `\"{:?}\"`, got: `\"{:?}\"`",
                            new_value, value
                        );
                        return false;
                    }
                }
                Err(e) => print_error_return_false!(e),
            },
            Err(e) => print_error_return_false!(e),
        }

    true
}
