use crate::ast::{ASTExpr, ASTExprCall, ASTExprId, ASTType, ASTUid, AST};
use crate::bindvar::{BindVar, BindVarId, BindVarsList};
use crate::letype::{FnType, Type};

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BindFunId {
    id: usize,
}

impl BindFunId {
    pub fn new(id: usize) -> Self {
        BindFunId { id }
    }

    pub fn as_usize(&self) -> usize {
        self.id
    }
}

pub struct BindFun {
    name: String,
    ty: FnType,
    id: BindFunId,
    ast_id: Option<ASTUid>, //native functions have no ast_id
    vars: BindVarsList,

    ast2var: HashMap<ASTUid, BindVarId>, //var for every ASTDefVar in the function

    exp_types: HashMap<ASTUid, Type>, //type for every ASTExpr in the function
    typename_types: HashMap<ASTUid, Type>, //type for every ASTType in the function

    exp_id_vars: HashMap<ASTUid, BindVarId>, //var for every ASTExprId in the function
    exp_call_funs: HashMap<ASTUid, BindFunId>, //fun for every ASTExprCall in the function
}

impl BindFun {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &FnType {
        &self.ty
    }

    pub fn id(&self) -> BindFunId {
        self.id
    }

    pub fn ast_id(&self) -> Option<ASTUid> {
        self.ast_id
    }

    pub fn is_native(&self) -> bool {
        self.ast_id.is_none()
    }

    pub fn dump_bindings(&self) {
        let code_kind = if self.is_native() { "NATIVE" } else { "USER" };
        println!(
            "[FUN][{}] {}: {:?} ({} CODE)",
            self.id.id, self.name, self.ty, code_kind
        );

        for (i, var) in self.vars.get_vars().iter().enumerate() {
            if i == self.ty.args().len() {
                println!("   Locals:");
            } else if i == 0 {
                println!("Arguments:");
            }
            var.dump_bindings();
        }
        println!();
    }

    pub fn add_var(&mut self, name: &str, ty: Type, ast_id: ASTUid) -> BindVarId {
        let var_id = self.vars.add_var(name, ty, ast_id);
        self.ast2var.insert(ast_id, var_id);
        var_id
    }

    pub fn get_var(&self, id: BindVarId) -> &BindVar {
        self.vars.get_var(id)
    }

    pub fn get_var_from_ast(&self, ast_id: ASTUid) -> &BindVar {
        let var_id = self.ast2var.get(&ast_id).unwrap();
        self.get_var(*var_id)
    }

    pub fn get_type_of_exp(&self, node: &dyn ASTExpr) -> Option<Type> {
        let id = node.get_uid();
        self.exp_types.get(&id).copied()
    }

    pub fn set_type_of_exp(&mut self, node: &dyn ASTExpr, ty: Type) {
        let id = node.get_uid();
        assert!(self.exp_types.get(&id).is_none());
        self.exp_types.insert(id, ty);
    }

    pub fn get_type_of_typename(&self, node: &dyn ASTType) -> Option<Type> {
        let id = node.get_uid();
        self.typename_types.get(&id).copied()
    }

    pub fn set_type_of_typename(&mut self, node: &dyn ASTType, ty: Type) {
        let id = node.get_uid();
        assert!(self.typename_types.get(&id).is_none());
        self.typename_types.insert(id, ty);
    }

    pub fn get_var_of_exp_id(&self, node: &ASTExprId) -> Option<BindVarId> {
        let id = node.get_uid();
        self.exp_id_vars.get(&id).copied()
    }

    pub fn set_var_of_exp_id(&mut self, node: &ASTExprId, var_id: BindVarId) {
        let id = node.get_uid();
        assert!(self.exp_id_vars.get(&id).is_none());
        self.exp_id_vars.insert(id, var_id);
    }

    pub fn get_fun_of_exp_call(&self, node: &ASTExprCall) -> Option<BindFunId> {
        let id = node.get_uid();
        self.exp_call_funs.get(&id).copied()
    }

    pub fn set_fun_of_exp_call(&mut self, node: &ASTExprCall, fun_id: BindFunId) {
        let id = node.get_uid();
        assert!(self.exp_call_funs.get(&id).is_none());
        self.exp_call_funs.insert(id, fun_id);
    }
}

pub struct BindFunsList {
    list: Vec<BindFun>,
}

impl BindFunsList {
    pub fn new() -> Self {
        BindFunsList { list: vec![] }
    }

    pub fn add_fun(&mut self, name: &str, ty: FnType, ast_id: Option<ASTUid>) -> BindFunId {
        let id = BindFunId {
            id: self.list.len(),
        };

        self.list.push(BindFun {
            name: String::from(name),
            ty,
            id,
            ast_id,
            vars: BindVarsList::new(),
            ast2var: HashMap::new(),

            exp_types: HashMap::new(),
            typename_types: HashMap::new(),

            exp_id_vars: HashMap::new(),
            exp_call_funs: HashMap::new(),
        });
        id
    }

    pub fn get_fun(&self, id: BindFunId) -> &BindFun {
        &self.list[id.id]
    }

    pub fn get_fun_mut(&mut self, id: BindFunId) -> &mut BindFun {
        &mut self.list[id.id]
    }

    pub fn get_funs(&self) -> &Vec<BindFun> {
        &self.list
    }
}
