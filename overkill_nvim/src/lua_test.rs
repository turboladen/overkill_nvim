#![allow(missing_docs, clippy::missing_panics_doc)]

use crate::{
    key_code::KeyCode,
    option::{
        flags::{AddAssignFlags, SubAssignFlags},
        CharFlags, CompleteOpt, CompleteOptSettings, NullableStringOption, PasteToggle, ShortMess,
        ShortMessItem, StringOption,
    },
};
use nvim_api_test::nvim_test;

#[nvim_test]
fn test_overkill_set_global_string_option() {
    let new_value = CompleteOptSettings::default().no_select().no_insert();

    CompleteOpt::set_global(new_value).unwrap();

    let value = CompleteOpt::get_global().unwrap();
    assert_eq!(value, new_value);
}

#[nvim_test]
fn test_overkill_set_global_nullable_string_option() {
    // The default is "", so test we get a None.
    assert!(PasteToggle::get_global().unwrap().is_none());

    let new_value = KeyCode::F5;

    PasteToggle::set_global(Some(new_value)).unwrap();
    let value = PasteToggle::get_global().unwrap().unwrap();

    assert_eq!(value, new_value);
}

#[nvim_test]
fn test_overkill_set_global_nullable_string_char_flags_option() {
    // First test setting to "".
    let value = ShortMess::get_global().unwrap();
    assert!(value.is_some(), "expected Some(_), got None");

    let new_value = CharFlags::new(vec![ShortMessItem::AbbreviateFile]);

    ShortMess::set_global(Some(new_value.clone())).unwrap();
    let value = ShortMess::get_global().unwrap().unwrap();

    assert_eq!(new_value, value);
}

#[nvim_test]
fn test_overkill_set_add_assign_global_nullable_string_char_flags_option() {
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

#[nvim_test]
fn test_overkill_set_sub_assign_global_nullable_string_char_flags_option() {
    let to_remove = ShortMessItem::AbbreviateFile;

    let before = ShortMess::get_global().unwrap().unwrap();
    let before_len = before.len();

    ShortMess::sub_assign_global(&to_remove).unwrap();

    let value = ShortMess::get_global().unwrap().unwrap();
    assert_eq!(value.len(), before_len - 1, "value: {:?}", value);

    let mut expected = before;
    expected.remove(&to_remove);

    assert_eq!(expected, value);
}
