use obuid;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ASTUid {
    id: usize,
}

impl ASTUid {
    fn next() -> ASTUid {
        ASTUid {
            id: obuid::unique_usize(),
        }
    }

    pub fn none() -> ASTUid {
        ASTUid { id: 0 }
    }
}

pub trait AST {
    fn accept(&self, v: &mut dyn ASTVisitor);
    fn accept_children(&self, v: &mut dyn ASTVisitor);
    fn get_uid(&self) -> ASTUid;
}
pub type ASTPtr = Box<dyn AST>;

pub trait ASTVisitor {
    fn visit_def_arg(&mut self, node: &ASTDefArg);
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

pub struct ASTDefArg {
    name: String,
    ty: ASTTypePtr,
    uid: ASTUid,
}

impl ASTDefArg {
    pub fn new(name: String, ty: ASTTypePtr) -> Box<ASTDefArg> {
        Box::new(ASTDefArg {
            name,
            ty,
            uid: ASTUid::next(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &ASTTypePtr {
        &self.ty
    }
}

impl AST for ASTDefArg {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_def_arg(self);
    }

    fn accept_children(&self, v: &mut dyn ASTVisitor) {
        self.ty.accept(v);
    }

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}
impl ASTDef for ASTDefArg {}

pub struct ASTDefFun {
    name: String,
    args: Vec<Box<ASTDefArg>>,
    ret: ASTTypePtr,
    body: ASTExprPtr,
    uid: ASTUid,
}

impl ASTDefFun {
    pub fn new(
        name: String,
        args: Vec<Box<ASTDefArg>>,
        ret: ASTTypePtr,
        body: ASTExprPtr,
    ) -> Box<ASTDefFun> {
        Box::new(ASTDefFun {
            name,
            args,
            ret,
            body,
            uid: ASTUid::next(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn args(&self) -> &Vec<Box<ASTDefArg>> {
        &self.args
    }

    pub fn ret(&self) -> &ASTTypePtr {
        &self.ret
    }

    pub fn body(&self) -> &ASTExprPtr {
        &self.body
    }
}

impl AST for ASTDefFun {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_def_fun(self);
    }

    fn accept_children(&self, v: &mut dyn ASTVisitor) {
        for arg in &self.args {
            arg.accept(v);
        }
        self.ret.accept(v);
        self.body.accept(v);
    }

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}

impl ASTDef for ASTDefFun {}

pub struct ASTDefVar {
    name: String,
    ty: Option<ASTTypePtr>,
    init: ASTExprPtr,
    uid: ASTUid,
}

impl ASTDefVar {
    pub fn new(name: String, ty: Option<ASTTypePtr>, init: ASTExprPtr) -> Box<ASTDefVar> {
        Box::new(ASTDefVar {
            name,
            ty,
            init,
            uid: ASTUid::next(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> Option<&ASTTypePtr> {
        self.ty.as_ref()
    }

    pub fn init(&self) -> &ASTExprPtr {
        &self.init
    }
}

impl AST for ASTDefVar {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_def_var(self);
    }

    fn accept_children(&self, v: &mut dyn ASTVisitor) {
        if let Some(ty) = &self.ty {
            ty.accept(v);
        }
        self.init.accept(v);
    }

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}
impl ASTDef for ASTDefVar {}

pub struct ASTExprBlock {
    exprs: Vec<ASTExprPtr>,
    uid: ASTUid,
}

impl ASTExprBlock {
    pub fn new(exprs: Vec<ASTExprPtr>) -> Box<ASTExprBlock> {
        Box::new(ASTExprBlock {
            exprs,
            uid: ASTUid::next(),
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

    fn accept_children(&self, v: &mut dyn ASTVisitor) {
        for expr in &self.exprs {
            expr.accept(v);
        }
    }

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}
impl ASTExpr for ASTExprBlock {}

pub struct ASTExprCall {
    name: String,
    args: Vec<ASTExprPtr>,
    uid: ASTUid,
}

impl ASTExprCall {
    pub fn new(name: String, args: Vec<ASTExprPtr>) -> Box<ASTExprCall> {
        Box::new(ASTExprCall {
            name,
            args,
            uid: ASTUid::next(),
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

    fn accept_children(&self, v: &mut dyn ASTVisitor) {
        for arg in &self.args {
            arg.accept(v);
        }
    }

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}
impl ASTExpr for ASTExprCall {}

pub struct ASTExprConst {
    val: i32,
    uid: ASTUid,
}

impl ASTExprConst {
    pub fn new(val: i32) -> Box<ASTExprConst> {
        Box::new(ASTExprConst {
            val,
            uid: ASTUid::next(),
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

    fn accept_children(&self, _v: &mut dyn ASTVisitor) {}

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}
impl ASTExpr for ASTExprConst {}

pub struct ASTExprId {
    name: String,
    uid: ASTUid,
}

impl ASTExprId {
    pub fn new(name: String) -> Box<ASTExprId> {
        Box::new(ASTExprId {
            name,
            uid: ASTUid::next(),
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

    fn accept_children(&self, _v: &mut dyn ASTVisitor) {}

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}
impl ASTExpr for ASTExprId {}

pub struct ASTExprIf {
    cond: ASTExprPtr,
    val_if: ASTExprPtr,
    val_else: ASTExprPtr,
    uid: ASTUid,
}

impl ASTExprIf {
    pub fn new(cond: ASTExprPtr, val_if: ASTExprPtr, val_else: ASTExprPtr) -> Box<ASTExprIf> {
        Box::new(ASTExprIf {
            cond,
            val_if,
            val_else,
            uid: ASTUid::next(),
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

    fn accept_children(&self, v: &mut dyn ASTVisitor) {
        self.cond.accept(v);
        self.val_if.accept(v);
        self.val_else.accept(v);
    }

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}
impl ASTExpr for ASTExprIf {}

pub struct ASTExprLet {
    defs: Vec<ASTDefPtr>,
    val: ASTExprPtr,
    uid: ASTUid,
}

impl ASTExprLet {
    pub fn new(defs: Vec<ASTDefPtr>, val: ASTExprPtr) -> Box<ASTExprLet> {
        Box::new(ASTExprLet {
            defs,
            val,
            uid: ASTUid::next(),
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

    fn accept_children(&self, v: &mut dyn ASTVisitor) {
        for def in &self.defs {
            def.accept(v);
        }
        self.val.accept(v);
    }

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}
impl ASTExpr for ASTExprLet {}

pub struct ASTExprWhile {
    cond: ASTExprPtr,
    body: ASTExprPtr,
    uid: ASTUid,
}

impl ASTExprWhile {
    pub fn new(cond: ASTExprPtr, body: ASTExprPtr) -> Box<ASTExprWhile> {
        Box::new(ASTExprWhile {
            cond,
            body,
            uid: ASTUid::next(),
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

    fn accept_children(&self, v: &mut dyn ASTVisitor) {
        self.cond.accept(v);
        self.body.accept(v);
    }

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}
impl ASTExpr for ASTExprWhile {}

pub struct ASTTypeName {
    name: String,
    uid: ASTUid,
}

impl ASTTypeName {
    pub fn new(name: String) -> Box<ASTTypeName> {
        Box::new(ASTTypeName {
            name,
            uid: ASTUid::next(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl AST for ASTTypeName {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_type_name(self);
    }

    fn accept_children(&self, _v: &mut dyn ASTVisitor) {}

    fn get_uid(&self) -> ASTUid {
        self.uid
    }
}
impl ASTType for ASTTypeName {}
