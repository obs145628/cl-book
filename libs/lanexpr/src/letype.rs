#[derive(Debug, Copy, Clone)]
pub enum TypeVal {
    Int,
}

impl TypeVal {
    pub fn is_int(&self) -> bool {
        match self {
            TypeVal::Int => true,
            _ => false,
        }
    }

    pub fn can_be_cast_to(&self, dst: &TypeVal) -> bool {
        true
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Val(TypeVal),
    Void,
    Ref(TypeVal),
}

impl Type {
    pub fn is_val(&self) -> bool {
        match self {
            Type::Val(_) => true,
            _ => false,
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Type::Void => true,
            _ => false,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            Type::Ref(_) => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Type::Val(x) if x.is_int() => true,
            _ => false,
        }
    }

    pub fn unref(&self) -> Option<TypeVal> {
        match self {
            Type::Val(x) => Some(*x),
            Type::Ref(x) => Some(*x),
            _ => None,
        }
    }

    pub fn can_be_cast_to(&self, dst: &Type) -> bool {
        if dst.is_void() {
            return self.is_void();
        }
        if dst.is_ref() && !self.is_ref() {
            return false;
        }

        let src_val = self.unref().unwrap();
        let dst_val = dst.unref().unwrap();
        src_val.can_be_cast_to(&dst_val)
    }
}

#[derive(Debug, Clone)]
pub struct FnType {
    args: Vec<Type>,
    ret: Type,
}

impl FnType {
    pub fn new(args: Vec<Type>, ret: Type) -> FnType {
        FnType { args, ret }
    }

    pub fn args(&self) -> &Vec<Type> {
        &self.args
    }

    pub fn ret(&self) -> &Type {
        &self.ret
    }
}
