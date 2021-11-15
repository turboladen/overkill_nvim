use crate::types::CharU;

extern "C" {
    pub fn tv_init(tv: *const TypvalT);

    pub fn tv_list_alloc(len: PtrDiffT) -> *mut ListT;
    pub fn tv_item_lock(tv: *const TypvalT, deep: i32, lock: bool, check_refcount: bool);
    pub fn tv_list_len(l: *const ListT) -> isize;

    pub fn tv_list_item_alloc() -> *mut ListitemT;

    pub fn tv_dict_alloc() -> *mut DictT;

}

#[repr(C)]
pub struct DictT {
    _unused: [u8; 0],
}

// https://github.com/neovim/neovim/blob/684299ed4c9c21f0353ceaec2d1679f956617737/src/nvim/eval/typval.c#L209
pub type ListT = ListvarS;

/// Structure to hold info about a list
/// Order of members is optimized to reduce padding.
// https://github.com/neovim/neovim/blob/7d21b958691c06ed6b40aa1909cd81c37a67844e/src/nvim/eval/typval.h#L174
#[repr(C)]
pub struct ListvarS {
    _unused: [u8; 0],
}

type ListitemT = ListitemS;

// https://github.com/neovim/neovim/blob/684299ed4c9c21f0353ceaec2d1679f956617737/src/nvim/eval/typval.c#L123
#[repr(C)]
pub struct ListitemS {
    _unused: [u8; 0],
}
// Not sure if this is right...
type PtrDiffT = usize;

// https://github.com/neovim/neovim/blob/0159e4daaed1347db8719c27946fcfdc4e49e92d/src/nvim/eval/typval.h#L128
#[repr(C)]
pub struct TypvalT {
    pub v_type: VarType,
    pub v_lock: VarLockStatus,
    pub vval: TypvalVvalUnion,
}

#[derive(PartialEq, Eq)]
#[repr(C)]
pub enum VarType {
    Unknown = 0,
    Number,
    String,
    Func,
    List,
    Dict,
    Float,
    Bool,
    Special,
    Partial,
    Blob,
}

#[repr(C)]
pub enum VarLockStatus {
    Unlocked = 0,
    Locked = 1,
    Fixed = 2,
}

#[repr(C)]
pub union TypvalVvalUnion {
    pub v_number: VarnumberT,
    pub v_bool: BoolVarValue,
    pub v_special: SpecialVarValue,
    pub v_float: FloatT,
    pub v_string: *mut CharU,
    pub v_list: *mut ListT,
    pub v_dict: *mut DictT,
    pub v_partial: *mut PartialT,
}

pub type VarnumberT = i64;
pub type FloatT = f64;
pub type PartialT = PartialS;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum BoolVarValue {
    BoolVarFalse = 0, // v:false
    BoolVarTrue,      // v:true
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum SpecialVarValue {
    SpecialVarNull = 0, // v:null
}

#[repr(C)]
pub struct PartialS {
    _unused: [u8; 0],
}
