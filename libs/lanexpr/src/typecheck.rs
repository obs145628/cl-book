use crate::ast::*;
use crate::bindbuilder::BindBuilder;
use crate::letype::{FnType, Type, TypeVal};

pub struct TypeCheck {
    res: Option<Type>,

    // when looking at deinfitions, only register the headers
    // used to first define all function proptotypes before looking at bodyes
    reg_headers: bool,

    builder: BindBuilder,
}

impl TypeCheck {
    pub fn new() -> TypeCheck {
        TypeCheck {
            res: None,
            reg_headers: false,

            builder: BindBuilder::new(),
        }
    }

    pub fn check(&mut self, node: &ASTExprPtr) {
        self.builder.begin();
        node.accept(self);
        self.builder.end();
    }

    fn get_exp_type(&mut self, node: &ASTExprPtr) -> Type {
        let fun = self.builder.actual_fn();
        let saved_ty = fun.get_type_of_exp(&**node);
        if !saved_ty.is_none() {
            return saved_ty.unwrap();
        }

        self.res = None;
        node.accept(self);
        let mut res: Option<Type> = None;
        std::mem::swap(&mut res, &mut self.res);
        let new_ty = res.unwrap();
        let fun = self.builder.actual_fn_mut();
        fun.set_type_of_exp(&**node, new_ty);
        new_ty
    }

    fn get_typename_type(&mut self, node: &ASTTypePtr) -> Type {
        let fun = self.builder.actual_fn();
        let saved_ty = fun.get_type_of_typename(&**node);
        if !saved_ty.is_none() {
            return saved_ty.unwrap();
        }

        self.res = None;
        node.accept(self);
        let mut res: Option<Type> = None;
        std::mem::swap(&mut res, &mut self.res);
        let new_ty = res.unwrap();
        let fun = self.builder.actual_fn_mut();
        fun.set_type_of_typename(&**node, new_ty);
        new_ty
    }

    fn check_fun_def(&mut self, node: &ASTDefFun) {
        let args: Vec<Type> = node
            .args()
            .iter()
            .map(|arg| self.get_typename_type(arg.ty()))
            .collect();
        let ret = self.get_typename_type(node.ret());

        let fn_type = FnType::new(args, ret);
        self.builder.add_fun_type(node, fn_type);
    }

    fn check_fun_body(&mut self, node: &ASTDefFun) {
        self.builder.open_fun_ast(node);
        let fn_ret_ty = {
            let fun = self.builder.actual_fn();
            *fun.ty().ret()
        };

        for arg in node.args() {
            let ty = self.get_typename_type(arg.ty());
            self.builder.add_arg_type(arg, ty);
        }

        let body_ty = self.get_exp_type(node.body());

        if !body_ty.can_be_cast_to(&fn_ret_ty) {
            panic!(
                "Invalid return type of function body: expected {:?}, got `{:?}",
                fn_ret_ty, body_ty
            );
        }

        self.builder.close_fun_ast();
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

        self.builder.add_var_type(node, var_ty);
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

        let fun = match self.builder.get_fun(node.name()) {
            Some(fun) => fun,
            None => panic!("Calling unknown function {}", node.name()),
        };
        let fun_id = fun.id();
        let fun_args = fun.ty().args();
        if fun_args.len() != args.len() {
            panic!(
                "Invalid fun call to {}, epexted {} arguments, got {}",
                node.name(),
                fun_args.len(),
                args.len()
            );
        }

        for i in 0..fun_args.len() {
            if !args[i].can_be_cast_to(&fun_args[i]) {
                panic!(
                    "Invalid fun call to {}, argument #{} is {:?}, expected {:?}",
                    node.name(),
                    i + 1,
                    args[i],
                    fun_args[i]
                );
            }
        }

        self.res = Some(*fun.ty().ret());
        let fun = self.builder.actual_fn_mut();
        fun.set_fun_of_exp_call(node, fun_id);
    }

    // => <int>
    fn visit_expr_const(&mut self, _: &ASTExprConst) {
        self.res = Some(Type::Val(TypeVal::Int));
    }

    // => typename(node.name())
    fn visit_expr_id(&mut self, node: &ASTExprId) {
        let var = match self.builder.get_var(node.name()) {
            Some(var) => var,
            None => panic!("Usage of undefined variable {}", node.name()),
        };
        let var_ty = var.ty();
        let var_id = var.id();

        match var_ty {
            Type::Val(vty) => self.res = Some(Type::Ref(vty)),
            _ => panic!(
                "Something wrong, cannot have an ExprId {} of type '{:?}'",
                node.name(),
                var_ty
            ),
        }

        let fun = self.builder.actual_fn_mut();
        fun.set_var_of_exp_id(node, var_id);
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
        self.builder.open_scope();

        self.reg_headers = true;
        for def in node.defs() {
            def.accept(self);
        }
        self.reg_headers = false;
        for def in node.defs() {
            def.accept(self);
        }

        let val_ty = self.get_exp_type(node.val());
        self.res = Some(val_ty);

        self.builder.close_scope();
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
        match self.builder.get_type(node.name()) {
            Some(ty) => self.res = Some(ty),
            None => panic!("unknown typename '{}'", node.name()),
        }
    }
}
