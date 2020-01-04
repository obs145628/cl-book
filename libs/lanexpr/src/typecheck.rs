use crate::ast::*;
use crate::letype::{LeType, LeTypePtr};
use crate::typeinfos::TypeInfos;

struct TypeCheck {
    res: Option<LeTypePtr>,
    ty_infos: TypeInfos,
}

impl TypeCheck {
    pub fn get_type(&mut self, node: &ASTExprPtr) -> LeTypePtr {
        let saved_ty = self.ty_infos.get_exp_type(&**node);
        if !saved_ty.is_none() {
            return saved_ty.unwrap().clone();
        }

        node.accept(self);
        let mut res: Option<LeTypePtr> = None;
        std::mem::swap(&mut res, &mut self.res);
        let new_ty = res.unwrap();
        self.ty_infos.set_exp_type(&**node, new_ty.clone());
        new_ty
    }
}

impl ASTVisitor for TypeCheck {
    fn visit_def_fun(&mut self, _: &ASTDefFun) {
        panic!("def_fun");
    }
    fn visit_def_var(&mut self, _: &ASTDefVar) {
        panic!("def_var");
    }

    fn visit_expr_block(&mut self, node: &ASTExprBlock) {
        for child in node.exprs() {
            self.get_type(child);
        }
    }

    fn visit_expr_call(&mut self, node: &ASTExprCall) {
        let args: Vec<LeTypePtr> = node.args().iter().map(|arg| self.get_type(arg)).collect();
    }

    fn visit_expr_const(&mut self, node: &ASTExprConst) {
        self.res = Some(LeType::new_int());
    }

    fn visit_expr_id(&mut self, node: &ASTExprId) {}

    fn visit_expr_if(&mut self, node: &ASTExprIf) {
        let ty_cond = self.get_type(node.cond());
        if !LeType::can_do_cast(&ty_cond, &LeType::new_int()) {
            panic!("ExprIf: condition type must be int, got {:?}", *ty_cond);
        }

        let ty_if = self.get_type(node.val_if());
        let ty_else = self.get_type(node.val_else());

        if !LeType::can_do_cast(&ty_else, &ty_if) {
            panic!(
                "ExprIf: types for if ({:?}) and else ({:?}) differ",
                *ty_if, *ty_else
            );
        }

        self.res = Some(ty_if.clone());
    }

    fn visit_expr_let(&mut self, node: &ASTExprLet) {}

    fn visit_expr_while(&mut self, node: &ASTExprWhile) {}

    fn visit_type_name(&mut self, _: &ASTTypeName) {
        panic!("type_name");
    }
}
