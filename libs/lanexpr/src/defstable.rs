use crate::ast::*;
use crate::letype::{FnType, Type, TypeVal};

use std::collections::HashMap;

pub struct DefFun {
    pub id: DefFunId,
    pub ast_id: ASTUid,
    pub ty: FnType,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DefFunId {
    id: usize,
}

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

    exp_types: HashMap<ASTUid, Type>, //contain type for every ASTExpr
    typename_types: HashMap<ASTUid, Type>, //contain type for every ASTType

    defs_fns: Vec<DefFun>,
    act_fn: Option<DefFunId>,

    ast_def_fun_defs: HashMap<ASTUid, DefFunId>, //contain DefFunId for every ASTDefFun
    ast_expr_call_defs: HashMap<ASTUid, DefFunId>, //contain DefFunId of the function called by every ASTExprCall
}

impl DefsTable {
    pub fn new() -> DefsTable {
        let mut res = DefsTable {
            scopes_tys: vec![],
            scopes_fns: vec![],
            scopes_vars: vec![],
            fn_scopes_pos: vec![],

            exp_types: HashMap::new(),
            typename_types: HashMap::new(),

            defs_fns: vec![],
            act_fn: None,

            ast_def_fun_defs: HashMap::new(),
            ast_expr_call_defs: HashMap::new(),
        };
        res.open_scope();
        res.set_native_defs();
        res
    }

    pub fn reset_actual_fn(&mut self) {
        self.act_fn = None;
    }

    pub fn change_actual_fn(&mut self, node: &ASTDefFun) {
        let fn_id = self.ast_def_fun_defs.get(&node.get_uid()).unwrap();
        self.act_fn = Some(*fn_id);
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

    pub fn add_fun(&mut self, id: &str, ty: FnType, ast_id: ASTUid) -> DefFunId {
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
            ASTUid::none(),
        );
        self.add_fun(
            "@op:eq",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            ASTUid::none(),
        );
        self.add_fun(
            "@op:lt",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            ASTUid::none(),
        );
        self.add_fun(
            "@op:gt",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            ASTUid::none(),
        );
        self.add_fun(
            "@op:add",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            ASTUid::none(),
        );
        self.add_fun(
            "@op:sub",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            ASTUid::none(),
        );
        self.add_fun(
            "@op:mul",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            ASTUid::none(),
        );
        self.add_fun(
            "@op:div",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            ASTUid::none(),
        );
        self.add_fun(
            "@op:mod",
            FnType::new(
                vec![Type::Val(TypeVal::Int), Type::Val(TypeVal::Int)],
                Type::Val(TypeVal::Int),
            ),
            ASTUid::none(),
        );
        self.add_fun(
            "@op:neg",
            FnType::new(vec![Type::Val(TypeVal::Int)], Type::Val(TypeVal::Int)),
            ASTUid::none(),
        );
        self.add_fun(
            "@op:not",
            FnType::new(vec![Type::Val(TypeVal::Int)], Type::Val(TypeVal::Int)),
            ASTUid::none(),
        );

        self.add_fun(
            "putc",
            FnType::new(vec![Type::Val(TypeVal::Int)], Type::Void),
            ASTUid::none(),
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

    pub fn get_ast_expr_call_def(&self, node: &ASTExprCall) -> Option<DefFunId> {
        let uid = node.get_uid();
        self.ast_expr_call_defs.get(&uid).copied()
    }

    pub fn set_ast_expr_call_def(&mut self, node: &ASTExprCall, def: DefFunId) {
        let uid = node.get_uid();
        assert!(self.ast_expr_call_defs.get(&uid).is_none());
        self.ast_expr_call_defs.insert(uid, def);
    }
}
