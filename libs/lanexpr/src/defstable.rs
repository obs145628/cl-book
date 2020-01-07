use crate::ast::*;
use crate::letype::{FnType, Type, TypeVal};

use std::collections::HashMap;

/*
struct DefVar {
    id: usize,
    ast_id: usize,
    ty: Type,
}

struct DefFun {
    id: usize,
    ast_id: usize,
    ty: FnType,
    vars: Vec<DefVar>,
}
*/

pub struct DefsTable {
    scopes_tys: Vec<HashMap<String, Type>>,
    scopes_fns: Vec<HashMap<String, FnType>>,
    scopes_vars: Vec<HashMap<String, Type>>,
    fn_scopes_pos: Vec<usize>,

    exp_types: HashMap<usize, Type>,
    typename_types: HashMap<usize, Type>,
    //fns: Vec<DefFun>,
    //act_fn: Option<usize>,
}

impl DefsTable {
    /*
    pub fn add_fun_def(&mut self, node: &ASTDefFun, ty: FnType) {
        let fun = DefFun {
            id: self.fns.len(),
            ast_id: node.get_uid(),
            ty,
            vars: vec![],
        };
        self.fns.push(fun);
    }

    pub fn add_var_def(&mut self, ast_id: usize, ty: Type) {
        let f = &mut self.fns[self.act_fn.unwrap()];
        f.vars.push(DefVar {
            id: f.vars.len(),
            ast_id,
            ty,
        });
    }

    pub fn change_actual_fn(&mut self, node: &ASTDefFun) {
        let fn_id = node.get_uid();
        let act_fn = self.fns.iter().position(|f| f.ast_id == fn_id).unwrap();
        self.act_fn = Some(act_fn);
    }
    */

    pub fn new() -> DefsTable {
        let mut res = DefsTable {
            scopes_tys: vec![],
            scopes_fns: vec![],
            scopes_vars: vec![],
            fn_scopes_pos: vec![],

            exp_types: HashMap::new(),
            typename_types: HashMap::new(),
            //fns: vec![],
            //act_fn: None,
        };
        res.open_scope();
        res.set_native_defs();
        res
    }

    pub fn total_scopes_len(&self) -> usize {
        self.scopes_tys.len()
    }

    pub fn add_type(&mut self, id: &str, ty: Type) {
        assert!(self.get_top_type(id).is_none());
        self.scopes_tys
            .last_mut()
            .unwrap()
            .insert(id.to_string(), ty);
    }

    pub fn add_fun(&mut self, id: &str, ty: FnType) {
        assert!(self.get_top_fun(id).is_none());
        self.scopes_fns
            .last_mut()
            .unwrap()
            .insert(id.to_string(), ty);
    }

    pub fn add_var(&mut self, id: &str, ty: Type) {
        assert!(self.get_top_var(id).is_none());
        self.scopes_vars
            .last_mut()
            .unwrap()
            .insert(id.to_string(), ty);
    }

    pub fn get_top_type(&self, id: &str) -> Option<Type> {
        self.scopes_tys.last().unwrap().get(id).copied()
    }

    pub fn get_top_fun(&self, id: &str) -> Option<&FnType> {
        self.scopes_fns.last().unwrap().get(id)
    }

    pub fn get_top_var(&self, id: &str) -> Option<Type> {
        self.scopes_vars.last().unwrap().get(id).copied()
    }

    pub fn get_type(&self, id: &str) -> Option<Type> {
        for scope in self.scopes_tys.iter().rev() {
            let item = scope.get(id);
            if item.is_some() {
                return item.copied();
            }
        }

        None
    }

    pub fn get_fun(&self, id: &str) -> Option<&FnType> {
        for scope in self.scopes_fns.iter().rev() {
            let item = scope.get(id);
            if item.is_some() {
                return item;
            }
        }

        None
    }

    pub fn get_var(&self, id: &str) -> Option<Type> {
        for scope in self.scopes_vars.iter().rev() {
            let item = scope.get(id);
            if item.is_some() {
                return item.copied();
            }
        }

        None
    }

    pub fn open_scope(&mut self) {
        self.scopes_tys.push(HashMap::new());
        self.scopes_fns.push(HashMap::new());
        self.scopes_vars.push(HashMap::new());
    }

    pub fn open_scope_fn(&mut self) {
        let beg_fn = self.scopes_tys.len();
        self.open_scope();
        self.fn_scopes_pos.push(beg_fn);
    }

    pub fn close_scope(&mut self) {
        if self.scopes_tys.len() <= 1 {
            panic!("Trying to close native defs scope");
        }

        match self.fn_scopes_pos.last() {
            Some(beg_fn) if *beg_fn >= self.scopes_tys.len() - 1 => {
                panic!("Trying to close scope for function, but no function there")
            }
            _ => {}
        }

        self.scopes_tys.pop().unwrap();
        self.scopes_fns.pop().unwrap();
        self.scopes_vars.pop().unwrap();
    }

    pub fn close_scope_fn(&mut self) {
        let beg_fn = self.fn_scopes_pos.last().unwrap();
        if *beg_fn != self.scopes_tys.len() - 1 {
            panic!("Trying to close scope for function, but no function there");
        }
        self.fn_scopes_pos.pop().unwrap();

        self.close_scope();
    }

    fn set_native_defs(&mut self) {
        self.add_type("int", Type::Val(TypeVal::Int));
        self.add_type("void", Type::Void);
    }

    pub fn get_exp_type(&self, node: &dyn ASTExpr) -> Option<Type> {
        let uid = node.get_uid();
        self.exp_types.get(&uid).copied()
    }

    pub fn set_exp_type(&mut self, node: &dyn ASTExpr, ty: Type) {
        let uid = node.get_uid();
        assert!(self.exp_types.get(&uid).is_none());
        self.exp_types.insert(uid, ty);
    }

    pub fn get_typename_type(&self, node: &dyn ASTType) -> Option<Type> {
        let uid = node.get_uid();
        self.typename_types.get(&uid).copied()
    }

    pub fn set_typename_type(&mut self, node: &dyn ASTType, ty: Type) {
        let uid = node.get_uid();
        assert!(self.typename_types.get(&uid).is_none());
        self.typename_types.insert(uid, ty);
    }
}
