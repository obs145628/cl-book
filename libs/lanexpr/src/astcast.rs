use crate::ast;

struct CasterToExprId<'a> {
    node: &'a dyn ast::ASTExpr,
    res: Option<&'a ast::ASTExprId>,
}

impl<'a> CasterToExprId<'a> {
    fn new(node: &'a dyn ast::ASTExpr) -> Self {
        CasterToExprId { node, res: None }
    }

    fn run(mut self) -> Option<&'a ast::ASTExprId> {
        self.node.accept(&mut self);
        self.res
    }
}

impl<'a> ast::ASTVisitor for CasterToExprId<'a> {
    fn visit_def_arg(&mut self, _node: &ast::ASTDefArg) {
        unreachable!();
    }
    fn visit_def_fun(&mut self, _node: &ast::ASTDefFun) {
        unreachable!();
    }

    fn visit_def_var(&mut self, _node: &ast::ASTDefVar) {
        unreachable!();
    }
    fn visit_expr_block(&mut self, _node: &ast::ASTExprBlock) {}
    fn visit_expr_call(&mut self, _node: &ast::ASTExprCall) {}
    fn visit_expr_const(&mut self, _node: &ast::ASTExprConst) {}
    fn visit_expr_id(&mut self, node: &ast::ASTExprId) {
        let node = unsafe { std::mem::transmute::<&ast::ASTExprId, &'a ast::ASTExprId>(node) };
        self.res = Some(node);
    }
    fn visit_expr_if(&mut self, _node: &ast::ASTExprIf) {}
    fn visit_expr_let(&mut self, _node: &ast::ASTExprLet) {}
    fn visit_expr_while(&mut self, _node: &ast::ASTExprWhile) {}
    fn visit_type_name(&mut self, _node: &ast::ASTTypeName) {}
}

pub fn cast_to_expr_id<'a>(node: &'a dyn ast::ASTExpr) -> Option<&'a ast::ASTExprId> {
    let caster = CasterToExprId::new(node);
    caster.run()
}
