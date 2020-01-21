use crate::ast::*;
use crate::bindapp::BindApp;
use crate::bindfun::{BindFun, BindFunId};
use crate::bindscope::BindScope;
use crate::bindvar::BindVar;
use crate::letype::{FnType, Type, TypeVal};
use crate::nativedefs;

use std::collections::HashMap;

pub struct BindBuilder {
    app: BindApp,
    scope: BindScope,
    type_names: HashMap<&'static str, Type>,
    funs: Vec<BindFunId>,
}

impl BindBuilder {
    pub fn new() -> BindBuilder {
        BindBuilder {
            app: BindApp::new(),
            scope: BindScope::new(),
            type_names: HashMap::new(),
            funs: vec![],
        }
    }

    pub fn begin(&mut self) {
        self.init_native_types();
        self.init_native_funs();
        self.init_main();
    }

    pub fn end(&mut self) {
        self.close_fun(); //close main function
        self.close_fun(); //close native defs function

        assert!(self.scope.nb_open_funs() == 0);
        assert!(self.funs.len() == 0);
    }

    pub fn get_binding(self) -> BindApp {
        self.app
    }

    pub fn open_scope(&mut self) {
        self.scope.open_scope();
    }

    pub fn close_scope(&mut self) {
        self.scope.close_scope();
    }

    pub fn open_fun_ast(&mut self, node: &ASTDefFun) {
        let fun_id = self.app.get_fun_from_ast(node.get_uid()).id();
        self.open_fun(fun_id);
    }

    pub fn close_fun_ast(&mut self) {
        self.close_fun();
    }

    pub fn actual_fn(&self) -> &BindFun {
        let fun_id = self.funs.last().unwrap();
        self.app.get_fun(*fun_id)
    }

    pub fn actual_fn_mut(&mut self) -> &mut BindFun {
        let fun_id = self.funs.last().unwrap();
        self.app.get_fun_mut(*fun_id)
    }

    pub fn add_fun_type(&mut self, node: &ASTDefFun, ty: FnType) {
        let fun_id = self.app.add_fun_ast(node.name(), ty, node.get_uid());
        self.scope.add_fun(node.name(), fun_id);
    }

    pub fn add_arg_type(&mut self, node: &ASTDefArg, ty: Type) {
        let fun = self.actual_fn_mut();
        let var_id = fun.add_var(node.name(), ty, node.get_uid());
        self.scope.add_var(node.name(), var_id);
    }

    pub fn add_var_type(&mut self, node: &ASTDefVar, ty: Type) {
        let fun = self.actual_fn_mut();
        let var_id = fun.add_var(node.name(), ty, node.get_uid());
        self.scope.add_var(node.name(), var_id);
    }

    pub fn get_fun(&mut self, name: &str) -> Option<&BindFun> {
        match self.scope.find_fun(name) {
            Some(fn_id) => Some(self.app.get_fun(fn_id)),
            None => None,
        }
    }

    pub fn get_var(&mut self, name: &str) -> Option<&BindVar> {
        let fun = self.actual_fn();
        match self.scope.find_var(name) {
            Some(var_id) => Some(fun.get_var(var_id)),
            None => None,
        }
    }

    pub fn get_type(&self, name: &str) -> Option<Type> {
        self.type_names.get(name).copied()
    }

    fn open_fun(&mut self, fun_id: BindFunId) {
        self.funs.push(fun_id);
        self.scope.open_fun();
    }

    fn close_fun(&mut self) {
        self.funs.pop().unwrap();
        self.scope.close_fun();
    }

    fn add_native_fun(&mut self, def: &nativedefs::NativeFun) -> BindFunId {
        let id = self.add_native_fun_hidden(def);
        self.scope.add_fun(def.name(), id);
        id
    }

    fn add_native_fun_hidden(&mut self, def: &nativedefs::NativeFun) -> BindFunId {
        self.app.add_fun_native(def.name(), def.ty().clone())
    }

    fn add_native_type(&mut self, def: &nativedefs::NativeType) {
        self.type_names.insert(def.name(), def.ty());
    }

    fn init_native_types(&mut self) {
        for ty in nativedefs::TYPES_LIST.iter() {
            self.add_native_type(ty);
        }
    }

    fn init_native_funs(&mut self) {
        let nat_defs_id = self.add_native_fun_hidden(&nativedefs::SPE_NATIVE_DEFS);
        self.open_fun(nat_defs_id);

        for op in nativedefs::OPS_LIST.iter() {
            self.add_native_fun(op);
        }
        for fun in nativedefs::STD_FUNS_LIST.iter() {
            self.add_native_fun(fun);
        }
    }

    fn init_main(&mut self) {
        let main_id = self.add_native_fun(&nativedefs::SPE_MAIN);
        self.open_fun(main_id);
    }
}
