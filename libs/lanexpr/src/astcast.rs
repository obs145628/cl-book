use crate::ast;

pub enum ASTStatic {
    DefArg,
    DefFun,
    DefVar,
    ExprBlock,
    ExprCall,
    ExprConst(i32),
    ExprId(String),
    ExprIf,
    ExprLet,
    ExprWhile,
    TypeName(String),
}

impl ASTStatic {
    pub fn resolve(node: &ast::ASTExprPtr) -> ASTStatic {
        let mut v = ASTGetStatic { res: None };
        node.accept(&mut v);
        v.res.unwrap()
    }
}

struct ASTGetStatic {
    res: Option<ASTStatic>,
}

impl ast::ASTVisitor for ASTGetStatic {
    fn visit_def_arg(&mut self, _: &ast::ASTDefArg) {
        self.res = Some(ASTStatic::DefArg);
    }

    fn visit_def_fun(&mut self, _: &ast::ASTDefFun) {
        self.res = Some(ASTStatic::DefFun);
    }

    fn visit_def_var(&mut self, _: &ast::ASTDefVar) {
        self.res = Some(ASTStatic::DefVar);
    }

    fn visit_expr_block(&mut self, _: &ast::ASTExprBlock) {
        self.res = Some(ASTStatic::ExprBlock);
    }

    fn visit_expr_call(&mut self, _: &ast::ASTExprCall) {
        self.res = Some(ASTStatic::ExprCall);
    }

    fn visit_expr_const(&mut self, node: &ast::ASTExprConst) {
        self.res = Some(ASTStatic::ExprConst(node.val()));
    }

    fn visit_expr_id(&mut self, node: &ast::ASTExprId) {
        self.res = Some(ASTStatic::ExprId(node.name().to_string()));
    }

    fn visit_expr_if(&mut self, _: &ast::ASTExprIf) {
        self.res = Some(ASTStatic::ExprIf);
    }

    fn visit_expr_let(&mut self, _: &ast::ASTExprLet) {
        self.res = Some(ASTStatic::ExprLet);
    }

    fn visit_expr_while(&mut self, _: &ast::ASTExprWhile) {
        self.res = Some(ASTStatic::ExprWhile);
    }

    fn visit_type_name(&mut self, node: &ast::ASTTypeName) {
        self.res = Some(ASTStatic::TypeName(node.name().to_string()));
    }
}
