extern crate llvm_sys as llvm;

use llvm::prelude::*;

use std::collections::HashMap;
use std::ptr;

use crate::ast;
use crate::ast::AST;
use crate::astcast;
use crate::bindapp::BindApp;
use crate::bindfun::{BindFun, BindFunId};
use crate::bindvar::BindVarId;
use crate::letype::FnType;
use crate::nativedefs;
use crate::translater::defslist;

//build null-temrinated string
fn build_zstring(s: &str) -> Vec<u8> {
    let mut res = Vec::from(s.as_bytes());
    res.push(0);
    res
}

pub struct Translater<'a> {
    root: &'a ast::ASTExprPtr,
    app: &'a BindApp,

    //fields about the current function being translated
    act_fun_node: Option<&'a ast::ASTDefFun>,
    act_fun_bind: Option<&'a BindFun>,
    act_fun_vars: HashMap<BindVarId, LLVMValueRef>,

    //llvm objects
    llvm_ctx: *mut llvm::LLVMContext,
    llvm_mod: *mut llvm::LLVMModule,
    llvm_builder: *mut llvm::LLVMBuilder,
    node_val: Option<LLVMValueRef>,
    llvm_funs: HashMap<BindFunId, LLVMValueRef>,
}

impl<'a> Translater<'a> {
    pub fn new(root: &'a ast::ASTExprPtr, app: &'a BindApp) -> Self {
        Translater {
            root,
            app,

            act_fun_node: None,
            act_fun_bind: None,
            act_fun_vars: HashMap::new(),

            llvm_ctx: ptr::null_mut(),
            llvm_mod: ptr::null_mut(),
            llvm_builder: ptr::null_mut(),
            node_val: None,
            llvm_funs: HashMap::new(),
        }
    }

    /// Perform the translation of the whole program and output LLVM IR code to stdout
    /// Must be called only once
    pub fn translate(mut self) {
        // 1) Init translation
        self.init_llvm();

        // 2) Create native definitons
        self.add_native_defs();

        // 3) list and create all user functions (including the user main)
        let fun_defs = defslist::list_fun_defs(&**self.root);
        let main_bind = self
            .app
            .get_fun_from_native_name(nativedefs::SPE_MAIN.name());
        self.create_fun(main_bind.id(), "_f1_main");
        for (fn_idx, def) in fun_defs.iter().enumerate() {
            let fun_name = format!("_f{}_{}", fn_idx + 2, def.name());
            let bind_id = self.app.get_fun_from_ast(def.get_uid()).id();
            self.create_fun(bind_id, &fun_name);
        }

        // 4) Generate code for the user main function
        self.act_fun_node = None;
        self.act_fun_bind = Some(main_bind);
        self.tl_fun(&self.root);

        // 5) Generate code for all other functions
        for def in fun_defs {
            let fn_bind = self.app.get_fun_from_ast(def.get_uid());
            self.act_fun_node = Some(def);
            self.act_fun_bind = Some(fn_bind);
            self.tl_fun(def.body());
        }

        // 6) Dump generated IR
        self.save_llvm_ir();

        // 7) Finish translation
        self.clean_llvm();
    }

    fn add_native_defs(&mut self) {
        self.add_standard_fn("putc");
    }

    fn add_standard_fn(&mut self, name: &str) {
        let bind_id = self.app.get_fun_from_native_name(name).id();
        let fun_name = format!("_std_{}", name);
        self.create_fun(bind_id, &fun_name);
    }

    fn create_fun(&mut self, fun_id: BindFunId, fun_name: &str) {
        let zfun_name = build_zstring(fun_name);
        let fun_bind = self.app.get_fun(fun_id);
        let fun_ty = self.llvm_type_fun(fun_bind.ty());
        let fun = self.llvm_begin_function(&zfun_name, fun_ty);
        self.llvm_funs.insert(fun_id, fun);

        /*
            println!(
                "FN {} ({:?}): {} args, {} allocs",
                fun_name,
                fun_bind.ty(),
                fun_bind.count_args(),
                fun_bind.count_variables(),
            );
        */
    }

    fn tl_fun(&mut self, body: &ast::ASTExprPtr) {
        let fn_bind = self.act_fun_bind.unwrap();
        let nb_args = fn_bind.count_args();

        // 1) Init function
        //TODO: should clean this
        let fun = *self.llvm_funs.get_mut(&fn_bind.id()).unwrap();
        unsafe {
            let bb = llvm::core::LLVMAppendBasicBlockInContext(
                self.llvm_ctx,
                fun,
                b"entry\0".as_ptr() as *const _,
            );
            llvm::core::LLVMPositionBuilderAtEnd(self.llvm_builder, bb);
        }

        // 2) Allocate memory for all arguments and local variables
        let mut locals_addrs = vec![];
        for var_bind in fn_bind.vars() {
            let var_ty = self.llvm_type_i32();
            let addr = self.llvm_ins_alloca(var_ty);
            locals_addrs.push(addr);
            self.act_fun_vars.insert(var_bind.id(), addr);
        }

        // 3) Store all arguments values to memory
        for i in 0..nb_args {
            let arg_val = self.llvm_val_fun_arg(fun, i);
            self.llvm_ins_store(arg_val, locals_addrs[i]);
        }

        // 4) Gen function body and return instruction
        let body_val = self.tl_expr(&**body);
        if fn_bind.ty().ret().is_void() {
            self.llvm_ins_ret_void();
        } else {
            self.llvm_ins_ret(body_val);
        }
    }

    fn tl_expr(&mut self, node: &dyn ast::ASTExpr) -> LLVMValueRef {
        self.node_val = None;

        let mut res = None;
        node.accept(self);
        std::mem::swap(&mut res, &mut self.node_val);
        res.expect("Internal errror: no value set by visitor")
    }

    //translate call to set operator
    fn tl_call_set(&mut self, node: &ast::ASTExprCall) {
        assert!(node.args().len() == 2);
        let fun_bind = self.act_fun_bind.unwrap();
        let ast_var = astcast::cast_to_expr_id(&*node.args()[0]).unwrap();
        let ast_val = &*node.args()[1];
        let var_id = fun_bind.get_var_of_exp_id(ast_var).unwrap();

        let var_mem = *self.act_fun_vars.get(&var_id).unwrap();
        let new_val = self.tl_expr(ast_val);
        self.llvm_ins_store(new_val, var_mem);
        self.node_val = Some(self.llvm_val_const_i32(0));
    }

    fn tl_call(&mut self, node: &ast::ASTExprCall, args: &[LLVMValueRef]) -> LLVMValueRef {
        let fun_bind = self.act_fun_bind.unwrap();
        let callee_id = fun_bind.get_fun_of_exp_call(node).unwrap();
        let callee_fun = *self.llvm_funs.get(&callee_id).unwrap();
        self.llvm_ins_call(callee_fun, args)
    }

    fn icmp_zext(
        &mut self,
        op: llvm::LLVMIntPredicate,
        src1: LLVMValueRef,
        src2: LLVMValueRef,
    ) -> LLVMValueRef {
        let bval = self.llvm_ins_icmp(op, src1, src2);
        let ty = self.llvm_type_i32();
        self.llvm_ins_zext(bval, ty)
    }

    fn tl_call_binop(&mut self, op_name: &str, args: &[LLVMValueRef]) -> LLVMValueRef {
        assert!(args.len() == 2);
        let src1 = args[0];
        let src2 = args[1];

        match op_name {
            "add" => self.llvm_ins_add(src1, src2),
            "sub" => self.llvm_ins_sub(src1, src2),
            "mul" => self.llvm_ins_mul(src1, src2),
            "div" => self.llvm_ins_sdiv(src1, src2),
            "mod" => self.llvm_ins_srem(src1, src2),
            "eq" => self.icmp_zext(llvm::LLVMIntPredicate::LLVMIntEQ, src1, src2),
            "lt" => self.icmp_zext(llvm::LLVMIntPredicate::LLVMIntSLT, src1, src2),
            "gt" => self.icmp_zext(llvm::LLVMIntPredicate::LLVMIntSGT, src1, src2),
            _ => unreachable!(),
        }
    }

    fn tl_call_unop(&mut self, op_name: &str, args: &[LLVMValueRef]) -> LLVMValueRef {
        assert!(args.len() == 1);
        let src = args[0];
        let val0 = self.llvm_val_const_i32(0);

        match op_name {
            // -x => 0 - x
            "neg" => self.llvm_ins_sub(val0, src),

            // !x => x == 0
            "not" => self.icmp_zext(llvm::LLVMIntPredicate::LLVMIntEQ, src, val0),

            _ => unreachable!(),
        }
    }

    // translate special calls: operators
    fn tl_call_special(&mut self, node: &ast::ASTExprCall, args: &[LLVMValueRef]) -> LLVMValueRef {
        let op_name = &node.name()[4..];
        match op_name {
            "add" | "sub" | "mul" | "div" | "mod" | "eq" | "lt" | "gt" => {
                self.tl_call_binop(op_name, args)
            }
            "neg" | "not" => self.tl_call_unop(op_name, args),
            _ => unreachable!(),
        }
    }

    fn init_llvm(&mut self) {
        unsafe {
            self.llvm_ctx = llvm::core::LLVMContextCreate();
            self.llvm_mod = llvm::core::LLVMModuleCreateWithNameInContext(
                b"app\0".as_ptr() as *const _,
                self.llvm_ctx,
            );
            self.llvm_builder = llvm::core::LLVMCreateBuilderInContext(self.llvm_ctx);
        }
    }

    fn clean_llvm(&mut self) {
        unsafe {
            llvm::core::LLVMDisposeBuilder(self.llvm_builder);
            llvm::core::LLVMDisposeModule(self.llvm_mod);
            llvm::core::LLVMContextDispose(self.llvm_ctx);
        }
    }

    fn save_llvm_ir(&mut self) {
        unsafe {
            llvm::core::LLVMDumpModule(self.llvm_mod);
        }
    }

    fn llvm_type_void(&mut self) -> LLVMTypeRef {
        unsafe { llvm::core::LLVMVoidTypeInContext(self.llvm_ctx) }
    }

    fn llvm_type_i32(&mut self) -> LLVMTypeRef {
        unsafe { llvm::core::LLVMInt32TypeInContext(self.llvm_ctx) }
    }

    fn llvm_type_fun(&mut self, fun: &FnType) -> LLVMTypeRef {
        let mut args = vec![];
        for _ in 0..fun.args().len() {
            //all argument types are void
            args.push(self.llvm_type_i32());
        }
        let ret = if fun.ret().is_void() {
            self.llvm_type_void()
        } else {
            self.llvm_type_i32()
        };

        unsafe { llvm::core::LLVMFunctionType(ret, args.as_mut_ptr(), args.len() as u32, 0) }
    }

    fn llvm_begin_function(&mut self, name: &[u8], ty: LLVMTypeRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMAddFunction(self.llvm_mod, name.as_ptr() as *const _, ty) }
    }

    fn llvm_ins_ret(&mut self, val: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildRet(self.llvm_builder, val) }
    }

    fn llvm_ins_ret_void(&mut self) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildRetVoid(self.llvm_builder) }
    }

    fn llvm_ins_alloca(&mut self, ty: LLVMTypeRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildAlloca(self.llvm_builder, ty, b"\0".as_ptr() as *const _) }
    }

    fn llvm_ins_store(&mut self, src_val: LLVMValueRef, dst_ptr: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildStore(self.llvm_builder, src_val, dst_ptr) }
    }

    fn llvm_ins_load(&mut self, src_ptr: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildLoad(self.llvm_builder, src_ptr, b"\0".as_ptr() as *const _) }
    }

    fn llvm_ins_call(&mut self, fun: LLVMValueRef, args: &[LLVMValueRef]) -> LLVMValueRef {
        let mut args = Vec::from(args);
        unsafe {
            llvm::core::LLVMBuildCall(
                self.llvm_builder,
                fun,
                args.as_mut_ptr(),
                args.len() as u32,
                b"\0".as_ptr() as *const _,
            )
        }
    }

    fn llvm_ins_add(&mut self, left: LLVMValueRef, right: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            llvm::core::LLVMBuildAdd(self.llvm_builder, left, right, b"\0".as_ptr() as *const _)
        }
    }

    fn llvm_ins_sub(&mut self, left: LLVMValueRef, right: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            llvm::core::LLVMBuildSub(self.llvm_builder, left, right, b"\0".as_ptr() as *const _)
        }
    }

    fn llvm_ins_mul(&mut self, left: LLVMValueRef, right: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            llvm::core::LLVMBuildMul(self.llvm_builder, left, right, b"\0".as_ptr() as *const _)
        }
    }

    fn llvm_ins_sdiv(&mut self, left: LLVMValueRef, right: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            llvm::core::LLVMBuildSDiv(self.llvm_builder, left, right, b"\0".as_ptr() as *const _)
        }
    }

    fn llvm_ins_srem(&mut self, left: LLVMValueRef, right: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            llvm::core::LLVMBuildSRem(self.llvm_builder, left, right, b"\0".as_ptr() as *const _)
        }
    }

    fn llvm_ins_icmp(
        &mut self,
        op: llvm::LLVMIntPredicate,
        left: LLVMValueRef,
        right: LLVMValueRef,
    ) -> LLVMValueRef {
        unsafe {
            llvm::core::LLVMBuildICmp(
                self.llvm_builder,
                op,
                left,
                right,
                b"\0".as_ptr() as *const _,
            )
        }
    }

    fn llvm_ins_zext(&mut self, val: LLVMValueRef, ty: LLVMTypeRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildZExt(self.llvm_builder, val, ty, b"\0".as_ptr() as *const _) }
    }

    fn llvm_val_const_i32(&mut self, val: i32) -> LLVMValueRef {
        let ty = self.llvm_type_i32();
        unsafe { llvm::core::LLVMConstInt(ty, val as u64, 0) }
    }

    fn llvm_val_fun_arg(&mut self, fun: LLVMValueRef, idx: usize) -> LLVMValueRef {
        unsafe { llvm::core::LLVMGetParam(fun, idx as u32) }
    }
}

impl<'a> ast::ASTVisitor for Translater<'a> {
    fn visit_def_arg(&mut self, _node: &ast::ASTDefArg) {
        unreachable!();
    }

    fn visit_def_fun(&mut self, _node: &ast::ASTDefFun) {
        //nothing to do
    }

    fn visit_def_var(&mut self, node: &ast::ASTDefVar) {
        // translate variable assignment
        let fun_bind = self.act_fun_bind.unwrap();
        let var_bind = fun_bind.get_var_from_ast(node.get_uid());
        let var_mem = *self.act_fun_vars.get(&var_bind.id()).unwrap();
        let val_init = self.tl_expr(&**node.init());
        self.llvm_ins_store(val_init, var_mem);
    }

    fn visit_expr_block(&mut self, node: &ast::ASTExprBlock) {
        let mut res = self.llvm_val_const_i32(0);
        for exp in node.exprs() {
            res = self.tl_expr(&**exp);
        }

        self.node_val = Some(res);
    }

    fn visit_expr_call(&mut self, node: &ast::ASTExprCall) {
        //TODO

        if node.name() == "@op:set" {
            //special case for set operator, cannot simply compute operands
            self.tl_call_set(node);
            return;
        }

        let args_vals: Vec<_> = node.args().iter().map(|arg| self.tl_expr(&**arg)).collect();
        let ret_val = if node.name().starts_with("@op") {
            self.tl_call_special(node, &args_vals)
        } else {
            self.tl_call(node, &args_vals)
        };
        self.node_val = Some(ret_val);
    }

    fn visit_expr_const(&mut self, node: &ast::ASTExprConst) {
        self.node_val = Some(self.llvm_val_const_i32(node.val()));
    }

    fn visit_expr_id(&mut self, node: &ast::ASTExprId) {
        let fun_bind = self.act_fun_bind.unwrap();
        let var_id = fun_bind.get_var_of_exp_id(node).unwrap();
        let var_mem = *self.act_fun_vars.get(&var_id).unwrap();
        self.node_val = Some(self.llvm_ins_load(var_mem));
    }

    fn visit_expr_if(&mut self, node: &ast::ASTExprIf) {
        //TODO
        let val_cond = self.tl_expr(&**node.cond());
        let val_if = self.tl_expr(&**node.val_if());
        let val_else = self.tl_expr(&**node.val_else());
        self.node_val = Some(self.llvm_val_const_i32(0));
    }

    fn visit_expr_let(&mut self, node: &ast::ASTExprLet) {
        for def in node.defs() {
            def.accept(self);
        }

        self.node_val = Some(self.tl_expr(&**node.val()));
    }

    fn visit_expr_while(&mut self, node: &ast::ASTExprWhile) {
        //TODO
        let val_cond = self.tl_expr(&**node.cond());
        let val_body = self.tl_expr(&**node.body());
        self.node_val = Some(self.llvm_val_const_i32(0));
    }

    fn visit_type_name(&mut self, _node: &ast::ASTTypeName) {
        unreachable!();
    }
}
