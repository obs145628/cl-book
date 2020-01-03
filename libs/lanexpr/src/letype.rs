pub enum LeType {
    TyInt,
    TyVoid,
    TyRef(LeTypePtr),
}

impl LeType {
    pub fn new_int() -> LeTypePtr {
        Box::new(LeType::TyInt)
    }

    pub fn new_void() -> LeTypePtr {
        Box::new(LeType::TyVoid)
    }

    pub fn new_ref(ty: LeTypePtr) -> LeTypePtr {
        Box::new(LeType::TyRef(ty))
    }
}

pub type LeTypePtr = Box<LeType>;

pub struct FnType {
    ret: LeTypePtr,
    args: Vec<LeTypePtr>,
}
