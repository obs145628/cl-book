pub trait AST {
    fn accept(&self, v: &mut dyn ASTVisitor);
}
pub type ASTPtr = Box<dyn AST>;

pub trait ASTVisitor {
    fn visit_def_fun(&mut self, node: &ASTDefFun);
    fn visit_def_var(&mut self, node: &ASTDefVar);
    fn visit_expr_const(&mut self, node: &ASTExprConst);
    fn visit_expr_call(&mut self, node: &ASTExprCall);
    fn visit_expr_let(&mut self, node: &ASTExprLet);
    fn visit_type_name(&mut self, node: &ASTTypeName);
}

pub trait ASTDef: AST {}
pub type ASTDefPtr = Box<dyn ASTDef>;
pub trait ASTExpr: AST {}
pub type ASTExprPtr = Box<dyn ASTExpr>;
pub trait ASTType: AST {}
pub type ASTTypePtr = Box<dyn ASTType>;

pub struct ASTDefFun {
    name: String,
    args: Vec<(String, ASTTypePtr)>,
    ret: ASTTypePtr,
    body: ASTExprPtr,
}

impl ASTDefFun {
    pub fn new(
        name: String,
        args: Vec<(String, ASTTypePtr)>,
        ret: ASTTypePtr,
        body: ASTExprPtr,
    ) -> Box<ASTDefFun> {
        Box::new(ASTDefFun {
            name,
            args,
            ret,
            body,
        })
    }
}

impl AST for ASTDefFun {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_def_fun(self);
    }
}
impl ASTDef for ASTDefFun {}

pub struct ASTDefVar {
    name: String,
    ty: Option<ASTTypePtr>,
    init: ASTExprPtr,
}

impl ASTDefVar {
    pub fn new(name: String, ty: Option<ASTTypePtr>, init: ASTExprPtr) -> Box<ASTDefVar> {
        Box::new(ASTDefVar { name, ty, init })
    }
}

impl AST for ASTDefVar {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_def_var(self);
    }
}
impl ASTDef for ASTDefVar {}

pub struct ASTExprCall {
    name: String,
    args: Vec<ASTExprPtr>,
}

impl ASTExprCall {
    pub fn new(name: String, args: Vec<ASTExprPtr>) -> Box<ASTExprCall> {
        Box::new(ASTExprCall { name, args })
    }
}

impl AST for ASTExprCall {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_expr_call(self);
    }
}
impl ASTExpr for ASTExprCall {}

pub struct ASTExprConst {
    val: i32,
}

impl ASTExprConst {
    pub fn new(val: i32) -> Box<ASTExprConst> {
        Box::new(ASTExprConst { val })
    }
}

impl AST for ASTExprConst {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_expr_const(self);
    }
}
impl ASTExpr for ASTExprConst {}

pub struct ASTExprLet {
    defs: Vec<ASTDefPtr>,
    val: ASTExprPtr,
}

impl ASTExprLet {
    pub fn new(defs: Vec<ASTDefPtr>, val: ASTExprPtr) -> Box<ASTExprLet> {
        Box::new(ASTExprLet { defs, val })
    }
}

impl AST for ASTExprLet {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_expr_let(self);
    }
}
impl ASTExpr for ASTExprLet {}

pub struct ASTTypeName {
    name: String,
}

impl ASTTypeName {
    fn new(name: String) -> Box<ASTTypeName> {
        Box::new(ASTTypeName { name })
    }
}

impl AST for ASTTypeName {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_type_name(self);
    }
}
impl ASTType for ASTTypeName {}
