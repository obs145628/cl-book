use crate::ast::*;
use crate::letype::{FnType, Type, TypeVal};

use std::collections::HashMap;

pub struct DefFun {
    pub id: DefFunId,
    pub ast_id: ASTUid,
    pub ty: FnType,

    vars: Vec<DefVar>,
}

impl DefFun {
    fn add_var(&mut self, ast_id: ASTUid, ty: Type) -> DefVarId {
        let var_id = DefVarId {
            id: self.vars.len(),
        };
        self.vars.push(DefVar {
            id: var_id,
            fn_id: self.id,
            ast_id,
            ty,
        });
        var_id
    }

    fn get_var(&self, var_id: DefVarId) -> &DefVar {
        &self.vars[var_id.as_usize()]
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DefFunId {
    id: usize,
}

impl DefFunId {
    pub fn as_usize(&self) -> usize {
        self.id
    }

    pub fn fn_main() -> DefFunId {
        DefFunId { id: 0 }
    }
}

pub struct DefVar {
    pub id: DefVarId,
    pub fn_id: DefFunId,
    pub ast_id: ASTUid,
    pub ty: Type,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DefVarId {
    id: usize,
}

impl DefVarId {
    pub fn as_usize(&self) -> usize {
        self.id
    }
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
    scopes_vars: Vec<HashMap<String, DefVarId>>,
    fn_scopes_pos: Vec<usize>,
    fn_scopes_ids: Vec<DefFunId>,

    exp_types: HashMap<ASTUid, Type>, //contain type for every ASTExpr
    typename_types: HashMap<ASTUid, Type>, //contain type for every ASTType

    defs_fns: Vec<DefFun>,

    ast_def_fun_defs: HashMap<ASTUid, DefFunId>, //contain DefFunId for every ASTDefFun
    ast_expr_call_defs: HashMap<ASTUid, DefFunId>, //contain DefFunId of the function called by every ASTExprCall
    ast_def_var_defs: HashMap<ASTUid, DefVarId>, //contain DefVarId for every variable def (ASTDefVar) and arg def (ASTDefArg)
    ast_expr_id_defs: HashMap<ASTUid, DefVarId>, //contain DefVarId of the var referenced by every ASTExprId
}

impl DefsTable {
    pub fn new() -> DefsTable {
        let mut res = DefsTable {
            scopes_tys: vec![],
            scopes_fns: vec![],
            scopes_vars: vec![],
            fn_scopes_pos: vec![],
            fn_scopes_ids: vec![],

            exp_types: HashMap::new(),
            typename_types: HashMap::new(),

            defs_fns: vec![],

            ast_def_fun_defs: HashMap::new(),
            ast_expr_call_defs: HashMap::new(),
            ast_def_var_defs: HashMap::new(),
            ast_expr_id_defs: HashMap::new(),
        };
        res.open_scope();
        res.set_native_defs();
        res
    }

    pub fn total_scopes_len(&self) -> usize {
        self.scopes_tys.len()
    }

    pub fn actual_fn(&self) -> &DefFun {
        let fn_id = self.fn_scopes_ids.last().unwrap();
        &self.defs_fns[fn_id.as_usize()]
    }

    pub fn actual_fn_as_mut(&mut self) -> &mut DefFun {
        let fn_id = self.fn_scopes_ids.last().unwrap();
        &mut self.defs_fns[fn_id.as_usize()]
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
            vars: vec![],
        });
        self.ast_def_fun_defs.insert(ast_id, fn_id);

        self.scopes_fns
            .last_mut()
            .unwrap()
            .insert(id.to_string(), fn_id);

        fn_id
    }

    pub fn add_var(&mut self, id: &str, ty: Type, ast_id: ASTUid) -> DefVarId {
        assert!(self.get_top_var_id(id).is_none());

        let fun = self.actual_fn_as_mut();
        let var_id = fun.add_var(ast_id, ty);
        self.ast_def_var_defs.insert(ast_id, var_id);

        self.scopes_vars
            .last_mut()
            .unwrap()
            .insert(id.to_string(), var_id);

        var_id
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

    pub fn get_top_var_id(&self, id: &str) -> Option<DefVarId> {
        self.scopes_vars.last().unwrap().get(id).copied()
    }

    pub fn get_top_var(&self, id: &str) -> Option<&DefVar> {
        match self.get_top_var_id(id) {
            Some(var_id) => Some(self.actual_fn().get_var(var_id)),
            _ => None,
        }
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

    pub fn get_var_id(&self, id: &str) -> Option<DefVarId> {
        for scope in self.scopes_vars.iter().rev() {
            let item = scope.get(id);
            if item.is_some() {
                return item.copied();
            }
        }

        None
    }

    pub fn get_scope_var(&self, id: &str) -> Option<&DefVar> {
        match self.get_var_id(id) {
            Some(var_id) => Some(self.actual_fn().get_var(var_id)),
            _ => None,
        }
    }

    pub fn get_var(&self, id: DefVarId) -> &DefVar {
        self.actual_fn().get_var(id)
    }

    pub fn open_scope(&mut self) {
        self.scopes_tys.push(HashMap::new());
        self.scopes_fns.push(HashMap::new());
        self.scopes_vars.push(HashMap::new());
    }

    pub fn open_scope_fn(&mut self, node: Option<&ASTDefFun>) {
        let beg_fn = self.scopes_tys.len();
        self.open_scope();
        self.fn_scopes_pos.push(beg_fn);

        let fn_id = match node {
            Some(def) => *self.ast_def_fun_defs.get(&def.get_uid()).unwrap(),
            None => DefFunId::fn_main(),
        };
        self.fn_scopes_ids.push(fn_id);
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
        self.fn_scopes_ids.pop().unwrap();

        self.close_scope();
    }

    fn set_native_defs(&mut self) {
        self.defs_fns.push(DefFun {
            id: DefFunId::fn_main(),
            ast_id: ASTUid::none(),
            ty: FnType::new(vec![], Type::Void),
            vars: vec![],
        });

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

    pub fn get_ast_expr_id_def(&self, node: &ASTExprId) -> Option<DefVarId> {
        let uid = node.get_uid();
        self.ast_expr_id_defs.get(&uid).copied()
    }

    pub fn set_ast_expr_id_def(&mut self, node: &ASTExprId, def: DefVarId) {
        let uid = node.get_uid();
        assert!(self.ast_expr_id_defs.get(&uid).is_none());
        self.ast_expr_id_defs.insert(uid, def);
    }
}
