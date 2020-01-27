use irint3a::ir;
use irint3a::irbuilder::IRBuilder;

use crate::ast;
use crate::ast::AST;
use crate::astcast;
use crate::bindapp::BindApp;
use crate::bindfun::{BindFun, BindFunId};
use crate::bindvar::BindVarId;
use crate::nativedefs;

use std::collections::HashMap;

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

pub struct Translater<'a> {
    root: &'a ast::ASTExprPtr,
    app: &'a BindApp,
    builder: IRBuilder,

    ir_fun_names: HashMap<BindFunId, String>,

    //fiels abouts the current function being translated
    act_fun_node: Option<&'a ast::ASTDefFun>,
    act_fun_bind: Option<&'a BindFun>,
    act_fun_vars: HashMap<BindVarId, ir::RegId>,

    // fields to help build the IR, might be moved to IRBuilder later for more general usage
    reg_allocs: Vec<i32>,
    expr_val: Option<ExprVal>, //value of last expression. None when it's not computed
    next_label: usize,         //used to build unique label names for control flow
}

impl<'a> Translater<'a> {
    pub fn new(root: &'a ast::ASTExprPtr, app: &'a BindApp) -> Self {
        Translater {
            root,
            app,
            builder: IRBuilder::new(),

            ir_fun_names: HashMap::new(),

            act_fun_node: None,
            act_fun_bind: None,
            act_fun_vars: HashMap::new(),

            reg_allocs: Vec::new(),
            expr_val: None,
            next_label: 0,
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

        // 3) define IR names for all user functions, and the user main function
        let main_bind = self
            .app
            .get_fun_from_native_name(nativedefs::SPE_MAIN.name());
        self.ir_fun_names
            .insert(main_bind.id(), "_f1_main".to_string());
        for (fn_idx, def) in fun_defs.iter().enumerate() {
            let ir_name = format!("_f{}_{}", fn_idx + 2, def.name());
            let bind_id = self.app.get_fun_from_ast(def.get_uid()).id();
            self.ir_fun_names.insert(bind_id, ir_name);
        }

        // 4) Generate code for the user main function
        self.act_fun_node = None;
        self.act_fun_bind = Some(main_bind);
        self.tl_fun(&self.root);

        // 5) Generate code for all other user functions
        for def in fun_defs {
            let fn_bind = self.app.get_fun_from_ast(def.get_uid());
            self.act_fun_node = Some(def);
            self.act_fun_bind = Some(fn_bind);
            self.tl_fun(def.body());
        }

        self.builder.build()
    }

    fn tl_fun(&mut self, body: &ast::ASTExprPtr) {
        let fn_bind = self.act_fun_bind.unwrap();
        let fn_name = self.ir_fun_names.get(&fn_bind.id()).unwrap();
        let nb_args = fn_bind.count_args();
        /*
            println!(
                "FN {} ({:?}): {} args, {} allocs",
                fn_name,
                fn_bind.ty(),
                nb_args,
                fn_bind.count_variables(),
            );
        */

        // 1) Begin function
        self.builder.begin_function(Some(fn_name), None);

        // 2) alloc registers (temporary), only used to contain args values
        let mut args_regs = vec![];
        for _ in 0..nb_args {
            args_regs.push(self.alloc_reg());
        }

        // 3) alloc memory to store all variables (argument + locals)
        let mut args_mem = vec![];
        for var_bind in fn_bind.vars() {
            let reg_id = self.alloc_reg();
            self.builder.ins_alloca(reg_id, None);
            args_mem.push(reg_id);
            self.act_fun_vars.insert(var_bind.id(), reg_id);
        }

        // 4) store all arguments to memory
        for i in 0..nb_args {
            self.builder.ins_store(args_mem[i], args_regs[i], None);
        }
        drop(args_mem);

        // 5) Free argument registers
        for arg_reg in args_regs {
            self.free_reg(arg_reg);
        }

        // 6) Generate body function code and return instruction
        match self.tl_expr(&**body) {
            ExprVal::TmpReg(res_reg) => {
                self.builder.ins_ret(res_reg, None);
                self.free_reg(res_reg);
            }

            ExprVal::NoVal => self.builder.ins_ret(ir::RegId(0), None),
        }

        // 7) Clear variable registers
        let mut fun_vars = HashMap::new();
        std::mem::swap(&mut fun_vars, &mut self.act_fun_vars);
        for (_, reg_id) in fun_vars {
            self.free_reg(reg_id);
        }
        assert!(self.count_alloc_regs() == 0);

        // 8) clear stored infos for current function
        self.act_fun_node = None;
        self.act_fun_bind = None;
        self.next_label = 0;

        // 9) Finish function
        self.builder.end_function();
    }

    fn add_standard_fn(&mut self, name: &str, ir_addr: ir::FunAddress) {
        let bind_id = self.app.get_fun_from_native_name("putc").id();
        let ir_name = format!("_std_{}", name);
        self.builder.add_extern_fun(Some(&ir_name), ir_addr);
        self.ir_fun_names.insert(bind_id, ir_name);
    }

    fn add_native_defs(&mut self) {
        self.add_standard_fn("putc", ir::FunAddress(257));
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

    fn get_unique_label(&mut self, prefix: &str) -> String {
        self.next_label += 1;
        format!("{}_{}", prefix, self.next_label)
    }

    fn gen_nop(&mut self, label: &str) {
        self.builder
            .ins_movr(ir::RegId(0), ir::RegId(0), Some(label));
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
        res.expect("IRint3A Translater: Internal errror: no value set by visitor")
    }

    // translate classic calls: user functions and native standard lib
    fn tl_call(&mut self, node: &ast::ASTExprCall, dst_reg: ir::RegId, args: &Vec<ir::RegId>) {
        let fun_bind = self.act_fun_bind.unwrap();
        let callee_id = fun_bind.get_fun_of_exp_call(node).unwrap();
        let callee_ir_name = self.ir_fun_names.get(&callee_id).unwrap();
        self.builder
            .ins_call_name(dst_reg, callee_ir_name, args.clone(), None);
    }

    fn tl_call_binop(&mut self, op_name: &str, reg_dst: ir::RegId, args: &Vec<ir::RegId>) {
        assert!(args.len() == 2);
        let src1 = args[0];
        let src2 = args[1];

        match op_name {
            "add" => self.builder.ins_add(reg_dst, src1, src2, None),
            "sub" => self.builder.ins_sub(reg_dst, src1, src2, None),
            "mul" => self.builder.ins_mul(reg_dst, src1, src2, None),
            "div" => self.builder.ins_div(reg_dst, src1, src2, None),
            "mod" => self.builder.ins_mod(reg_dst, src1, src2, None),
            "eq" => self.builder.ins_cmpeq(reg_dst, src1, src2, None),
            "lt" => self.builder.ins_cmplt(reg_dst, src1, src2, None),
            "gt" => self.builder.ins_cmpgt(reg_dst, src1, src2, None),
            _ => unreachable!(),
        }
    }

    fn tl_call_unop(&mut self, op_name: &str, reg_dst: ir::RegId, args: &Vec<ir::RegId>) {
        assert!(args.len() == 1);
        let src = args[0];
        let reg_0 = self.alloc_reg();
        self.builder.ins_movi(reg_0, 0, None);

        match op_name {
            // -x => 0 - x
            "neg" => self.builder.ins_sub(reg_dst, reg_0, src, None),

            // !x => x == 0
            "not" => self.builder.ins_cmpeq(reg_dst, src, reg_0, None),

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
        self.expr_val = Some(ExprVal::NoVal);

        assert!(node.args().len() == 2);
        let fun_bind = self.act_fun_bind.unwrap();
        let ast_var = astcast::cast_to_expr_id(&*node.args()[0]).unwrap();
        let ast_val = &*node.args()[1];
        let var_id = fun_bind.get_var_of_exp_id(ast_var).unwrap();
        let var_reg = *self.act_fun_vars.get(&var_id).unwrap();

        let val_reg = match self.tl_expr(ast_val) {
            ExprVal::TmpReg(reg) => reg,
            ExprVal::NoVal => panic!("Internal error for @op:set: the value is none"),
        };

        self.builder.ins_store(var_reg, val_reg, None);
        self.free_reg(val_reg);
        self.expr_val = Some(ExprVal::NoVal);
    }
}

impl<'a> ast::ASTVisitor for Translater<'a> {
    fn visit_def_arg(&mut self, _node: &ast::ASTDefArg) {
        unreachable!();
    }

    fn visit_def_fun(&mut self, _node: &ast::ASTDefFun) {
        // nothing to do
    }

    fn visit_def_var(&mut self, node: &ast::ASTDefVar) {
        // translate variable assignment
        let fun_bind = self.act_fun_bind.unwrap();
        let var_bind = fun_bind.get_var_from_ast(node.get_uid());
        let var_reg = *self.act_fun_vars.get(&var_bind.id()).unwrap();

        let init_reg = match self.tl_expr(&**node.init()) {
            ExprVal::TmpReg(reg) => reg,
            ExprVal::NoVal => panic!(
                "irint3a translater internal error for var_def {}: init has no value",
                node.name()
            ),
        };

        self.builder.ins_store(var_reg, init_reg, None);
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
                _ => panic!(
                    "Translater internal error: expr_call {}: argument has no value",
                    node.name()
                ),
            })
            .collect();

        // 2) Translate call
        let dst_reg = self.alloc_reg();
        if node.name().starts_with("@op") {
            self.tl_call_special(node, dst_reg, &args_regs);
        } else {
            self.tl_call(node, dst_reg, &args_regs);
        }

        // 3) Clear arguments
        for arg in args_regs {
            self.free_reg(arg);
        }

        // 4) Set return value
        self.expr_val = Some(ExprVal::TmpReg(dst_reg));
    }

    fn visit_expr_const(&mut self, node: &ast::ASTExprConst) {
        let val = node.val();
        let reg = self.alloc_reg();
        self.builder.ins_movi(reg, val, None);
        self.expr_val = Some(ExprVal::TmpReg(reg));
    }

    fn visit_expr_id(&mut self, node: &ast::ASTExprId) {
        let fun_bind = self.act_fun_bind.unwrap();
        let var_id = fun_bind.get_var_of_exp_id(node).unwrap();
        let var_reg = *self.act_fun_vars.get(&var_id).unwrap();
        let dst_reg = self.alloc_reg();

        self.builder.ins_load(dst_reg, var_reg, None);
        self.expr_val = Some(ExprVal::TmpReg(dst_reg));
    }

    fn visit_expr_if(&mut self, node: &ast::ASTExprIf) {
        let fun_bind = self.act_fun_bind.unwrap();
        let is_void = fun_bind.get_type_of_exp(node).unwrap().is_void();

        // 1) generate the condition code with the conditional branching
        let cond_reg = match self.tl_expr(&**node.cond()) {
            ExprVal::TmpReg(reg) => reg,
            ExprVal::NoVal => panic!("Internal error: the condition of if should have a value"),
        };
        let dst_reg = if is_void {
            None
        } else {
            Some(self.alloc_reg())
        };
        let label_if = self.get_unique_label("Lif");
        let label_else = self.get_unique_label("Lelse");
        let label_end = self.get_unique_label("Lfi");
        self.builder.ins_br(cond_reg, &label_if, &label_else, None);
        self.free_reg(cond_reg);

        // 2) generate the if block with jump to the end
        self.gen_nop(&label_if);
        let val_if = self.tl_expr(&**node.val_if());
        if let Some(dst_reg) = dst_reg {
            let reg_if =
                val_if.get_reg("Internal error: if is non-void, so if block should have a value");
            self.builder.ins_movr(dst_reg, reg_if, None);
        }
        self.builder.ins_jump(&label_end, None);
        if let ExprVal::TmpReg(reg_if) = val_if {
            self.free_reg(reg_if);
        }

        // 3) generate the else block with jump to the end
        self.gen_nop(&label_else);
        let val_else = self.tl_expr(&**node.val_else());
        if let Some(dst_reg) = dst_reg {
            let reg_else = val_else
                .get_reg("Internal error: if is non-void, so else block should have a value");
            self.builder.ins_movr(dst_reg, reg_else, None);
        }
        self.builder.ins_jump(&label_end, None);
        if let ExprVal::TmpReg(reg_else) = val_else {
            self.free_reg(reg_else);
        }

        // 4) generate the end block to merge both parts
        self.gen_nop(&label_end);
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
        let cond_label = self.get_unique_label("Lcond");
        let body_label = self.get_unique_label("Lbody");
        let end_label = self.get_unique_label("Lend");

        // 1) End current block by jumping to the condition
        self.builder.ins_jump(&cond_label, None);

        // 2) Generate the condition block with the conditional branching
        self.gen_nop(&cond_label);
        let cond_reg = match self.tl_expr(&**node.cond()) {
            ExprVal::TmpReg(reg) => reg,
            ExprVal::NoVal => panic!("Internal error: the condition of while should have a value"),
        };
        self.builder.ins_br(cond_reg, &body_label, &end_label, None);
        self.free_reg(cond_reg);

        // 3) Generate the body block with jump to the condition
        self.gen_nop(&body_label);
        let val_body = self.tl_expr(&**node.body());
        self.builder.ins_jump(&cond_label, None);
        if let ExprVal::TmpReg(reg_body) = val_body {
            self.free_reg(reg_body);
        }

        // 4) Generate the end block after the loop
        self.gen_nop(&end_label);
        self.expr_val = Some(ExprVal::NoVal);
    }

    fn visit_type_name(&mut self, _node: &ast::ASTTypeName) {
        unreachable!();
    }
}
