use neovim_sys::typval::{self, TypvalT};
use std::ptr::NonNull;

#[no_mangle]
pub unsafe extern "C" fn complete_info() -> *const typval::TypvalT {
    let argvars = Typval::List(TypvalList::default());
    let argvars_inner = argvars.inner();

    let mut retval = Typval::Dict(TypvalDict::default());
    let retval_inner = retval.inner_mut();

    neovim_sys::f_complete_info(
        argvars_inner as *const typval::TypvalT,
        retval_inner as *mut typval::TypvalT,
        neovim_sys::no_op_fn_ptr,
    );

    retval_inner as *const TypvalT
}

pub enum Typval {
    Unknown,
    Number,
    String,
    Func,
    List(TypvalList),
    Dict(TypvalDict),
    Float,
    Bool,
    Special,
    Partial,
    Blob,
}

impl Typval {
    pub fn inner(&self) -> &typval::TypvalT {
        match self {
            Self::Unknown
            | Self::Number
            | Self::String
            | Self::Func
            | Self::Float
            | Self::Bool
            | Self::Special
            | Self::Partial
            | Self::Blob => {
                todo!()
            }
            Self::List(list) => list.inner(),
            Self::Dict(dict) => dict.inner(),
        }
    }

    pub fn inner_mut(&mut self) -> &mut typval::TypvalT {
        match self {
            Self::Unknown
            | Self::Number
            | Self::String
            | Self::Func
            | Self::Float
            | Self::Bool
            | Self::Special
            | Self::Partial
            | Self::Blob => {
                todo!()
            }
            Self::List(list) => list.inner_mut(),
            Self::Dict(dict) => dict.inner_mut(),
        }
    }
}

//-----------------------------------------------------------------------------
// Dict
//-----------------------------------------------------------------------------
pub struct TypvalDict {
    inner: TypvalT,
}

impl TypvalDict {
    pub fn new(mut dict: Dict) -> Self {
        let tv = typval::TypvalT {
            v_type: typval::VarType::Dict,
            v_lock: typval::VarLockStatus::Unlocked,
            vval: typval::TypvalVvalUnion {
                v_dict: dict.as_mut_ptr(),
            },
        };
        unsafe { typval::tv_init(&tv) };

        TypvalDict { inner: tv }
    }

    pub fn inner(&self) -> &TypvalT {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut TypvalT {
        &mut self.inner
    }
}

impl Default for TypvalDict {
    fn default() -> Self {
        let mut dict = Dict::default();

        let tv = typval::TypvalT {
            v_type: typval::VarType::Dict,
            v_lock: typval::VarLockStatus::Unlocked,
            vval: typval::TypvalVvalUnion {
                v_dict: dict.as_mut_ptr(),
            },
        };
        unsafe { typval::tv_init(&tv) };

        TypvalDict { inner: tv }
    }
}

pub struct Dict {
    inner: NonNull<typval::DictT>,
}

impl Dict {
    pub fn as_ptr(&mut self) -> *const typval::DictT {
        self.inner.as_ptr() as *const typval::DictT
    }

    pub fn as_mut_ptr(&mut self) -> *mut typval::DictT {
        self.inner.as_ptr()
    }
}

impl Default for Dict {
    fn default() -> Self {
        Self {
            inner: unsafe { NonNull::new(typval::tv_dict_alloc()).unwrap() },
        }
    }
}

//-----------------------------------------------------------------------------
// List
//-----------------------------------------------------------------------------
pub struct TypvalList {
    inner: TypvalT,
}

impl TypvalList {
    pub fn new(mut list: List) -> Self {
        let tv = typval::TypvalT {
            v_type: typval::VarType::List,
            v_lock: typval::VarLockStatus::Unlocked,
            vval: typval::TypvalVvalUnion {
                v_list: list.as_mut_ptr(),
            },
        };
        unsafe { typval::tv_init(&tv) };

        TypvalList { inner: tv }
    }

    pub fn inner(&self) -> &TypvalT {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut TypvalT {
        &mut self.inner
    }
}

impl Default for TypvalList {
    fn default() -> Self {
        let mut list = List::default();

        let tv = typval::TypvalT {
            v_type: typval::VarType::List,
            v_lock: typval::VarLockStatus::Unlocked,
            vval: typval::TypvalVvalUnion {
                v_list: list.as_mut_ptr(),
            },
        };
        unsafe { typval::tv_init(&tv) };

        TypvalList { inner: tv }
    }
}

pub struct List {
    inner: NonNull<typval::ListT>,
}

impl List {
    pub fn new(len: usize) -> Self {
        Self {
            inner: unsafe { NonNull::new(typval::tv_list_alloc(len)).unwrap() },
        }
    }

    pub fn as_ptr(&mut self) -> *const typval::ListT {
        self.inner.as_ptr() as *const typval::ListT
    }

    pub fn as_mut_ptr(&mut self) -> *mut typval::ListT {
        self.inner.as_ptr()
    }
}

impl Default for List {
    fn default() -> Self {
        Self {
            inner: unsafe { NonNull::new(typval::tv_list_alloc(0)).unwrap() },
        }
    }
}
