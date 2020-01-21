use crate::letype::{FnType, Type, TypeVal};

pub struct NativeFun {
    name: &'static str,
    ty: FnType,
}

impl NativeFun {
    pub fn new(name: &'static str, ty: FnType) -> Self {
        NativeFun { name, ty }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn ty(&self) -> &FnType {
        &self.ty
    }
}

lazy_static! {
    pub static ref OP_SET: NativeFun = NativeFun::new(
        "@op:set",
        FnType::new(
            vec![Type::Ref(TypeVal::Int), Type::Val(TypeVal::Int)],
            Type::Void,
        )
    );
    pub static ref OP_EQ: NativeFun = NativeFun::new(
        "@op:eq",
        FnType::new(
            vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
            Type::Val(TypeVal::Int)
        )
    );
    pub static ref OP_LT: NativeFun = NativeFun::new(
        "@op:lt",
        FnType::new(
            vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
            Type::Val(TypeVal::Int)
        )
    );
    pub static ref OP_GT: NativeFun = NativeFun::new(
        "@op:gt",
        FnType::new(
            vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
            Type::Val(TypeVal::Int)
        )
    );
    pub static ref OP_ADD: NativeFun = NativeFun::new(
        "@op:add",
        FnType::new(
            vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
            Type::Val(TypeVal::Int)
        )
    );
    pub static ref OP_SUB: NativeFun = NativeFun::new(
        "@op:sub",
        FnType::new(
            vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
            Type::Val(TypeVal::Int)
        )
    );
    pub static ref OP_MUL: NativeFun = NativeFun::new(
        "@op:mul",
        FnType::new(
            vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
            Type::Val(TypeVal::Int)
        )
    );
    pub static ref OP_DIV: NativeFun = NativeFun::new(
        "@op:div",
        FnType::new(
            vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
            Type::Val(TypeVal::Int)
        )
    );
    pub static ref OP_MOD: NativeFun = NativeFun::new(
        "@op:mod",
        FnType::new(
            vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
            Type::Val(TypeVal::Int)
        )
    );
    pub static ref OP_NEG: NativeFun = NativeFun::new(
        "@op:neg",
        FnType::new(vec![Type::Val(TypeVal::Int)], Type::Val(TypeVal::Int))
    );
    pub static ref OP_NOT: NativeFun = NativeFun::new(
        "@op:not",
        FnType::new(vec![Type::Val(TypeVal::Int)], Type::Val(TypeVal::Int))
    );
    pub static ref STD_FUN_PUTC: NativeFun = NativeFun::new(
        "putc",
        FnType::new(vec![Type::Val(TypeVal::Int)], Type::Void)
    );
    pub static ref SPE_MAIN: NativeFun =
        NativeFun::new("@spe:__main", FnType::new(vec![], Type::Void));
    pub static ref SPE_NATIVE_DEFS: NativeFun =
        NativeFun::new("@spe:__native_defs", FnType::new(vec![], Type::Void));
    pub static ref OPS_LIST: Vec<&'static NativeFun> = vec![
        &OP_SET, &OP_EQ, &OP_LT, &OP_GT, &OP_ADD, &OP_SUB, &OP_MUL, &OP_DIV, &OP_MOD, &OP_NEG,
        &OP_NOT
    ];
    pub static ref STD_FUNS_LIST: Vec<&'static NativeFun> = vec![&STD_FUN_PUTC];
}
