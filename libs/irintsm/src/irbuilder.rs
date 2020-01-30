use std::collections::HashMap;

use crate::ir;
use crate::irvalidation;

// basic block in construction
struct BasicBlockBuilder {
    id: ir::BasicBlockRef,
    ins: Vec<ir::Ins>,
    complete: bool,
}

impl BasicBlockBuilder {
    fn new(id: ir::BasicBlockRef) -> Self {
        BasicBlockBuilder {
            id,
            ins: vec![],
            complete: false,
        }
    }

    fn add_ins(&mut self, ins: ir::Ins) {
        if self.complete {
            panic!("Basic block already completed");
        }
        self.ins.push(ins);
    }

    fn finish(self) -> ir::BasicBlock {
        if !self.complete {
            panic!("Basic block not completed yet");
        }
        ir::BasicBlock::new(self.id, self.ins)
    }
}

// function in construction
struct FunctionBuilder {
    id: ir::FunctionRef,
    bbs: Vec<BasicBlockBuilder>,
}

impl FunctionBuilder {
    fn new(id: ir::FunctionRef) -> Self {
        FunctionBuilder { id, bbs: vec![] }
    }

    fn finish(mut self) -> ir::Function {
        let mut bbs = vec![];
        std::mem::swap(&mut bbs, &mut self.bbs);
        let bbs: Vec<_> = bbs.into_iter().map(|x| x.finish()).collect();
        let bbs = if bbs.len() == 0 { None } else { Some(bbs) };

        ir::Function::new(self.id, bbs)
    }
}

/// Best way to create an ir::Module
/// Helps to handle functions, BasicBlocks, and creation of instructions
/// It has an insert point, set to None by default, where new instruction are added.
/// This insert point must be changed to the end of the basicblock where we want to insert code
/// The created ir::Module is checked using irvalidation
pub struct IRBuilder {
    funs: Vec<FunctionBuilder>,
    bbs: Vec<BasicBlockBuilder>,
    mapping_fun: HashMap<ir::FunctionRef, usize>,
    mapping_bb: HashMap<ir::BasicBlockRef, usize>,
    mapping_bb_fun: HashMap<ir::BasicBlockRef, ir::FunctionRef>,

    act_bb: Option<ir::BasicBlockRef>,
}

impl IRBuilder {
    pub fn new() -> Self {
        IRBuilder {
            funs: vec![],
            bbs: vec![],
            mapping_fun: HashMap::new(),
            mapping_bb: HashMap::new(),
            mapping_bb_fun: HashMap::new(),

            act_bb: None,
        }
    }

    /// Returns the basic block id of the actual insert point
    /// Or None if there is no insert point
    pub fn actual_basic_block(&self) -> Option<ir::BasicBlockRef> {
        self.act_bb
    }

    /// Returns the function id of the actual insert point
    /// Or None if there is no insert point
    pub fn actual_function(&self) -> Option<ir::FunctionRef> {
        let bb_id = self.act_bb?;
        Some(*self.mapping_bb_fun.get(&bb_id).unwrap())
    }

    /// Change the insert point to the end of a basic block
    pub fn set_insert_point(&mut self, bb: ir::BasicBlockRef) {
        assert!(self.mapping_bb.get(&bb).is_some());
        self.act_bb = Some(bb);
    }

    /// Remove the insert point, cannot create functions
    pub fn reset_insert_point(&mut self) {
        self.act_bb = None;
    }

    /// Create a new empty function
    pub fn create_function(&mut self, id: Option<ir::FunctionRef>) -> ir::FunctionRef {
        let id = id.unwrap_or(self.gen_fun_id());
        if self.mapping_fun.get(&id).is_some() {
            panic!("There is already a function with id {:?}", id);
        }

        let fun_idx = self.funs.len();
        let fun = FunctionBuilder::new(id);
        self.funs.push(fun);
        self.mapping_fun.insert(id, fun_idx);
        id
    }

    /// Add a new empty basic block to a function
    /// It doesn't change the insert point
    pub fn create_basic_block(&mut self, fun_id: ir::FunctionRef) -> ir::BasicBlockRef {
        assert!(self.mapping_fun.get(&fun_id).is_some());

        let id = self.gen_bb_id();
        let bb_idx = self.bbs.len();
        let bb = BasicBlockBuilder::new(id);
        self.bbs.push(bb);
        self.mapping_bb.insert(id, bb_idx);
        self.mapping_bb_fun.insert(id, fun_id);
        id
    }

    /// Discard the builder and create the final ir::Module
    /// Also validates the module
    pub fn finish(self) -> ir::Module {
        let funs = self.funs.into_iter().map(|f| f.finish()).collect();
        let res = ir::Module::new(funs);
        irvalidation::validate_module(&res);
        res
    }

    /// Add an instruction at the current insert point
    /// Use carefully, it's often better to use ins_* instead
    pub fn add_instruction(&mut self, ins: ir::Ins) {
        let bb_id = match self.act_bb {
            Some(id) => id,
            None => panic!("The insert point is None"),
        };

        let bb = self
            .bbs
            .get_mut(*self.mapping_bb.get(&bb_id).unwrap())
            .unwrap();
        bb.add_ins(ins);
    }

    pub fn ins_pop(&mut self) {
        self.add_instruction(ir::Ins::Pop(ir::InsPop::new()));
    }

    pub fn ins_const(&mut self, val: i32) {
        self.add_instruction(ir::Ins::Const(ir::InsConst::new(val)));
    }

    pub fn ins_load(&mut self, src: ir::LocalsIndex) {
        self.add_instruction(ir::Ins::Load(ir::InsLoad::new(src)));
    }

    pub fn ins_store(&mut self, dst: ir::LocalsIndex) {
        self.add_instruction(ir::Ins::Store(ir::InsStore::new(dst)));
    }

    pub fn ins_add(&mut self) {
        self.add_instruction(ir::Ins::Opbin(ir::InsOpbin::Add));
    }

    pub fn ins_sub(&mut self) {
        self.add_instruction(ir::Ins::Opbin(ir::InsOpbin::Sub));
    }

    pub fn ins_mul(&mut self) {
        self.add_instruction(ir::Ins::Opbin(ir::InsOpbin::Mul));
    }

    pub fn ins_div(&mut self) {
        self.add_instruction(ir::Ins::Opbin(ir::InsOpbin::Div));
    }

    pub fn ins_rem(&mut self) {
        self.add_instruction(ir::Ins::Opbin(ir::InsOpbin::Rem));
    }

    pub fn ins_cmpeq(&mut self) {
        self.add_instruction(ir::Ins::Cmpbin(ir::InsCmpbin::Eq));
    }

    pub fn ins_cmplt(&mut self) {
        self.add_instruction(ir::Ins::Cmpbin(ir::InsCmpbin::Lt));
    }

    pub fn ins_cmpgt(&mut self) {
        self.add_instruction(ir::Ins::Cmpbin(ir::InsCmpbin::Gt));
    }

    pub fn ins_jump(&mut self, dst: ir::BasicBlockRef) {
        self.add_instruction(ir::Ins::Jump(ir::InsJump::new(dst)));
    }

    pub fn ins_br(&mut self, dst_true: ir::BasicBlockRef, dst_false: ir::BasicBlockRef) {
        self.add_instruction(ir::Ins::Br(ir::InsBr::new(dst_true, dst_false)));
    }

    pub fn ins_call(&mut self, fun: ir::FunctionRef, nb_args: usize) {
        self.add_instruction(ir::Ins::Call(ir::InsCall::new(fun, nb_args)));
    }

    pub fn ins_ret(&mut self) {
        self.add_instruction(ir::Ins::Ret(ir::InsRet::new()));
    }

    fn gen_bb_id(&self) -> ir::BasicBlockRef {
        ir::BasicBlockRef::new(self.bbs.len())
    }

    fn gen_fun_id(&self) -> ir::FunctionRef {
        let mut id = self.funs.len();
        loop {
            if self.mapping_fun.get(&ir::FunctionRef::new(id)).is_none() {
                break;
            }
            id += 1;
        }
        ir::FunctionRef::new(id)
    }
}
