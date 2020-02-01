use crate::ir;

use std::collections::HashMap;
use std::collections::HashSet;

// This file contains tools to divide the IR into basic blocks, and manipulate it

/// Represent an identifier of a basicblock, to have a reference to basic blocks, and to be able to easily move them
/// Each BasicBlockRef has a unique value in the module
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BasicBlockRef(pub usize);

#[derive(Debug)]
pub struct BasicBlock {
    id: BasicBlockRef,
    fun: ir::FunAddress,
    begin: ir::LocalLabel,
    end: ir::LocalLabel,
}

struct FunctionBasicBlocks {
    bbs: Vec<BasicBlockRef>,                      //basic block ordering,
    begs: HashMap<ir::LocalLabel, BasicBlockRef>, //which basic block start at a specific instruction
    ends: HashMap<ir::LocalLabel, BasicBlockRef>, //which basic block ends at a specific instruction (actually the control flow is one before end)
}

pub struct ModuleBasicBlocks {
    bbs: Vec<BasicBlock>,
    fun_infos: HashMap<ir::FunAddress, FunctionBasicBlocks>,
    bb_to_fun: HashMap<BasicBlockRef, ir::FunAddress>, //at which function a basic block belongs to
}

impl ModuleBasicBlocks {
    fn new() -> Self {
        ModuleBasicBlocks {
            bbs: vec![],
            fun_infos: HashMap::new(),
            bb_to_fun: HashMap::new(),
        }
    }

    /// Do not create any basic block, just the list of defined functions
    /// Ignore extern functions
    fn register_funs(&mut self, module: &ir::Module) {
        for fun in module.defs() {
            if fun.is_extern() {
                continue;
            }
            self.fun_infos.insert(
                fun.addr(),
                FunctionBasicBlocks {
                    bbs: vec![],
                    begs: HashMap::new(),
                    ends: HashMap::new(),
                },
            );
        }
    }

    fn add_basic_block(&mut self, fun: ir::FunAddress, begin: ir::LocalLabel, end: ir::LocalLabel) {
        let id = BasicBlockRef(self.bbs.len());
        let bb = BasicBlock {
            id,
            fun,
            begin,
            end,
        };
        self.bbs.push(bb);
        self.bb_to_fun.insert(id, fun);

        let infos = self.fun_infos.get_mut(&fun).unwrap();
        infos.bbs.push(id);
        infos.begs.insert(begin, id);
        infos.ends.insert(end, id);
    }

    pub fn get_bb_starting_at(
        &self,
        fun: ir::FunAddress,
        pos: ir::LocalLabel,
    ) -> Option<BasicBlockRef> {
        let infos = self.fun_infos.get(&fun).unwrap();
        infos.begs.get(&pos).copied()
    }

    pub fn get_bb_ending_at(
        &self,
        fun: ir::FunAddress,
        pos: ir::LocalLabel,
    ) -> Option<BasicBlockRef> {
        let infos = self.fun_infos.get(&fun).unwrap();
        infos.ends.get(&pos).copied()
    }
}

pub struct BasicBlocksBuilder<'a> {
    module: &'a ir::Module,
    blocs: ModuleBasicBlocks,
}

impl<'a> BasicBlocksBuilder<'a> {
    pub fn new(module: &'a ir::Module) -> Self {
        BasicBlocksBuilder {
            module,
            blocs: ModuleBasicBlocks::new(),
        }
    }

    pub fn build(mut self) -> ModuleBasicBlocks {
        self.blocs.register_funs(self.module);

        for fun in self.module.defs() {
            if fun.is_extern() {
                continue;
            }
            self.build_fun(fun);
            self.check_fun(fun);
        }

        self.blocs
    }

    fn build_fun(&mut self, fun: &'a ir::DefFun) {
        let fun_id = fun.addr();

        let mut bb_start = 0;
        for (ins_idx, ins) in fun.body().unwrap().iter().enumerate() {
            if ins.is_control_flow() {
                let bb_end = ins_idx + 1;
                self.blocs.add_basic_block(
                    fun_id,
                    ir::LocalLabel(bb_start),
                    ir::LocalLabel(bb_end),
                );
                bb_start = bb_end;
            }
        }

        if bb_start != fun.body().unwrap().len() {
            panic!("Last instruction must be a control flow");
        }
    }

    fn check_fun(&mut self, fun: &'a ir::DefFun) {
        let fun_id = fun.addr();
        let mut begins = HashSet::new();

        for ins in fun.body().unwrap() {
            match ins {
                ir::Ins::Jump(ins) => {
                    begins.insert(ins.label());
                }
                ir::Ins::Br(ins) => {
                    begins.insert(ins.label_true());
                    begins.insert(ins.label_true());
                }
                _ => {}
            }
        }

        for label in begins {
            if self.blocs.get_bb_starting_at(fun_id, label).is_none() {
                panic!("Invalid code: branching to instruction in the middle of a Basic Block");
            }
        }
    }
}
