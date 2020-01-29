use std::collections::HashSet;

use crate::ir;

// Check module and return all errors
pub fn validate_module(module: &ir::Module) -> Vec<ValidationError> {
    let mut vd = ModuleValidation::new(module);
    vd.check();
    vd.errs
}

#[derive(Debug)]
pub struct InsError {
    pub mess: &'static str,
    pub ins_id: usize,
    pub bb_id: ir::BasicBlockRef,
    pub fun_id: ir::FunctionRef,
}

#[derive(Debug)]
pub struct BasicBlockError {
    pub mess: &'static str,
    pub bb_id: ir::BasicBlockRef,
    pub fun_id: ir::FunctionRef,
}

#[derive(Debug)]
pub struct FunctionError {
    pub mess: &'static str,
    pub fun_id: ir::FunctionRef,
}

#[derive(Debug)]
pub enum ValidationError {
    Ins(InsError),
    BasicBlock(BasicBlockError),
    Function(FunctionError),
}

struct ModuleValidation<'a> {
    module: &'a ir::Module,
    errs: Vec<ValidationError>,
    fun_ids: HashSet<usize>,

    act_fun: Option<&'a ir::Function>,
    act_bb: Option<&'a ir::BasicBlock>,
    act_ins: Option<usize>,
}

impl<'a> ModuleValidation<'a> {
    fn new(module: &'a ir::Module) -> Self {
        ModuleValidation {
            module,
            errs: vec![],
            fun_ids: HashSet::new(),

            act_fun: None,
            act_bb: None,
            act_ins: None,
        }
    }

    fn check(&mut self) {
        for fun in self.module.fun_list() {
            self.fun_ids.insert(fun.id().0);
        }

        for fun in self.module.fun_list() {
            if fun.is_extern() {
                continue;
            }
            self.act_fun = Some(fun);
            self.check_fun();
        }
        self.act_fun = None;
    }

    // 1) BasicBlocks must be in the exact order of their ids
    // 2) A function must have at least one basic block
    fn check_fun(&mut self) {
        let fun = self.act_fun.unwrap();

        if fun.bb_list().len() == 0 {
            //2)
            return self.err_fun("Function has no Basic Blocks");
        }

        for (i, bb) in fun.bb_list().iter().enumerate() {
            if bb.id().0 != i {
                // 1)
                return self.err_fun("Invalid ordering of basic blocks");
            }
        }

        for bb in fun.bb_list() {
            self.act_bb = Some(bb);
            self.check_bb();
        }
        self.act_bb = None;
    }

    // 1) a Basic Block must not be empty
    fn check_bb(&mut self) {
        let bb = self.act_bb.unwrap();

        if bb.ins_list().len() == 0 {
            //1)
            return self.err_bb("BasicBlock is empty");
        }

        for i in 0..bb.ins_list().len() {
            self.act_ins = Some(i);
            self.check_ins();
        }
        self.act_ins = None;
    }

    // 1) the last instruction must be a control flow instruction (br, jump, or ret)
    // 2) there must not be any other control flow instruction
    // 3) branching instructions must jump to existing basic blocs
    // 4) call instructions must reference existing functions
    fn check_ins(&mut self) {
        let ins_id = self.act_ins.unwrap();
        let ins = self.act_bb.unwrap().ins_list()[ins_id];
        let nb_bbs = self.act_fun.unwrap().bb_list().len();
        let is_last = ins_id == self.act_bb.unwrap().ins_list().len() - 1;

        if is_last && !ins.is_control_flow() {
            // 1)
            return self.err_ins("Last of BasicBlock is not a control flow instruction");
        }

        if !is_last && ins.is_control_flow() {
            // 2)
            return self.err_ins("Non-last of BasicBlock is a control flow instruction");
        }

        if let ir::Ins::Jump(ins) = ins {
            if ins.dst().0 >= nb_bbs {
                // 3)
                return self.err_ins("Jump to undefined Basic Block");
            }
        } else if let ir::Ins::Br(ins) = ins {
            if ins.dst_true().0 >= nb_bbs || ins.dst_false().0 >= nb_bbs {
                // 3)
                return self.err_ins("Br to undefined Basic Block");
            }
        } else if let ir::Ins::Call(ins) = ins {
            if self.fun_ids.get(&ins.fun().0).is_none() {
                // 4)
                return self.err_ins("Call to undefined function");
            }
        }
    }

    fn err_ins(&mut self, mess: &'static str) {
        self.errs.push(ValidationError::Ins(InsError {
            mess,
            ins_id: self.act_ins.unwrap(),
            bb_id: self.act_bb.unwrap().id(),
            fun_id: self.act_fun.unwrap().id(),
        }));
    }

    fn err_bb(&mut self, mess: &'static str) {
        self.errs.push(ValidationError::BasicBlock(BasicBlockError {
            mess,
            bb_id: self.act_bb.unwrap().id(),
            fun_id: self.act_fun.unwrap().id(),
        }));
    }

    fn err_fun(&mut self, mess: &'static str) {
        self.errs.push(ValidationError::Function(FunctionError {
            mess,
            fun_id: self.act_fun.unwrap().id(),
        }));
    }
}
