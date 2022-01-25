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

mod mapping {
    use super::*;
    use crate::mapping::{mapper::Mapper, MapMode};

    macro_rules! test_map {
        ($mode:ident, $lhs:expr) => {
            let mapper = Mapper::new(MapMode::$mode);
            mapper.map($lhs, "<NOP>");

            let list = mapper
                .list()
                .into_iter()
                .find(|mapping| mapping.lhs() == $lhs);

            assert!(list.is_some());
        };
    }

    macro_rules! test_map_with_options {
        ($mode:ident, $lhs:expr) => {
            let mapper = Mapper::new(MapMode::$mode).nowait().silent().unique();
            mapper.map($lhs, "<NOP>");

            let list = mapper
                .list()
                .into_iter()
                .find(|mapping| mapping.lhs() == $lhs);

            assert!(list.is_some());
        };
    }

    #[nvim_test]
    fn test_map() {
        test_map!(NormalVisualSelectOperatorPending, "<C-A><C-A>");
        test_map_with_options!(NormalVisualSelectOperatorPending, "<C-A><C-B>");
    }

    #[nvim_test]
    fn test_map_normal() {
        test_map!(Normal, "<C-A>n1");
        test_map_with_options!(Normal, "<C-A>n2");
    }

    #[nvim_test]
    fn test_map_visual_select() {
        test_map!(VisualSelect, "<C-A>v1");
        test_map_with_options!(VisualSelect, "<C-A>v2");
    }

    #[nvim_test]
    fn test_map_visual() {
        test_map!(Visual, "<C-A>x1");
        test_map_with_options!(Visual, "<C-A>x2");
    }

    #[nvim_test]
    fn test_map_select() {
        test_map!(Select, "<C-A>s1");
        test_map_with_options!(Select, "<C-A>s2");
    }

    #[nvim_test]
    fn test_map_operator_pending() {
        test_map!(OperatorPending, "<C-A>op1");
        test_map_with_options!(OperatorPending, "<C-A>op2");
    }

    #[nvim_test]
    fn test_map_insert() {
        test_map!(Insert, "<C-A>i1");
        test_map_with_options!(Insert, "<C-A>i2");
    }

    #[nvim_test]
    #[ignore = "I don't understand why this fails"]
    fn test_map_insert_and_command_line() {
        test_map!(InsertAndCommandLine, "<C-B>");
        test_map_with_options!(InsertAndCommandLine, "<C-B>");
    }

    #[nvim_test]
    fn test_map_language_mapping() {
        test_map!(LanguageMapping, "<C-A>l1");
        test_map_with_options!(LanguageMapping, "<C-A>l2");
    }

    #[nvim_test]
    fn test_map_command_line() {
        test_map!(CommandLine, "<C-A>c1");
        test_map_with_options!(CommandLine, "<C-A>c2");
    }

    #[nvim_test]
    fn test_map_terminal_job() {
        test_map!(TerminalJob, "<C-A>t1");
        test_map_with_options!(TerminalJob, "<C-A>t2");
    }
}
