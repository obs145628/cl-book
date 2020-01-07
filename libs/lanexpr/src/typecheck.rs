use crate::ast::*;
use crate::defstable::DefsTable;
use crate::letype::{Type, TypeVal};

struct TypeCheck {
    res: Option<Type>,
    defs: DefsTable,
}

impl TypeCheck {
    pub fn new() -> TypeCheck {
        TypeCheck {
            res: None,
            defs: DefsTable::new(),
        }
    }

    pub fn check(&mut self, node: &ASTExprPtr) {
        self.defs.open_scope_fn();

        node.accept(self);

        self.defs.close_scope_fn();
        if self.defs.total_scopes_len() != 1 {
            panic!("check AST end: defs table must be of size 1");
        }
    }

    fn get_exp_type(&mut self, node: &ASTExprPtr) -> Type {
        let saved_ty = self.defs.get_exp_type(&**node);
        if !saved_ty.is_none() {
            return saved_ty.unwrap();
        }

        self.res = None;
        node.accept(self);
        let mut res: Option<Type> = None;
        std::mem::swap(&mut res, &mut self.res);
        let new_ty = res.unwrap();
        self.defs.set_exp_type(&**node, new_ty.clone());
        new_ty
    }

    fn get_typename_type(&mut self, node: &ASTTypePtr) -> Type {
        let saved_ty = self.defs.get_typename_type(&**node);
        if !saved_ty.is_none() {
            return saved_ty.unwrap();
        }

        self.res = None;
        node.accept(self);
        let mut res: Option<Type> = None;
        std::mem::swap(&mut res, &mut self.res);
        let new_ty = res.unwrap();
        self.defs.set_typename_type(&**node, new_ty.clone());
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

    // => node.empty() ? <void> : type(node.last())
    fn visit_expr_block(&mut self, node: &ASTExprBlock) {
        for child in node.exprs() {
            self.get_exp_type(child);
        }

        if node.exprs().len() == 0 {
            self.res = Some(Type::Void);
        }
    }

    // => fntype(node.name()).ret
    fn visit_expr_call(&mut self, node: &ASTExprCall) {
        let args: Vec<Type> = node
            .args()
            .iter()
            .map(|arg| self.get_exp_type(arg))
            .collect();

        let fun_ty = self.defs.get_fun(node.name()).unwrap();
        let fun_args = fun_ty.args();
        if fun_args.len() != fun_args.len() {
            panic!(
                "Invalid fun call to {}, epexted {} arguments, got {}",
                node.name(),
                fun_args.len(),
                args.len()
            );
        }

        for i in 0..fun_args.len() {
            if (!args[i].can_be_cast_to(&fun_args[i])) {
                panic!(
                    "Invalid fun call to {}, argument #{} is {:?}, expected {:?}",
                    node.name(),
                    i + 1,
                    args[i],
                    fun_args[i]
                );
            }
        }

        self.res = Some(*fun_ty.ret());
    }

    // => <int>
    fn visit_expr_const(&mut self, _: &ASTExprConst) {
        self.res = Some(Type::Val(TypeVal::Int));
    }

    // => typename(node.name())
    fn visit_expr_id(&mut self, node: &ASTExprId) {
        let var_ty = self.defs.get_var(node.name());
        if var_ty.is_none() {
            panic!("Usage of undefined variable {}", node.name());
        }
        self.res = Some(var_ty.unwrap());
    }

    // if (<int>) then <a> else <a> => <a>
    fn visit_expr_if(&mut self, node: &ASTExprIf) {
        let ty_cond = self.get_exp_type(node.cond());
        if !ty_cond.can_be_cast_to(&Type::Val(TypeVal::Int)) {
            panic!("ExprIf: condition type must be int, got {:?}", ty_cond);
        }

        let ty_if = self.get_exp_type(node.val_if());
        let ty_else = self.get_exp_type(node.val_else());

        if !ty_else.can_be_cast_to(&ty_if) {
            panic!(
                "ExprIf: types for if ({:?}) and else ({:?}) differ",
                ty_if, ty_else
            );
        }

        self.res = Some(ty_if);
    }

    fn visit_expr_let(&mut self, node: &ASTExprLet) {
        //TODO: open scope
        self.defs.open_scope();

        //TODO: handle defs

        let val_ty = self.get_exp_type(node.val());
        self.res = Some(val_ty);

        self.defs.close_scope();
    }

    // while (<int>) do <void> => <void>
    fn visit_expr_while(&mut self, node: &ASTExprWhile) {
        let ty_cond = self.get_exp_type(node.cond());
        if !ty_cond.can_be_cast_to(&Type::Val(TypeVal::Int)) {
            panic!("ExprWhile: condition type must be int, got {:?}", ty_cond);
        }

        let ty_body = self.get_exp_type(node.body());
        if !ty_body.can_be_cast_to(&Type::Void) {
            panic!("ExprWhile:  body type must be void, got {:?}", ty_body);
        }

        self.res = Some(Type::Void);
    }

    fn visit_type_name(&mut self, node: &ASTTypeName) {
        match self.defs.get_type(node.name()) {
            Some(ty) => self.res = Some(ty),
            None => panic!("unknown typename '{}'", node.name()),
        }
    }
}
