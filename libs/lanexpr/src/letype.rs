#[derive(Debug)]
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
        match *ty {
            LeType::TyRef(_) => panic!("Create create a ref of ref"),
            _ => {}
        }
        Box::new(LeType::TyRef(ty))
    }

    pub fn is_int(&self) -> bool {
        match self {
            LeType::TyInt => true,
            _ => false,
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            LeType::TyVoid => true,
            _ => false,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            LeType::TyRef(_) => true,
            _ => false,
        }
    }

    pub fn rm_ref(ty: &LeTypePtr) -> &LeTypePtr {
        match **ty {
            LeType::TyRef(ref ty_in) => &ty_in,
            _ => ty,
        }
    }

    pub fn can_do_cast(ty_src: &LeTypePtr, ty_dst: &LeTypePtr) -> bool {
        if ty_dst.is_ref() && !ty_src.is_ref() {
            return false;
        }

        let ty_src = LeType::rm_ref(ty_src);
        let ty_dst = LeType::rm_ref(ty_src);

        match **ty_dst {
            LeType::TyInt => ty_src.is_int(),
            LeType::TyVoid => ty_src.is_void(),
            LeType::TyRef(_) => unreachable!(),
        }
    }

    pub fn can_be_cast_to(&self, ty: &LeTypePtr) -> bool {
        true
        /*
        if ty.is_ref() {
            return self.is_ref() && self.can_be_cast_to(LeType::rm_ref(ty));
        }

        let self_ty = LeType::rm_ref(self);


        if self.get_ref().unwrap() {

        }

        match self {

            LeType::TyInt => {
            match
            ,

        }
        */
    }
}

impl Clone for LeType {
    fn clone(&self) -> Self {
        match self {
            LeType::TyInt => LeType::TyInt,
            LeType::TyVoid => LeType::TyVoid,
            LeType::TyRef(ty) => LeType::TyRef(Box::new(*ty.clone())),
        }
    }
}

pub type LeTypePtr = Box<LeType>;

pub struct FnType {
    ret: LeTypePtr,
    args: Vec<LeTypePtr>,
}
