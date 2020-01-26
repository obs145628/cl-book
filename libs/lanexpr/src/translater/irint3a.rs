use irint3a::ir;
use irint3a::irbuilder::IRBuilder;

use crate::ast;
use crate::ast::AST;
use crate::bindapp::BindApp;

struct DefFunsFinder<'a> {
    root: &'a ast::ASTExprPtr,
    res: Vec<&'a ast::ASTDefFun>,
}

impl<'a> DefFunsFinder<'a> {
    pub fn new(root: &'a ast::ASTExprPtr) -> Self {
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

pub struct Translater<'a> {
    root: &'a ast::ASTExprPtr,
    app: &'a BindApp,
    builder: IRBuilder,
}

impl<'a> Translater<'a> {
    pub fn new(root: &'a ast::ASTExprPtr, app: &'a BindApp) -> Self {
        Translater {
            root,
            app,
            builder: IRBuilder::new(),
        }
    }

    pub fn translate(mut self) -> ir::ModuleExtended {
        // 1) Add native function definitions
        self.add_native_defs();

        // 2) List all user functions
        let fun_defs = {
            let get_fun = DefFunsFinder::new(self.root);
            get_fun.run()
        };

        // 3) Generate code for the user main function
        self.tl_fun(&self.root, "_f1__main");

        // 4) Generate code for all other user functions
        for (fn_idx, def) in fun_defs.iter().enumerate() {
            let fn_name = format!("_f{}_{}", fn_idx + 2, def.name());
            self.tl_fun(def.body(), &fn_name);
        }

        self.builder.build()
    }

    fn tl_fun(&mut self, _body: &ast::ASTExprPtr, name: &str) {
        self.builder.begin_function(Some(name), None);
        self.builder.ins_ret(ir::RegId(0), None);
        self.builder.end_function();
    }

    fn add_native_defs(&mut self) {
        self.builder
            .add_extern_fun(Some("_putc"), ir::FunAddress(257));
    }
}
