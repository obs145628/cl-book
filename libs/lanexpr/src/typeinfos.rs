use crate::ast::ASTExpr;
use crate::letype::LeTypePtr;

use std::collections::HashMap;

pub struct TypeInfos {
    exp_types: HashMap<usize, LeTypePtr>,
}

impl TypeInfos {
    pub fn new() -> TypeInfos {
        TypeInfos {
            exp_types: HashMap::new(),
        }
    }

    pub fn get_exp_type(&self, node: &dyn ASTExpr) -> Option<&LeTypePtr> {
        let uid = node.get_uid();
        self.exp_types.get(&uid)
    }

    pub fn set_exp_type(&mut self, node: &dyn ASTExpr, ty: LeTypePtr) {
        let uid = node.get_uid();
        assert!(self.exp_types.get(&uid).is_none());
        self.exp_types.insert(uid, ty);
    }
}
