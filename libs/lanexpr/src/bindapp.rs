use crate::ast::ASTUid;
use crate::bindfun::{BindFun, BindFunId, BindFunsList};
use crate::letype::FnType;

use std::collections::HashMap;

pub struct BindApp {
    funs: BindFunsList,

    ast2fun: HashMap<ASTUid, BindFunId>,
    native2fun: HashMap<String, BindFunId>,
}

impl BindApp {
    pub fn new() -> Self {
        BindApp {
            funs: BindFunsList::new(),

            ast2fun: HashMap::new(),
            native2fun: HashMap::new(),
        }
    }

    pub fn dump_bindings(&self) {
        for fun in self.funs.get_funs() {
            fun.dump_bindings();
        }
    }

    pub fn add_fun_ast(&mut self, name: &str, ty: FnType, ast_id: ASTUid) -> BindFunId {
        let fun_id = self.funs.add_fun(name, ty, Some(ast_id));
        self.ast2fun.insert(ast_id, fun_id);
        fun_id
    }

    pub fn add_fun_native(&mut self, name: &str, ty: FnType) -> BindFunId {
        let fun_id = self.funs.add_fun(name, ty, None);
        self.native2fun.insert(String::from(name), fun_id);
        fun_id
    }

    pub fn get_fun(&self, id: BindFunId) -> &BindFun {
        self.funs.get_fun(id)
    }

    pub fn get_fun_mut(&mut self, id: BindFunId) -> &mut BindFun {
        self.funs.get_fun_mut(id)
    }

    pub fn get_fun_from_ast(&self, id: ASTUid) -> &BindFun {
        let fun_id = self.ast2fun.get(&id).unwrap();
        self.get_fun(*fun_id)
    }

    pub fn get_fun_from_native_name(&self, name: &str) -> &BindFun {
        let fun_id = self.native2fun.get(name).unwrap();
        self.get_fun(*fun_id)
    }
}
