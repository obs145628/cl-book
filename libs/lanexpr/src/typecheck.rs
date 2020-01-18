use crate::ast::*;
use crate::defstable::DefsTable;
use crate::letype::{FnType, Type, TypeVal};

pub struct TypeCheck {
    res: Option<Type>,
    defs: DefsTable,

    // when looking at deinfitions, only register the headers
    // used to first define all function proptotypes before looking at bodyes
    reg_headers: bool,
}

impl TypeCheck {
    pub fn new() -> TypeCheck {
        TypeCheck {
            res: None,
            defs: DefsTable::new(),
            reg_headers: false,
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

    fn check_fun_def(&mut self, node: &ASTDefFun) {
        let name = node.name();
        let args: Vec<Type> = node
            .args()
            .iter()
            .map(|arg| self.get_typename_type(arg.ty()))
            .collect();
        let ret = self.get_typename_type(node.ret());

        let fn_type = FnType::new(args, ret);
        self.defs.add_fun(name, fn_type, node.get_uid());
    }

    fn check_fun_body(&mut self, node: &ASTDefFun) {
        let fn_ret_ty = *self.defs.get_scope_fun(node.name()).unwrap().ty.ret();
        self.defs.open_scope_fn();
        self.defs.change_actual_fn(node);

        for arg in node.args() {
            let ty = self.get_typename_type(arg.ty());
            self.defs.add_var(arg.name(), ty);
        }

        let body_ty = self.get_exp_type(node.body());

        if !body_ty.can_be_cast_to(&fn_ret_ty) {
            panic!(
                "Invalid return type of function body: expected {:?}, got `{:?}",
                fn_ret_ty, body_ty
            );
        }

        self.defs.reset_actual_fn();
        self.defs.close_scope_fn();
    }

    fn check_var_def(&mut self, _: &ASTDefVar) {
        // nothing to do
    }

    fn check_var_body(&mut self, node: &ASTDefVar) {
        let name = node.name();
        let var_ty = node.ty();
        let init_ty = self.get_exp_type(node.init());

        let var_ty = match var_ty {
            None => init_ty,
            Some(ty) => {
                let ty = self.get_typename_type(ty);
                if !init_ty.can_be_cast_to(&ty) {
                    panic!(
                        "Invalid init type for variable: expected {:?}, got {:?}",
                        ty, init_ty
                    );
                }
                ty
            }
        };

        if var_ty.is_void() {
            panic!("Variable type cannot be void");
        }

        self.defs.add_var(name, var_ty);
    }
}

impl ASTVisitor for TypeCheck {
    fn visit_def_arg(&mut self, _: &ASTDefArg) {
        unreachable!();
    }

    fn visit_def_fun(&mut self, node: &ASTDefFun) {
        if self.reg_headers {
            self.check_fun_def(node);
        } else {
            self.check_fun_body(node);
        }
    }
    fn visit_def_var(&mut self, node: &ASTDefVar) {
        if self.reg_headers {
            self.check_var_def(node);
        } else {
            self.check_var_body(node);
        }
    }

    // => node.empty() ? <void> : type(node.last())
    fn visit_expr_block(&mut self, node: &ASTExprBlock) {
        let mut res = Type::Void;

        for child in node.exprs() {
            res = self.get_exp_type(child);
        }

        self.res = Some(res);
    }

    // => fntype(node.name()).ret
    fn visit_expr_call(&mut self, node: &ASTExprCall) {
        let args: Vec<Type> = node
            .args()
            .iter()
            .map(|arg| self.get_exp_type(arg))
            .collect();

        let fun_ty = match self.defs.get_scope_fun(node.name()) {
            Some(ty) => ty,
            None => panic!("Calling unknown function {}", node.name()),
        };
        let fun_id = fun_ty.id;
        let fun_args = fun_ty.ty.args();
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

        self.res = Some(*fun_ty.ty.ret());
        self.defs.set_ast_expr_call_def(node, fun_id);
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

        match var_ty.unwrap() {
            Type::Val(vty) => self.res = Some(Type::Ref(vty)),
            _ => panic!(
                "Something wrong, cannot have an ExprId {} of type '{:?}'",
                node.name(),
                var_ty.unwrap()
            ),
        }
    }

    // if (<int>) then <a> else <a> => <a>
    fn visit_expr_if(&mut self, node: &ASTExprIf) {
        let ty_cond = self.get_exp_type(node.cond());
        if !ty_cond.can_be_cast_to(&Type::Val(TypeVal::Int)) {
            panic!("ExprIf: condition type must be int, got {:?}", ty_cond);
        }

        let ty_if = self.get_exp_type(node.val_if());
        let ty_else = self.get_exp_type(node.val_else());

        if ty_else.can_be_cast_to(&ty_if) {
            self.res = Some(ty_if);
        } else if ty_if.can_be_cast_to(&ty_else) {
            self.res = Some(ty_else);
        } else {
            panic!(
                "ExprIf: types for if ({:?}) and else ({:?}) differ",
                ty_if, ty_else
            );
        }
    }

    fn visit_expr_let(&mut self, node: &ASTExprLet) {
        self.defs.open_scope();

        //TODO: handle defs
        for def in node.defs() {
            self.reg_headers = true;
            def.accept(self);
            self.reg_headers = false;
            def.accept(self);
        }

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
