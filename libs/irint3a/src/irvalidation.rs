// This module is used to validate the IR of a function or a whole module
//
// There is a set of rules that must be enforced by every modules
// Some are enforced by design and API calls, others must be checked
//
// Every function id must be unique [ENFORCED]
// A function must have at least one basic block
// All basic blocks of a function must be unique [ENFORCED]
// A Basic Block must not be empty
// The last instruction of a basic block must be a control flow instruction
// The non-last instruction of a basick block cannot be a control flow instruction
// Branching instructions must jump to basic blocs of the same function
// Call instructions must reference existing functions

use std::collections::HashSet;

use crate::ir;

fn validate_fun(
    fun: &ir::Function,
    funs: Option<&HashSet<ir::FunctionId>>,
    out_res: Option<&mut bool>,
) {
    let mut vd = FunctionValidation::new(fun, funs);
    vd.check();

    let mut valid = true;
    for err in vd.errs {
        println!("F #{}: {:?}", fun.id().0, err);
        valid = false;
    }
    if !valid {
        match out_res {
            Some(out_res) => *out_res = false,
            None => panic!("Function validation for #{} failed", fun.id().0),
        }
    }
}

/// Validate if a function is rightly constructed
/// `funs` is the optional list of all functions in the module, in order to check if a call is valid
/// If none, calls are not checked
/// Print errors and panics if validation failed
pub fn validate_function(fun: &ir::Function, funs: Option<&HashSet<ir::FunctionId>>) {
    validate_fun(fun, funs, None);
}

/// Validate if a whole module is rightly constructed
/// Print errors and panics if validation failed
pub fn validate_module(module: &ir::Module) {
    let mut funs = HashSet::new();
    for fun in module.funs() {
        funs.insert(fun.id());
    }

    let mut valid = true;
    for fun in module.funs() {
        validate_fun(fun, Some(&funs), Some(&mut valid));
    }

    if !valid {
        panic!("Module validation failed");
    }
}

#[derive(Debug)]
pub struct InsError {
    pub mess: &'static str,
    pub ins_id: usize,
    pub bb_id: ir::BasicBlockId,
    pub fun_id: ir::FunctionId,
}

#[derive(Debug)]
pub struct BasicBlockError {
    pub mess: &'static str,
    pub bb_id: ir::BasicBlockId,
    pub fun_id: ir::FunctionId,
}

#[derive(Debug)]
pub struct FunctionError {
    pub mess: &'static str,
    pub fun_id: ir::FunctionId,
}

#[derive(Debug)]
pub enum ValidationError {
    Ins(InsError),
    BasicBlock(BasicBlockError),
    Function(FunctionError),
}

struct FunctionValidation<'a> {
    fun: &'a ir::Function,
    errs: Vec<ValidationError>,
    fun_ids: Option<&'a HashSet<ir::FunctionId>>,
    bb_ids: HashSet<ir::BasicBlockId>,

    act_bb: Option<&'a ir::BasicBlock>,
    act_ins: Option<usize>,
}

impl<'a> FunctionValidation<'a> {
    fn new(fun: &'a ir::Function, fun_ids: Option<&'a HashSet<ir::FunctionId>>) -> Self {
        FunctionValidation {
            fun,
            errs: vec![],
            fun_ids,
            bb_ids: HashSet::new(),

            act_bb: None,
            act_ins: None,
        }
    }

    // 1) A function must have at least one basic block
    fn check(&mut self) {
        if self.fun.is_extern() {
            //nothing to check in extern function
            return;
        }

        if self.fun.basic_blocks_list().len() == 0 {
            //1)
            self.err_fun("Function has no Basic Blocks");
        }

        for bb in self.fun.basic_blocks_list() {
            self.bb_ids.insert(*bb);
        }

        for bb_id in self.fun.basic_blocks_list() {
            let bb = self.fun.get_basic_block(*bb_id);
            self.act_bb = Some(bb);
            self.check_bb();
        }
        self.act_bb = None;
    }

    // 1) A Basic Block must not be empty
    fn check_bb(&mut self) {
        let bb = self.act_bb.unwrap();

        if bb.size() == 0 {
            //1)
            self.err_bb("BasicBlock is empty");
        }

        for i in 0..bb.size() {
            self.act_ins = Some(i);
            self.check_ins();
        }
        self.act_ins = None;
    }

    // 1) the last instruction must be a control flow instruction (br, jump, or ret)
    // 2) there must not be any other control flow instruction
    // 3) branching instructions must jump to basic blocs of the same function
    // 4) call instructions must reference existing functions
    fn check_ins(&mut self) {
        let ins_idx = self.act_ins.unwrap();
        let bb = self.act_bb.unwrap();
        let ins = bb.get_ins(ins_idx);
        let is_last = ins_idx == bb.size() - 1;

        if is_last && !ins.is_control_flow() {
            // 1)
            return self.err_ins("Last of BasicBlock is not a control flow instruction");
        }

        if !is_last && ins.is_control_flow() {
            // 2)
            return self.err_ins("Non-last of BasicBlock is a control flow instruction");
        }

        if let ir::Ins::Jump(ins) = ins {
            if self.bb_ids.get(&ins.dst()).is_none() {
                // 3)
                return self.err_ins("Jump to undefined Basic Block");
            }
        } else if let ir::Ins::Br(ins) = ins {
            if self.bb_ids.get(&ins.dst_true()).is_none()
                || self.bb_ids.get(&ins.dst_false()).is_none()
            {
                // 3)
                return self.err_ins("Br to undefined Basic Block");
            }
        } else if let ir::Ins::Call(ins) = ins {
            if let Some(fun_ids) = self.fun_ids {
                if fun_ids.get(&ins.fun()).is_none() {
                    // 4)
                    return self.err_ins("Call to undefined function");
                }
            }
        }
    }

    fn err_ins(&mut self, mess: &'static str) {
        self.errs.push(ValidationError::Ins(InsError {
            mess,
            ins_id: self.act_ins.unwrap(),
            bb_id: self.act_bb.unwrap().id(),
            fun_id: self.fun.id(),
        }));
    }

    fn err_bb(&mut self, mess: &'static str) {
        self.errs.push(ValidationError::BasicBlock(BasicBlockError {
            mess,
            bb_id: self.act_bb.unwrap().id(),
            fun_id: self.fun.id(),
        }));
    }

    fn err_fun(&mut self, mess: &'static str) {
        self.errs.push(ValidationError::Function(FunctionError {
            mess,
            fun_id: self.fun.id(),
        }));
    }
}
