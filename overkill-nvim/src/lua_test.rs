#![allow(missing_docs, clippy::missing_panics_doc)]

use crate::{
    key_code::KeyCode,
    option::{
        flags::{AddAssignFlags, SubAssignFlags},
        CharFlags, CompleteOpt, CompleteOptSettings, NullableStringOption, PasteToggle, ShortMess,
        ShortMessItem, StringOption,
    },
};

fn panic_hook(panic_info: &std::panic::PanicInfo<'_>) {
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
        return;
    }
}

macro_rules! test {
    ($name:ident, $body:tt) => {
        #[allow(box_pointers)]
        #[no_mangle]
        pub extern "C" fn $name() -> bool {
            std::panic::set_hook(Box::new(panic_hook));

            let result = std::panic::catch_unwind(|| {
                $body
            });

            result.is_ok()
        }
    };
}

test!(test_overkill_set_global_string_option, {
    let new_value = CompleteOptSettings::default().no_select().no_insert();

    CompleteOpt::set_global(new_value).unwrap();

    let value = CompleteOpt::get_global().unwrap();
    assert_eq!(value, new_value);
});

test!(test_overkill_set_global_nullable_string_option, {
    // The default is "", so test we get a None.
    assert!(PasteToggle::get_global().unwrap().is_none());

    let new_value = KeyCode::F5;

    PasteToggle::set_global(Some(new_value)).unwrap();
    let value = PasteToggle::get_global().unwrap().unwrap();

    assert_eq!(value, new_value);
});

test!(
    test_overkill_set_global_nullable_string_char_flags_option,
    {
        // First test setting to "".
        let value = ShortMess::get_global().unwrap();
        assert!(value.is_some(), "expected Some(_), got None");

        let new_value = CharFlags::new(vec![ShortMessItem::AbbreviateFile]);

        ShortMess::set_global(Some(new_value.clone())).unwrap();
        let value = ShortMess::get_global().unwrap().unwrap();

        assert_eq!(new_value, value);
    }
);

test!(
    test_overkill_set_add_assign_global_nullable_string_char_flags_option,
    {
        let new_value = ShortMessItem::AbbreviateModified;

        let expected = ShortMess::get_global()
            .unwrap()
            .map(|mut current| {
                current.push(new_value);
                current
            })
            .unwrap();

        ShortMess::add_assign_global(new_value).unwrap();

        let value = ShortMess::get_global().unwrap().unwrap();

        assert_eq!(expected, value);
    }
);

test!(
    test_overkill_set_sub_assign_global_nullable_string_char_flags_option,
    {
        let to_remove = ShortMessItem::AbbreviateFile;

        let before = ShortMess::get_global().unwrap().unwrap();
        let before_len = before.len();

        ShortMess::sub_assign_global(&to_remove).unwrap();

        let value = ShortMess::get_global().unwrap().unwrap();
        assert_eq!(value.len(), before_len - 1, "value: {:?}", value);

        let mut expected = before.clone();
        expected.remove(&to_remove);

        assert_eq!(expected, value);
    }
);
