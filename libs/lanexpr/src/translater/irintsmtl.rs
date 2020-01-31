use std::collections::HashMap;

use irintsm::ir;
use irintsm::irbuilder::IRBuilder;

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
    builder: IRBuilder,
    ir_funs: HashMap<BindFunId, ir::FunctionRef>,

    //fiels abouts the current function being translated
    act_fun_node: Option<&'a ast::ASTDefFun>,
    act_fun_bind: Option<&'a BindFun>,
    act_fun_vars: HashMap<BindVarId, ir::LocalsIndex>,
}

impl<'a> Translater<'a> {
    pub fn new(root: &'a ast::ASTExprPtr, app: &'a BindApp) -> Self {
        Translater {
            root,
            app,
            builder: IRBuilder::new(),
            ir_funs: HashMap::new(),

            act_fun_node: None,
            act_fun_bind: None,
            act_fun_vars: HashMap::new(),
        }
    }

    pub fn translate(mut self) -> ir::Module {
        // 1) Add native function definitions
        self.add_native_defs();

        // 2) List all user functions
        let fun_defs = defslist::list_fun_defs(&**self.root);

        // 3) create all IR functions for all user functions, and the user main function
        let main_fun = self.builder.create_function(None);
        let main_bind = self
            .app
            .get_fun_from_native_name(nativedefs::SPE_MAIN.name());
        self.ir_funs.insert(main_bind.id(), main_fun);
        for def in &fun_defs {
            let bind_id = self.app.get_fun_from_ast(def.get_uid()).id();
            let fun = self.builder.create_function(None);
            self.ir_funs.insert(bind_id, fun);
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

        //6) Generate code for the start function
        self.gen_start_fun();

        self.builder.finish()
    }

    fn tl_fun(&mut self, body: &ast::ASTExprPtr) {
        let fun_bind = self.act_fun_bind.unwrap();
        let fun = *self.ir_funs.get(&fun_bind.id()).unwrap();
        /*
            println!(
                "FN {} ({:?}): {} args, {} allocs",
                self.act_fun_node.name(),
                fun_bind.ty(),
                nb_args,
                fun_bind.count_variables(),
            );
        */

        // 1) Begin function
        let bb_entry = self.builder.create_basic_block(fun);
        self.builder.set_insert_point(bb_entry);
        self.builder.reset_operands_count();

        // 2) Register locals (arguments + variables)
        for (local_idx, var_bind) in fun_bind.vars().iter().enumerate() {
            self.act_fun_vars
                .insert(var_bind.id(), ir::LocalsIndex::new(local_idx));
        }

        // 3) Generate body function code and return instruction
        let is_void = self.tl_expr(&**body);
        if is_void {
            self.builder.ins_const(0);
        }
        self.builder.ins_ret();

        // 4) clear stored infos for current function
        assert!(self.builder.get_operands_count() == 0);
        self.act_fun_node = None;
        self.act_fun_bind = None;
        self.act_fun_vars.clear();
    }

    fn gen_start_fun(&mut self) {
        // 1) Get user main function
        let main_bind = self
            .app
            .get_fun_from_native_name(nativedefs::SPE_MAIN.name());
        let main_fun = *self.ir_funs.get(&main_bind.id()).unwrap();

        // 2) Create start function
        let start_fun = self.builder.create_function(Some(ir::FunctionRef::new(0)));
        let start_bb = self.builder.create_basic_block(start_fun);
        self.builder.set_insert_point(start_bb);

        // 3) Call user main function
        self.builder.ins_call(main_fun, 0);

        // 4) Call extern exit function (258) with argument 0
        self.builder.ins_const(0);
        self.builder.ins_call(ir::FunctionRef::new(258), 1);

        // 2) Finish function
        self.builder.ins_ret();
    }

    fn add_standard_fn(&mut self, name: &str, ir_addr: ir::FunctionRef) {
        let bind_id = self.app.get_fun_from_native_name(name).id();
        let fun = self.builder.create_function(Some(ir_addr));
        self.ir_funs.insert(bind_id, fun);
    }

    fn add_native_defs(&mut self) {
        self.add_standard_fn("putc", ir::FunctionRef::new(257));
        self.builder
            .create_function(Some(ir::FunctionRef::new(258))); //exit
    }

    fn tl_expr(&mut self, node: &dyn ast::ASTExpr) -> bool {
        let fun_bind = self.act_fun_bind.unwrap();
        let is_void = fun_bind.get_type_of_exp(node).unwrap().is_void();

        let old_count = self.builder.get_operands_count();
        node.accept(self);
        let exp_count = old_count + (!is_void as i32);
        assert!(self.builder.get_operands_count() == exp_count);
        is_void
    }

    // translate classic calls: user functions and native standard lib
    fn tl_call(&mut self, node: &ast::ASTExprCall) {
        let fun_bind = self.act_fun_bind.unwrap();
        let callee_id = fun_bind.get_fun_of_exp_call(node).unwrap();
        let callee_fun = *self.ir_funs.get(&callee_id).unwrap();
        self.builder.ins_call(callee_fun, node.args().len());

        let is_void = fun_bind.get_type_of_exp(node).unwrap().is_void();
        if is_void {
            //a function always return a value
            self.builder.ins_pop();
        }
    }

    // translate special calls:
    // operators (except and neg)
    fn tl_call_special(&mut self, op_name: &str) {
        match op_name {
            "add" => self.builder.ins_add(),
            "sub" => self.builder.ins_sub(),
            "mul" => self.builder.ins_mul(),
            "div" => self.builder.ins_div(),
            "mod" => self.builder.ins_rem(),
            "eq" => self.builder.ins_cmpeq(),
            "lt" => self.builder.ins_cmplt(),
            "gt" => self.builder.ins_cmpgt(),
            "not" => {
                self.builder.ins_const(0);
                self.builder.ins_cmpeq();
            }
            _ => unreachable!(),
        }
    }

    //translate call to set operator
    fn tl_call_set(&mut self, node: &ast::ASTExprCall) {
        assert!(node.args().len() == 2);
        let fun_bind = self.act_fun_bind.unwrap();
        let ast_var = astcast::cast_to_expr_id(&*node.args()[0]).unwrap();
        let ast_val = &*node.args()[1];
        let var_id = fun_bind.get_var_of_exp_id(ast_var).unwrap();
        let var_local = *self.act_fun_vars.get(&var_id).unwrap();

        self.tl_expr(ast_val);
        self.builder.ins_store(var_local);
    }

    //translate call to neg operator
    //must be handled diffenrtly from the others because we need to push a 0 before the argument
    fn tl_call_neg(&mut self, node: &ast::ASTExprCall) {
        assert!(node.args().len() == 1);

        // -x => 0 - x
        self.builder.ins_const(0);
        self.tl_expr(&*node.args()[0]);
        self.builder.ins_sub();
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
        let var_local = *self.act_fun_vars.get(&var_bind.id()).unwrap();

        self.tl_expr(&**node.init());
        self.builder.ins_store(var_local);
    }

    fn visit_expr_block(&mut self, node: &ast::ASTExprBlock) {
        for (idx, exp) in node.exprs().iter().enumerate() {
            let is_void = self.tl_expr(&**exp);
            if idx + 1 < node.exprs().len() && !is_void {
                //If not last val, discard unused value
                self.builder.ins_pop();
            }
        }
    }

    fn visit_expr_call(&mut self, node: &ast::ASTExprCall) {
        if node.name() == "@op:set" {
            //special case for set operator, cannot simply compute operands
            self.tl_call_set(node);
            return;
        } else if node.name() == "@op:neg" {
            //special case for neg operator, need to insert a 0 first
            self.tl_call_neg(node);
            return;
        }

        // 1) put arguments on the stack
        for arg in node.args() {
            self.tl_expr(&**arg);
        }

        // 2) Translate call
        if node.name().starts_with("@op") {
            self.tl_call_special(&node.name()[4..]);
        } else {
            self.tl_call(node);
        }
    }

    fn visit_expr_const(&mut self, node: &ast::ASTExprConst) {
        let val = node.val();
        self.builder.ins_const(val);
    }

    fn visit_expr_id(&mut self, node: &ast::ASTExprId) {
        let fun_bind = self.act_fun_bind.unwrap();
        let var_id = fun_bind.get_var_of_exp_id(node).unwrap();
        let var_local = *self.act_fun_vars.get(&var_id).unwrap();
        self.builder.ins_load(var_local);
    }

    fn visit_expr_if(&mut self, node: &ast::ASTExprIf) {
        let fun_bind = self.act_fun_bind.unwrap();
        let fun = *self.ir_funs.get(&fun_bind.id()).unwrap();

        // 1) Create basic blocks
        let bb_if = self.builder.create_basic_block(fun);
        let bb_else = self.builder.create_basic_block(fun);
        let bb_end = self.builder.create_basic_block(fun);

        // 1) Generate the condition code with the condition branching
        self.tl_expr(&**node.cond());
        self.builder.ins_br(bb_if, bb_else);

        // 2) Generate the if code with jump to the end
        self.builder.set_insert_point(bb_if);
        self.tl_expr(&**node.val_if());
        self.builder.ins_jump(bb_end);

        // 3) generate the else code with jump to the ende
        self.builder.set_insert_point(bb_else);
        self.tl_expr(&**node.val_else());
        self.builder.ins_jump(bb_end);

        // 4) generate the end code
        self.builder.set_insert_point(bb_end);

        // 5) Trick for the operands stack counters
        // If the node is not void, both the if and else branch pushed their return value to the operands stack
        // The code will only execute one of them, but the IRBuilder doesn't know this.
        // So we need to update the counter manually
        let is_void = fun_bind.get_type_of_exp(node).unwrap().is_void();
        if !is_void {
            self.builder.update_operands_count(-1);
        }
    }

    fn visit_expr_let(&mut self, node: &ast::ASTExprLet) {
        for def in node.defs() {
            def.accept(self);
        }
        self.tl_expr(&**node.val());
    }

    fn visit_expr_while(&mut self, node: &ast::ASTExprWhile) {
        let fun_bind = self.act_fun_bind.unwrap();
        let fun = *self.ir_funs.get(&fun_bind.id()).unwrap();

        // 1) Create basic blocks
        let bb_cond = self.builder.create_basic_block(fun);
        let bb_body = self.builder.create_basic_block(fun);
        let bb_end = self.builder.create_basic_block(fun);

        // 2) End current block by jumping to the condition
        self.builder.ins_jump(bb_cond);

        // 3) Generate the condition block with the conditional branching
        self.builder.set_insert_point(bb_cond);
        self.tl_expr(&**node.cond());
        self.builder.ins_br(bb_body, bb_end);

        // 4) Generate the body loop block with jump to the condition
        self.builder.set_insert_point(bb_body);
        self.tl_expr(&**node.body());
        self.builder.ins_jump(bb_cond);

        // 5) Generate the end block after the loop
        self.builder.set_insert_point(bb_end);
    }

    fn visit_type_name(&mut self, _node: &ast::ASTTypeName) {
        unreachable!();
    }
}
