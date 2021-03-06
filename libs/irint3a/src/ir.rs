// Basic rules about the IR
//
// Registers:
// All registrs are 32 bits
// There is an infinite number of registers, identified by a unique usize
//
// Datatypes:
// The IR is not typed, but basically instructions work on 2 types of data:
// - 32 bits signed integer
// - 32 bits memory address
//
// Function call / return:
// all function arguments are 32 bits, and stored in register 0, 1, 2, etc
// The notion of stack frame / calling convention is abstracted away and not present in the IR
// There is no special register to pass arguments, or get return value: it's all chosen with call instruction
// There is no special register to set return value: it's all chosen with ret instruction
// All the registers remain unchanged after returning from a function call (even if callee change some registers)
//
// Branching:
// Jump / Br instructions can only jump to the beginning of a basic block of the same function
//
// Control flow instructions:
// Instructions that cause to jump to another point in the program
// These are the instructions Jump, Br, and Ret
// (not call because for the caller perspective it simply goes to the next instruction)
//
// IR Validity
// Every module must respect rules
// It's possible to create an invalid IR
// More infos about the rules and checking at irvalidation.rs
//
// IR Syntax
// More infos at irparser.rs

use std::collections::HashMap;

/// Registers are represented by a unique usize identifier
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct RegId(pub usize);

/// BasicBlock unique identifier
/// Every basic block in a function has a different identifier
/// It's used to manipulate and move around basic blocks more easily (without using the whole structure)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct BasicBlockId(pub usize);

/// Function unique identifier
/// It's used to reference a function without having to manipulate the whole structure
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct FunctionId(pub usize);

/// Represent a simple IR instruction
#[derive(Clone, Debug)]
pub enum Ins {
    Movi(InsMovi),
    Movr(InsMovr),
    Load(InsLoad),
    Store(InsStore),
    Alloca(InsAlloca),
    Opbin(InsOpbin),   //add, sub, mov, div, mod
    Cmpbin(InsCmpbin), //cmpeq, cmplt, cmpgt
    Jump(InsJump),
    Br(InsBr),
    Call(InsCall),
    Ret(InsRet),
}

impl Ins {
    pub fn is_control_flow(&self) -> bool {
        match self {
            Ins::Jump(_) | Ins::Br(_) | Ins::Ret(_) => true,
            _ => false,
        }
    }
}

/// Instruction movi
/// Move constant integer into a register
#[derive(Clone, Copy, Debug)]
pub struct InsMovi {
    dst: RegId,
    const_val: i32,
}

impl InsMovi {
    pub fn new(dst: RegId, const_val: i32) -> Self {
        InsMovi { dst, const_val }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn const_val(&self) -> i32 {
        self.const_val
    }
}

/// Instruction movr
/// Copy value of one register to another
#[derive(Clone, Copy, Debug)]
pub struct InsMovr {
    dst: RegId,
    src: RegId,
}

impl InsMovr {
    pub fn new(dst: RegId, src: RegId) -> Self {
        InsMovr { dst, src }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn src(&self) -> RegId {
        self.src
    }
}

/// Instruction load
/// Read the 32bit address stored in src register, and load the value at that address to the register dst
#[derive(Clone, Copy, Debug)]
pub struct InsLoad {
    dst: RegId,
    src: RegId,
}

impl InsLoad {
    pub fn new(dst: RegId, src: RegId) -> Self {
        InsLoad { dst, src }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn src(&self) -> RegId {
        self.src
    }
}

/// Instruction store
/// Read the 32bit adress stored in dst register, and store the content of src register at that address
#[derive(Clone, Copy, Debug)]
pub struct InsStore {
    dst: RegId,
    src: RegId,
}

impl InsStore {
    pub fn new(dst: RegId, src: RegId) -> Self {
        InsStore { dst, src }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn src(&self) -> RegId {
        self.src
    }
}

/// Instruction Alloca
/// Allocate a variable local to the function, and write it's memory address in dst register
#[derive(Clone, Copy, Debug)]
pub struct InsAlloca {
    dst: RegId,
}

impl InsAlloca {
    pub fn new(dst: RegId) -> Self {
        InsAlloca { dst }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InsOpbinKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

/// Represent multiple instructions for binary operations
/// Read int32 values from both src registers, compute and store result in dst register
/// regroup instructions add, sub, mul, div, mod
#[derive(Clone, Copy, Debug)]
pub struct InsOpbin {
    kind: InsOpbinKind,
    dst: RegId,
    src1: RegId,
    src2: RegId,
}

impl InsOpbin {
    pub fn new(kind: InsOpbinKind, dst: RegId, src1: RegId, src2: RegId) -> Self {
        InsOpbin {
            kind,
            dst,
            src1,
            src2,
        }
    }

    pub fn kind(&self) -> InsOpbinKind {
        self.kind
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn src1(&self) -> RegId {
        self.src1
    }

    pub fn src2(&self) -> RegId {
        self.src2
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InsCmpbinKind {
    Eq,
    Lt,
    Gt,
}

/// Represent multiple instructions for binary comparisons
/// Read int32 values from both src registers, compute and store result in dst register: 1 is cmp is true, 0 if false
/// regroup instructions cmpeq, cmplt, cmpgt
#[derive(Clone, Copy, Debug)]
pub struct InsCmpbin {
    kind: InsCmpbinKind,
    dst: RegId,
    src1: RegId,
    src2: RegId,
}

impl InsCmpbin {
    pub fn new(kind: InsCmpbinKind, dst: RegId, src1: RegId, src2: RegId) -> Self {
        InsCmpbin {
            kind,
            dst,
            src1,
            src2,
        }
    }

    pub fn kind(&self) -> InsCmpbinKind {
        self.kind
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn src1(&self) -> RegId {
        self.src1
    }

    pub fn src2(&self) -> RegId {
        self.src2
    }
}

/// Instruction jump
/// Unconditional jump to a basic block
/// The basic block must belong to the current function
/// (checked with validator module)
#[derive(Clone, Copy, Debug)]
pub struct InsJump {
    dst: BasicBlockId,
}

impl InsJump {
    pub fn new(dst: BasicBlockId) -> Self {
        InsJump { dst }
    }

    pub fn dst(&self) -> BasicBlockId {
        self.dst
    }
}

/// Instruction Br
/// Conditional jump depending on value of register src
/// If value != 0, jump to dst-true, otherwhise to dst-false
/// The basic blocks must belong to the current function
/// (checked with validator module)
#[derive(Clone, Copy, Debug)]
pub struct InsBr {
    src: RegId,
    dst_true: BasicBlockId,
    dst_false: BasicBlockId,
}

impl InsBr {
    pub fn new(src: RegId, dst_true: BasicBlockId, dst_false: BasicBlockId) -> Self {
        InsBr {
            src,
            dst_true,
            dst_false,
        }
    }

    pub fn src(&self) -> RegId {
        self.src
    }

    pub fn dst_true(&self) -> BasicBlockId {
        self.dst_true
    }

    pub fn dst_false(&self) -> BasicBlockId {
        self.dst_false
    }
}

/// Instruction call
/// Call specified function with arguments stored in given args registers, and store return value in dst register
/// The function id must exit in the current Module
/// (checked with validator module)
#[derive(Clone, Debug)]
pub struct InsCall {
    dst: RegId,
    fun: FunctionId,
    args: Vec<RegId>,
}

impl InsCall {
    pub fn new(dst: RegId, fun: FunctionId, args: Vec<RegId>) -> Self {
        InsCall { dst, fun, args }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn fun(&self) -> FunctionId {
        self.fun
    }

    pub fn args(&self) -> &Vec<RegId> {
        &self.args
    }
}

/// Instruction ret
/// Stop the current function, and return the value in the register src
#[derive(Clone, Copy, Debug)]
pub struct InsRet {
    src: RegId,
}

impl InsRet {
    pub fn new(src: RegId) -> Self {
        InsRet { src }
    }

    pub fn src(&self) -> RegId {
        self.src
    }
}

/// A Basic block is an ordered sequence of instructions that must end with a control flow instruction.
/// This rule is not enforced by the struct implementation, but by an extern validation module.
/// It's possible to insert or remove instructions anywhere from the list,
/// to change the position of one instruction, or to mutate one instruction
pub struct BasicBlock {
    id: BasicBlockId,
    fun_id: FunctionId,
    ins_list: Vec<Ins>,
}

impl BasicBlock {
    fn new(id: BasicBlockId, fun_id: FunctionId) -> Self {
        BasicBlock {
            id,
            fun_id,
            ins_list: vec![],
        }
    }

    pub fn id(&self) -> BasicBlockId {
        self.id
    }

    pub fn fun_id(&self) -> FunctionId {
        self.fun_id
    }

    /// Returns the number of instructions
    pub fn size(&self) -> usize {
        self.ins_list.len()
    }

    pub fn get_ins(&self, idx: usize) -> &Ins {
        &self.ins_list[idx]
    }

    pub fn get_ins_mut(&mut self, idx: usize) -> &mut Ins {
        &mut self.ins_list[idx]
    }

    pub fn iter(&self) -> std::slice::Iter<Ins> {
        self.ins_list.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Ins> {
        self.ins_list.as_mut_slice().iter_mut()
    }

    /// Add an instruction at the end of the basic block
    pub fn push_ins(&mut self, ins: Ins) {
        self.ins_list.push(ins);
    }

    /// Remove the final instruction of the basic block
    pub fn pop_ins(&mut self) {
        self.ins_list.pop().unwrap();
    }

    /// Insert instruction `ins` at the position `idx`, all other instructions after will be shifted to the right
    pub fn insert_ins(&mut self, idx: usize, ins: Ins) {
        self.ins_list.insert(idx, ins);
    }

    /// Remove instruction at the position `idx`, all other instructions after will be shifted to the left
    pub fn remove_ins(&mut self, idx: usize) {
        self.ins_list.remove(idx);
    }

    /// Move the instruction at the position `old_idx` to the position `new_idx`, shifting instructions in between.
    pub fn move_ins(&mut self, old_idx: usize, new_idx: usize) {
        let ins = self.ins_list.remove(old_idx);
        self.ins_list.insert(new_idx, ins);
    }
}

/// Function definition
/// A function has no types or registers, it simply pass argument and return values through registers as it wants
/// A function can be extern: only a declaration, no code
///
/// A function is an ordered sequence of basic block
/// It's possible to insert, remove and change the order of basic blocks
/// The basic block at position 0 is the entry point of the function
/// The ordering after 0 doesn't change program execution, only the display of code
pub struct Function {
    id: FunctionId,
    is_extern: bool,
    bbs: HashMap<BasicBlockId, BasicBlock>,
    bbs_list: Vec<BasicBlockId>,
    bb_count: usize, //counter to create unique ids
}

impl Function {
    fn new(id: FunctionId, is_extern: bool) -> Self {
        Function {
            id,
            is_extern,
            bbs: HashMap::new(),
            bbs_list: vec![],
            bb_count: 0,
        }
    }

    pub fn id(&self) -> FunctionId {
        self.id
    }

    pub fn is_extern(&self) -> bool {
        self.is_extern
    }

    pub fn basic_blocks_list(&self) -> &[BasicBlockId] {
        assert!(!self.is_extern);
        &self.bbs_list
    }

    pub fn get_basic_block(&self, id: BasicBlockId) -> &BasicBlock {
        assert!(!self.is_extern);
        self.bbs.get(&id).unwrap()
    }

    pub fn get_basic_block_mut(&mut self, id: BasicBlockId) -> &mut BasicBlock {
        assert!(!self.is_extern);
        self.bbs.get_mut(&id).unwrap()
    }

    /// Return the position of a basic blocks in the ordered sequence
    pub fn get_basic_block_pos(&mut self, id: BasicBlockId) -> usize {
        assert!(!self.is_extern);
        self.bbs_list.iter().position(|x| *x == id).unwrap()
    }

    /// Move the basicblock at the position `old_idx` to the position `new_idx`, shifting basicblocks in between.
    pub fn move_basic_block(&mut self, old_idx: usize, new_idx: usize) {
        assert!(!self.is_extern);
        let bb = self.bbs_list.remove(old_idx);
        self.bbs_list.insert(new_idx, bb);
    }

    /// Change the entry point of the function, shifting right all basic blocks before `id`
    /// `id` must refer to a basic block already in the function
    pub fn set_entry_point(&mut self, id: BasicBlockId) {
        assert!(!self.is_extern);
        let pos = self.get_basic_block_pos(id);
        self.move_basic_block(pos, 0);
    }

    /// Add a new empty basic block at the end of the function
    pub fn create_basic_block(&mut self) -> BasicBlockId {
        assert!(!self.is_extern);
        let bb_id = self.gen_bb_id();
        let bb = BasicBlock::new(bb_id, self.id);

        if self.bbs.insert(bb_id, bb).is_some() {
            panic!("Internal error: there is alreay a basic block in the function with this id");
        }
        self.bbs_list.push(bb_id);
        bb_id
    }

    /// Remove the basic block `id` belonging to the function`
    pub fn remove_basic_block(&mut self, id: BasicBlockId) {
        assert!(!self.is_extern);
        if self.bbs.remove(&id).is_none() {
            panic!("Failed to remove basic block: unknown id");
        }
        let pos = self.get_basic_block_pos(id);
        self.bbs_list.remove(pos);
    }

    fn gen_bb_id(&mut self) -> BasicBlockId {
        let id = BasicBlockId(self.bb_count);
        self.bb_count += 1;
        id
    }
}

/// A module represent the whole definition of an IR file
/// It's a sequence of functions
/// It's possible to add or remove functions
/// Remove a function also remove all the basic blocks
/// You also need to use this class if you want to add/remove a basic block for a function
pub struct Module {
    funs: Vec<Function>,
    funs_by_id: HashMap<FunctionId, usize>, //value is the index in funs vector
}

impl Module {
    pub fn new() -> Self {
        Module {
            funs: vec![],
            funs_by_id: HashMap::new(),
        }
    }

    pub fn funs(&self) -> &[Function] {
        &self.funs
    }

    pub fn get_fun(&self, id: FunctionId) -> Option<&Function> {
        let idx = *self.funs_by_id.get(&id)?;
        Some(&self.funs[idx])
    }

    pub fn get_fun_mut(&mut self, id: FunctionId) -> Option<&mut Function> {
        let idx = *self.funs_by_id.get(&id)?;
        Some(&mut self.funs[idx])
    }

    /// Create a new empty non-extern function
    /// `id` optional id for the new function, if none, one is generated
    pub fn create_function(&mut self, id: Option<FunctionId>) -> FunctionId {
        let id = id.unwrap_or(self.gen_fun_id());
        self.create_new_function(id, false);
        id
    }

    /// Create an extern function
    pub fn create_extern_function(&mut self, id: FunctionId) {
        self.create_new_function(id, true);
    }

    fn create_new_function(&mut self, id: FunctionId, is_extern: bool) {
        if self.funs_by_id.get(&id).is_some() {
            panic!("Module already has a function with id #{}", id.0);
        }

        let fun = Function::new(id, is_extern);
        let fun_idx = self.funs.len();
        self.funs.push(fun);
        self.funs_by_id.insert(id, fun_idx);
    }

    fn gen_fun_id(&self) -> FunctionId {
        let mut next_id = FunctionId(1000 + self.funs().len());
        loop {
            if self.funs_by_id.get(&next_id).is_none() {
                break;
            }
            next_id.0 += 1;
        }
        next_id
    }
}
