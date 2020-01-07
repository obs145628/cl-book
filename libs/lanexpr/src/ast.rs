use obuid;

pub trait AST {
    fn accept(&self, v: &mut dyn ASTVisitor);
    fn get_uid(&self) -> usize;
}
pub type ASTPtr = Box<dyn AST>;

pub trait ASTVisitor {
    fn visit_def_fun(&mut self, node: &ASTDefFun);
    fn visit_def_var(&mut self, node: &ASTDefVar);
    fn visit_expr_block(&mut self, node: &ASTExprBlock);
    fn visit_expr_call(&mut self, node: &ASTExprCall);
    fn visit_expr_const(&mut self, node: &ASTExprConst);
    fn visit_expr_id(&mut self, node: &ASTExprId);
    fn visit_expr_if(&mut self, node: &ASTExprIf);
    fn visit_expr_let(&mut self, node: &ASTExprLet);
    fn visit_expr_while(&mut self, node: &ASTExprWhile);
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
    uid: usize,
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
            uid: obuid::unique_usize(),
        })
    }
}

impl AST for ASTDefFun {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_def_fun(self);
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}
impl ASTDef for ASTDefFun {}

pub struct ASTDefVar {
    name: String,
    ty: Option<ASTTypePtr>,
    init: ASTExprPtr,
    uid: usize,
}

impl ASTDefVar {
    pub fn new(name: String, ty: Option<ASTTypePtr>, init: ASTExprPtr) -> Box<ASTDefVar> {
        Box::new(ASTDefVar {
            name,
            ty,
            init,
            uid: obuid::unique_usize(),
        })
    }
}

impl AST for ASTDefVar {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_def_var(self);
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}
impl ASTDef for ASTDefVar {}

pub struct ASTExprBlock {
    exprs: Vec<ASTExprPtr>,
    uid: usize,
}

impl ASTExprBlock {
    pub fn new(exprs: Vec<ASTExprPtr>) -> Box<ASTExprBlock> {
        Box::new(ASTExprBlock {
            exprs,
            uid: obuid::unique_usize(),
        })
    }

    pub fn exprs(&self) -> &Vec<ASTExprPtr> {
        &self.exprs
    }
}

impl AST for ASTExprBlock {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_expr_block(self);
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}
impl ASTExpr for ASTExprBlock {}

pub struct ASTExprCall {
    name: String,
    args: Vec<ASTExprPtr>,
    uid: usize,
}

impl ASTExprCall {
    pub fn new(name: String, args: Vec<ASTExprPtr>) -> Box<ASTExprCall> {
        Box::new(ASTExprCall {
            name,
            args,
            uid: obuid::unique_usize(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn args(&self) -> &Vec<ASTExprPtr> {
        &self.args
    }
}

impl AST for ASTExprCall {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_expr_call(self);
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}
impl ASTExpr for ASTExprCall {}

pub struct ASTExprConst {
    val: i32,
    uid: usize,
}

impl ASTExprConst {
    pub fn new(val: i32) -> Box<ASTExprConst> {
        Box::new(ASTExprConst {
            val,
            uid: obuid::unique_usize(),
        })
    }

    pub fn val(&self) -> i32 {
        self.val
    }
}

impl AST for ASTExprConst {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_expr_const(self);
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}
impl ASTExpr for ASTExprConst {}

pub struct ASTExprId {
    name: String,
    uid: usize,
}

impl ASTExprId {
    pub fn new(name: String) -> Box<ASTExprId> {
        Box::new(ASTExprId {
            name,
            uid: obuid::unique_usize(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl AST for ASTExprId {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_expr_id(self);
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}
impl ASTExpr for ASTExprId {}

pub struct ASTExprIf {
    cond: ASTExprPtr,
    val_if: ASTExprPtr,
    val_else: ASTExprPtr,
    uid: usize,
}

impl ASTExprIf {
    pub fn new(cond: ASTExprPtr, val_if: ASTExprPtr, val_else: ASTExprPtr) -> Box<ASTExprIf> {
        Box::new(ASTExprIf {
            cond,
            val_if,
            val_else,
            uid: obuid::unique_usize(),
        })
    }

    pub fn cond(&self) -> &ASTExprPtr {
        &self.cond
    }

    pub fn val_if(&self) -> &ASTExprPtr {
        &self.val_if
    }

    pub fn val_else(&self) -> &ASTExprPtr {
        &self.val_else
    }
}

impl AST for ASTExprIf {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_expr_if(self);
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}
impl ASTExpr for ASTExprIf {}

pub struct ASTExprLet {
    defs: Vec<ASTDefPtr>,
    val: ASTExprPtr,
    uid: usize,
}

impl ASTExprLet {
    pub fn new(defs: Vec<ASTDefPtr>, val: ASTExprPtr) -> Box<ASTExprLet> {
        Box::new(ASTExprLet {
            defs,
            val,
            uid: obuid::unique_usize(),
        })
    }

    pub fn defs(&self) -> &Vec<ASTDefPtr> {
        &self.defs
    }

    pub fn val(&self) -> &ASTExprPtr {
        &self.val
    }
}

impl AST for ASTExprLet {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_expr_let(self);
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}
impl ASTExpr for ASTExprLet {}

pub struct ASTExprWhile {
    cond: ASTExprPtr,
    body: ASTExprPtr,
    uid: usize,
}

impl ASTExprWhile {
    pub fn new(cond: ASTExprPtr, body: ASTExprPtr) -> Box<ASTExprWhile> {
        Box::new(ASTExprWhile {
            cond,
            body,
            uid: obuid::unique_usize(),
        })
    }

    pub fn cond(&self) -> &ASTExprPtr {
        &self.cond
    }

    pub fn body(&self) -> &ASTExprPtr {
        &self.body
    }
}

impl AST for ASTExprWhile {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_expr_while(self);
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}
impl ASTExpr for ASTExprWhile {}

pub struct ASTTypeName {
    name: String,
    uid: usize,
}

impl ASTTypeName {
    pub fn new(name: String) -> Box<ASTTypeName> {
        Box::new(ASTTypeName {
            name,
            uid: obuid::unique_usize(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}

impl AST for ASTTypeName {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_type_name(self);
    }

    fn get_uid(&self) -> usize {
        self.uid
    }
}
impl ASTType for ASTTypeName {}
