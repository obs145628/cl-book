use std::collections::HashMap;

use irint3a::ir;
use irint3a::irbuilder::IRBuilder;
use irint3a::irnames;
use irint3a::irvalidation;

use crate::ast;
use crate::ast::AST;
use crate::astcast;
use crate::bindapp::BindApp;
use crate::bindfun::{BindFun, BindFunId};
use crate::bindvar::BindVarId;
use crate::nativedefs;
use crate::translater::defslist;

pub struct Translater<'a> {
    root: &'a ast::ASTExprPtr,
    app: &'a BindApp,
    module: ir::Module,
    names: irnames::ModuleNames,
    ir_funs: HashMap<BindFunId, ir::FunctionId>,
}

impl<'a> Translater<'a> {
    pub fn new(root: &'a ast::ASTExprPtr, app: &'a BindApp) -> Self {
        Translater {
            root,
            app,
            module: ir::Module::new(),
            names: irnames::ModuleNames::new(),
            ir_funs: HashMap::new(),
        }
    }

    pub fn translate(mut self) -> (ir::Module, irnames::ModuleNames) {
        // 1) Add native function definitions
        self.add_native_defs();

        // 2) List all user functions
        let fun_defs = defslist::list_fun_defs(&**self.root);

        // 3) define IR names for all user functions, and the user main function
        let main_bind = self
            .app
            .get_fun_from_native_name(nativedefs::SPE_MAIN.name());
        self.add_fun(main_bind.id(), "main_1".to_string());
        for (fn_idx, def) in fun_defs.iter().enumerate() {
            let fun_bind = self.app.get_fun_from_ast(def.get_uid());
            let fun_name = format!("{}_{}", def.name(), fn_idx + 2);
            self.add_fun(fun_bind.id(), fun_name);
        }

        // 4) Generate code for the user main function
        self.tl_fun(&self.root, main_bind);

        // 5) Generate code for all other user functions
        for def in fun_defs {
            let fun_bind = self.app.get_fun_from_ast(def.get_uid());
            self.tl_fun(def.body(), fun_bind);
        }

        // 6) Generate code for the start function
        self.gen_start_fun();

        // 7) Finish module
        irvalidation::validate_module(&self.module);
        (self.module, self.names)
    }

    fn tl_fun(&mut self, body: &ast::ASTExprPtr, fun_bind: &'a BindFun) {
        let fun_ir = *self.ir_funs.get(&fun_bind.id()).unwrap();
        let mut fun = self.module.get_fun_mut(fun_ir).unwrap();

        let builder = IRBuilder::new(&mut fun);
        let mut tl = FunctionTranslater::new(builder, fun_bind, &self.ir_funs);
        tl.run(body);
    }

    fn gen_start_fun(&mut self) {
        let main_bind = self
            .app
            .get_fun_from_native_name(nativedefs::SPE_MAIN.name());
        let main_id = *self.ir_funs.get(&main_bind.id()).unwrap();
        let start_id = ir::FunctionId(0);
        let exit_id = ir::FunctionId(258);

        // 1) Create and initialize function
        self.module.create_function(Some(start_id));
        self.names.add_function(start_id, "_start".to_string());
        let fun = self.module.get_fun_mut(start_id).unwrap();
        let mut builder = IRBuilder::new(fun);
        let bb_entry = builder.create_basic_block();
        builder.set_insert_point(bb_entry);

        // 2) Call user main function
        builder.ins_call(ir::RegId(0), main_id, vec![]);

        // 3) Call extern exit function with argument 0
        builder.ins_movi(ir::RegId(0), 0);
        builder.ins_call(ir::RegId(0), exit_id, vec![ir::RegId(0)]);

        // 4) Finish function
        builder.ins_ret(ir::RegId(0));
    }

    fn add_fun(&mut self, bind_id: BindFunId, name: String) {
        let fun_ir = self.module.create_function(None);
        self.ir_funs.insert(bind_id, fun_ir);
        self.names.add_function(fun_ir, name);
    }

    fn add_standard_fn(&mut self, name: &str, id: ir::FunctionId) {
        let bind_id = self.app.get_fun_from_native_name(name).id();
        self.module.create_extern_function(id);
        self.ir_funs.insert(bind_id, id);
        self.names.add_function(id, name.to_string());
    }

    fn add_native_defs(&mut self) {
        self.add_standard_fn("putc", ir::FunctionId(257));
        self.add_standard_fn("exit", ir::FunctionId(258));
        self.add_standard_fn("getc", ir::FunctionId(259));
        self.add_standard_fn("fmemget", ir::FunctionId(260));
        self.add_standard_fn("fmemset", ir::FunctionId(261));
        self.add_standard_fn("fmemcpy", ir::FunctionId(262));
    }
}

enum ExprVal {
    NoVal,             //the expression return type is void
    TmpReg(ir::RegId), //value stored in register
}

impl ExprVal {
    fn get_reg(&self, err_mess: &str) -> ir::RegId {
        match self {
            ExprVal::TmpReg(reg) => *reg,
            ExprVal::NoVal => panic!("{}", err_mess),
        }
    }
}

struct FunctionTranslater<'a> {
    builder: IRBuilder<'a>,
    ir_funs: &'a HashMap<BindFunId, ir::FunctionId>,

    //fiels abouts the current function being translated
    fun_bind: &'a BindFun,
    fun_vars: HashMap<BindVarId, ir::RegId>,

    // fields to help build the IR, might be moved to IRBuilder later for more general usage
    reg_allocs: Vec<i32>,
    expr_val: Option<ExprVal>, //value of last expression. None when it's not computed
}

impl<'a> FunctionTranslater<'a> {
    pub fn new(
        builder: IRBuilder<'a>,
        fun_bind: &'a BindFun,
        ir_funs: &'a HashMap<BindFunId, ir::FunctionId>,
    ) -> Self {
        FunctionTranslater {
            builder,
            ir_funs,

            fun_bind,
            fun_vars: HashMap::new(),

            reg_allocs: vec![],
            expr_val: None,
        }
    }

    fn run(&mut self, body: &ast::ASTExprPtr) {
        let nb_args = self.fun_bind.count_args();
        /*
            println!(
                "FN {} ({:?}): {} args, {} allocs",
                self.builder.actual_function().0,
                self.fun_bind.ty(),
                nb_args,
                self.fun_bind.count_variables(),
            );
        */

        // 1) Begin function
        let bb_entry = self.builder.create_basic_block();
        self.builder.set_insert_point(bb_entry);

        // 2) alloc registers (temporary), only used to contain args values
        let mut args_regs = vec![];
        for _ in 0..nb_args {
            args_regs.push(self.alloc_reg());
        }

        // 3) alloc memory to store all variables (argument + locals)
        let mut args_mem = vec![];
        for var_bind in self.fun_bind.vars() {
            let reg_id = self.alloc_reg();
            self.builder.ins_alloca(reg_id);
            args_mem.push(reg_id);
            self.fun_vars.insert(var_bind.id(), reg_id);
        }

        // 4) store all arguments to memory
        for i in 0..nb_args {
            self.builder.ins_store(args_mem[i], args_regs[i]);
        }
        drop(args_mem);

        // 5) Free argument registers
        for arg_reg in args_regs {
            self.free_reg(arg_reg);
        }

        // 6) Generate body function code and return instruction
        match self.tl_expr(&**body) {
            ExprVal::TmpReg(res_reg) => {
                self.builder.ins_ret(res_reg);
                self.free_reg(res_reg);
            }

            ExprVal::NoVal => self.builder.ins_ret(ir::RegId(0)),
        }

        // 7) Clear variable registers
        let mut fun_vars = HashMap::new();
        std::mem::swap(&mut fun_vars, &mut self.fun_vars);
        for (_, reg_id) in fun_vars {
            self.free_reg(reg_id);
        }
        assert!(self.count_alloc_regs() == 0);
    }

    fn alloc_reg(&mut self) -> ir::RegId {
        let mut reg_id = 0;
        while reg_id < self.reg_allocs.len() && self.reg_allocs[reg_id] == 1 {
            reg_id += 1;
        }

        if reg_id == self.reg_allocs.len() {
            let new_elems = std::cmp::max(4, self.reg_allocs.len());
            for _ in 0..new_elems {
                self.reg_allocs.push(0);
            }
        }

        assert!(self.reg_allocs[reg_id] == 0);
        self.reg_allocs[reg_id] = 1;
        ir::RegId(reg_id)
    }

    fn free_reg(&mut self, reg: ir::RegId) {
        let reg = reg.0;
        assert!(self.reg_allocs[reg] == 1);
        self.reg_allocs[reg] = 0;
    }

    fn count_alloc_regs(&self) -> usize {
        let mut res = 0;
        for alloc in &self.reg_allocs {
            res += alloc;
        }
        res as usize
    }

    fn tl_expr(&mut self, node: &dyn ast::ASTExpr) -> ExprVal {
        self.expr_val = None;

        let mut res = None;
        node.accept(self);
        std::mem::swap(&mut res, &mut self.expr_val);
        let res = res.expect("IRint3A Translater: Internal errror: no value set by visitor");

        let is_void = self.fun_bind.get_type_of_exp(node).unwrap().is_void();
        match &res {
            ExprVal::NoVal => assert!(is_void),
            ExprVal::TmpReg(_) => assert!(!is_void),
        }

        res
    }

    // translate classic calls: user functions and native standard lib
    fn tl_call(&mut self, node: &ast::ASTExprCall, dst_reg: ir::RegId, args: &Vec<ir::RegId>) {
        let callee_bind = self.fun_bind.get_fun_of_exp_call(node).unwrap();
        let callee_id = *self.ir_funs.get(&callee_bind).unwrap();
        self.builder.ins_call(dst_reg, callee_id, args.clone());
    }

    fn tl_call_binop(&mut self, op_name: &str, reg_dst: ir::RegId, args: &Vec<ir::RegId>) {
        assert!(args.len() == 2);
        let src1 = args[0];
        let src2 = args[1];

        match op_name {
            "add" => self.builder.ins_add(reg_dst, src1, src2),
            "sub" => self.builder.ins_sub(reg_dst, src1, src2),
            "mul" => self.builder.ins_mul(reg_dst, src1, src2),
            "div" => self.builder.ins_div(reg_dst, src1, src2),
            "mod" => self.builder.ins_mod(reg_dst, src1, src2),
            "eq" => self.builder.ins_cmpeq(reg_dst, src1, src2),
            "lt" => self.builder.ins_cmplt(reg_dst, src1, src2),
            "gt" => self.builder.ins_cmpgt(reg_dst, src1, src2),
            _ => unreachable!(),
        }
    }

    fn tl_call_unop(&mut self, op_name: &str, reg_dst: ir::RegId, args: &Vec<ir::RegId>) {
        assert!(args.len() == 1);
        let src = args[0];
        let reg_0 = self.alloc_reg();
        self.builder.ins_movi(reg_0, 0);

        match op_name {
            // -x => 0 - x
            "neg" => self.builder.ins_sub(reg_dst, reg_0, src),

            // !x => x == 0
            "not" => self.builder.ins_cmpeq(reg_dst, src, reg_0),

            _ => unreachable!(),
        }

        self.free_reg(reg_0);
    }

    // translate special calls: operators
    fn tl_call_special(
        &mut self,
        node: &ast::ASTExprCall,
        reg_dst: ir::RegId,
        args: &Vec<ir::RegId>,
    ) {
        let op_name = &node.name()[4..];
        match op_name {
            "add" | "sub" | "mul" | "div" | "mod" | "eq" | "lt" | "gt" => {
                self.tl_call_binop(op_name, reg_dst, args)
            }
            "neg" | "not" => self.tl_call_unop(op_name, reg_dst, args),
            _ => unreachable!(),
        }
    }

    //translate call to set operator
    fn tl_call_set(&mut self, node: &ast::ASTExprCall) {
        assert!(node.args().len() == 2);
        let ast_var = astcast::cast_to_expr_id(&*node.args()[0]).unwrap();
        let ast_val = &*node.args()[1];
        let var_id = self.fun_bind.get_var_of_exp_id(ast_var).unwrap();
        let var_reg = *self.fun_vars.get(&var_id).unwrap();

        let val_reg = match self.tl_expr(ast_val) {
            ExprVal::TmpReg(reg) => reg,
            _ => unreachable!(),
        };

        self.builder.ins_store(var_reg, val_reg);
        self.free_reg(val_reg);
        self.expr_val = Some(ExprVal::NoVal);
    }
}

impl<'a> ast::ASTVisitor for FunctionTranslater<'a> {
    fn visit_def_arg(&mut self, _node: &ast::ASTDefArg) {
        unreachable!();
    }

    fn visit_def_fun(&mut self, _node: &ast::ASTDefFun) {
        // nothing to do
    }

    fn visit_def_var(&mut self, node: &ast::ASTDefVar) {
        // translate variable assignment
        let var_bind = self.fun_bind.get_var_from_ast(node.get_uid());
        let var_reg = *self.fun_vars.get(&var_bind.id()).unwrap();

        let init_reg = match self.tl_expr(&**node.init()) {
            ExprVal::TmpReg(reg) => reg,
            _ => unreachable!(),
        };

        self.builder.ins_store(var_reg, init_reg);
        self.free_reg(init_reg);
    }

    fn visit_expr_block(&mut self, node: &ast::ASTExprBlock) {
        let mut res = ExprVal::NoVal;
        for (idx, exp) in node.exprs().iter().enumerate() {
            res = self.tl_expr(&**exp);
            if idx + 1 < node.exprs().len() {
                //If not last val, discard unused value
                if let ExprVal::TmpReg(tmp_reg) = &res {
                    self.free_reg(*tmp_reg);
                }
            }
        }

        self.expr_val = Some(res);
    }

    fn visit_expr_call(&mut self, node: &ast::ASTExprCall) {
        if node.name() == "@op:set" {
            //special case for set operator, cannot simply compute operands
            self.tl_call_set(node);
            return;
        }

        // 1) Translate arguments
        let args_vals: Vec<ExprVal> = node.args().iter().map(|arg| self.tl_expr(&**arg)).collect();
        let args_regs: Vec<ir::RegId> = args_vals
            .into_iter()
            .map(|arg| match arg {
                ExprVal::TmpReg(reg_id) => reg_id,
                _ => unreachable!(),
            })
            .collect();

        // 2) Translate call
        let is_void = self.fun_bind.get_type_of_exp(node).unwrap().is_void();
        let dst_reg = self.alloc_reg();
        if node.name().starts_with("@op") {
            self.tl_call_special(node, dst_reg, &args_regs);
        } else {
            self.tl_call(node, dst_reg, &args_regs);
        }

        // 3) Clear arguments and return value
        for arg in args_regs {
            self.free_reg(arg);
        }
        if is_void {
            self.free_reg(dst_reg);
        }

        // 4) Set return value
        self.expr_val = if is_void {
            Some(ExprVal::NoVal)
        } else {
            Some(ExprVal::TmpReg(dst_reg))
        };
    }

    fn visit_expr_const(&mut self, node: &ast::ASTExprConst) {
        let val = node.val();
        let reg = self.alloc_reg();
        self.builder.ins_movi(reg, val);
        self.expr_val = Some(ExprVal::TmpReg(reg));
    }

    fn visit_expr_id(&mut self, node: &ast::ASTExprId) {
        let var_id = self.fun_bind.get_var_of_exp_id(node).unwrap();
        let var_reg = *self.fun_vars.get(&var_id).unwrap();
        let dst_reg = self.alloc_reg();

        self.builder.ins_load(dst_reg, var_reg);
        self.expr_val = Some(ExprVal::TmpReg(dst_reg));
    }

    fn visit_expr_if(&mut self, node: &ast::ASTExprIf) {
        let is_void = self.fun_bind.get_type_of_exp(node).unwrap().is_void();

        // 1) Create the basic blocks
        let bb_if = self.builder.create_basic_block();
        let bb_else = self.builder.create_basic_block();
        let bb_end = self.builder.create_basic_block();

        // 2) generate the condition code with the conditional branching
        let cond_reg = match self.tl_expr(&**node.cond()) {
            ExprVal::TmpReg(reg) => reg,
            _ => unreachable!(),
        };
        let dst_reg = if is_void {
            None
        } else {
            Some(self.alloc_reg())
        };
        self.builder.ins_br(cond_reg, bb_if, bb_else);
        self.free_reg(cond_reg);

        // 3) generate the if block with jump to the end
        self.builder.set_insert_point(bb_if);
        let val_if = self.tl_expr(&**node.val_if());
        if let Some(dst_reg) = dst_reg {
            let reg_if =
                val_if.get_reg("Internal error: if is non-void, so if block should have a value");
            self.builder.ins_movr(dst_reg, reg_if);
            self.free_reg(reg_if);
        }
        self.builder.ins_jump(bb_end);

        // 4) generate the else block with jump to the end
        self.builder.set_insert_point(bb_else);
        let val_else = self.tl_expr(&**node.val_else());
        if let Some(dst_reg) = dst_reg {
            let reg_else = val_else
                .get_reg("Internal error: if is non-void, so else block should have a value");
            self.builder.ins_movr(dst_reg, reg_else);
            self.free_reg(reg_else);
        }
        self.builder.ins_jump(bb_end);

        // 5) generate the end block to merge both parts
        self.builder.set_insert_point(bb_end);
        self.expr_val = match dst_reg {
            Some(reg) => Some(ExprVal::TmpReg(reg)),
            None => Some(ExprVal::NoVal),
        };
    }

    fn visit_expr_let(&mut self, node: &ast::ASTExprLet) {
        for def in node.defs() {
            def.accept(self);
        }

        self.expr_val = Some(self.tl_expr(&**node.val()));
    }

    fn visit_expr_while(&mut self, node: &ast::ASTExprWhile) {
        self.expr_val = Some(ExprVal::NoVal);

        // 1) Create basic blocks
        let bb_cond = self.builder.create_basic_block();
        let bb_body = self.builder.create_basic_block();
        let bb_end = self.builder.create_basic_block();

        // 2) End current block by jumping to the condition
        self.builder.ins_jump(bb_cond);

        // 3) Generate the condition block with the conditional branching
        self.builder.set_insert_point(bb_cond);
        let cond_reg = match self.tl_expr(&**node.cond()) {
            ExprVal::TmpReg(reg) => reg,
            _ => unreachable!(),
        };
        self.builder.ins_br(cond_reg, bb_body, bb_end);
        self.free_reg(cond_reg);

        // 4) Generate the body block with jump to the condition
        self.builder.set_insert_point(bb_body);
        self.tl_expr(&**node.body());
        self.builder.ins_jump(bb_cond);

        // 5) Generate the end block after the loop
        self.builder.set_insert_point(bb_end);
        self.expr_val = Some(ExprVal::NoVal);
    }

    fn visit_type_name(&mut self, _node: &ast::ASTTypeName) {
        unreachable!();
    }
}
