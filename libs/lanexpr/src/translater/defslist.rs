use crate::ast;
use crate::ast::AST;

struct DefFunsFinder<'a> {
    root: &'a dyn ast::ASTExpr,
    res: Vec<&'a ast::ASTDefFun>,
}

impl<'a> DefFunsFinder<'a> {
    pub fn new(root: &'a dyn ast::ASTExpr) -> Self {
        DefFunsFinder { root, res: vec![] }
    }

    pub fn run(mut self) -> Vec<&'a ast::ASTDefFun> {
        self.root.accept(&mut self);
        self.res
    }
}

impl<'a> ast::ASTVisitor for DefFunsFinder<'a> {
    fn visit_def_arg(&mut self, node: &ast::ASTDefArg) {
        node.accept_children(self);
    }

    fn visit_def_fun(&mut self, node: &ast::ASTDefFun) {
        node.accept_children(self);
        let node = unsafe { std::mem::transmute::<&ast::ASTDefFun, &'a ast::ASTDefFun>(node) };
        self.res.push(node);
    }

    fn visit_def_var(&mut self, node: &ast::ASTDefVar) {
        node.accept_children(self);
    }
    fn visit_expr_block(&mut self, node: &ast::ASTExprBlock) {
        node.accept_children(self);
    }

    fn visit_expr_call(&mut self, node: &ast::ASTExprCall) {
        node.accept_children(self);
    }
    fn visit_expr_const(&mut self, node: &ast::ASTExprConst) {
        node.accept_children(self);
    }
    fn visit_expr_id(&mut self, node: &ast::ASTExprId) {
        node.accept_children(self);
    }
    fn visit_expr_if(&mut self, node: &ast::ASTExprIf) {
        node.accept_children(self);
    }
    fn visit_expr_let(&mut self, node: &ast::ASTExprLet) {
        node.accept_children(self);
    }
    fn visit_expr_while(&mut self, node: &ast::ASTExprWhile) {
        node.accept_children(self);
    }
    fn visit_type_name(&mut self, node: &ast::ASTTypeName) {
        node.accept_children(self);
    }
}

pub fn list_fun_defs<'a>(root: &'a dyn ast::ASTExpr) -> Vec<&'a ast::ASTDefFun> {
    let finder = DefFunsFinder::new(root);
    finder.run()
}
