use crate::ast::*;
use crate::letype::{FnType, Type, TypeVal};

use std::collections::HashMap;

pub struct DefFun {
    pub id: DefFunId,
    pub ast_id: usize,
    pub ty: FnType,
}

#[derive(Debug, Copy, Clone)]
pub struct DefFunId {
    id: usize,
}

const DEF_ID_NATIVE: usize = 0;

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
    scopes_fns: Vec<HashMap<String, DefFunId>>,
    scopes_vars: Vec<HashMap<String, Type>>,
    fn_scopes_pos: Vec<usize>,

    exp_types: HashMap<usize, Type>, //contain type for every ASTExpr
    typename_types: HashMap<usize, Type>, //contain type fro every ASTType

    defs_fns: Vec<DefFun>,
    ast_def_fun_defs: HashMap<usize, DefFunId>, //contain DefFunId for every ASTDefFun

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
            defs_fns: vec![],

            ast_def_fun_defs: HashMap::new(),
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

    pub fn add_fun(&mut self, id: &str, ty: FnType, ast_id: usize) -> DefFunId {
        assert!(self.get_top_fun_id(id).is_none());

        let fn_id = DefFunId {
            id: self.defs_fns.len(),
        };
        self.defs_fns.push(DefFun {
            id: fn_id,
            ast_id,
            ty,
        });

        self.scopes_fns
            .last_mut()
            .unwrap()
            .insert(id.to_string(), fn_id);
        self.ast_def_fun_defs.insert(ast_id, fn_id);

        fn_id
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

    pub fn get_top_fun_id(&self, id: &str) -> Option<DefFunId> {
        self.scopes_fns.last().unwrap().get(id).copied()
    }

    pub fn get_top_fun(&self, id: &str) -> Option<&DefFun> {
        match self.get_top_fun_id(id) {
            Some(fn_id) => self.defs_fns.get(fn_id.id),
            _ => None,
        }
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

    pub fn get_fun_id(&self, id: &str) -> Option<DefFunId> {
        for scope in self.scopes_fns.iter().rev() {
            let item = scope.get(id);
            if item.is_some() {
                return item.copied();
            }
        }

        None
    }

    pub fn get_scope_fun(&self, id: &str) -> Option<&DefFun> {
        match self.get_fun_id(id) {
            Some(fn_id) => self.defs_fns.get(fn_id.id),
            _ => None,
        }
    }

    pub fn get_fun(&self, id: DefFunId) -> &DefFun {
        self.defs_fns.get(id.id).unwrap()
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

        self.add_fun(
            "@op:set",
            FnType::new(
                vec![Type::Ref(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Void,
            ),
            DEF_ID_NATIVE,
        );
        self.add_fun(
            "@op:eq",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            DEF_ID_NATIVE,
        );
        self.add_fun(
            "@op:lt",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            DEF_ID_NATIVE,
        );
        self.add_fun(
            "@op:gt",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            DEF_ID_NATIVE,
        );
        self.add_fun(
            "@op:add",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            DEF_ID_NATIVE,
        );
        self.add_fun(
            "@op:sub",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            DEF_ID_NATIVE,
        );
        self.add_fun(
            "@op:mul",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            DEF_ID_NATIVE,
        );
        self.add_fun(
            "@op:div",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            DEF_ID_NATIVE,
        );
        self.add_fun(
            "@op:mod",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            DEF_ID_NATIVE,
        );
        self.add_fun(
            "@op:neg",
            FnType::new(vec![Type::Val(TypeVal::Int)], Type::Val(TypeVal::Int)),
            DEF_ID_NATIVE,
        );
        self.add_fun(
            "@op:not",
            FnType::new(vec![Type::Val(TypeVal::Int)], Type::Val(TypeVal::Int)),
            DEF_ID_NATIVE,
        );

        self.add_fun(
            "putc",
            FnType::new(vec![Type::Val(TypeVal::Int)], Type::Void),
            DEF_ID_NATIVE,
        );
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
